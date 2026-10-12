use crate::command::get_process_group;
use crate::command::logger::Logger;
use crate::project::{get_working_directory, short_path};
use secretpass_core::{CommandInstanceDetails, CommandStatus, LogItem, LogSource};
use std::collections::HashMap;
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::Command as StdCommand;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Child, Command};

pub struct CommandInstance {
    pub command: String,
    pub workdir: PathBuf,
    display_path: String,
    pub logger: Arc<Mutex<Logger>>,
    pub status: CommandStatus,
    pub process: Option<Child>,
    pub stderr_monitor: Option<tokio::task::JoinHandle<()>>,
    pub stdout_monitor: Option<tokio::task::JoinHandle<()>>,
}

impl CommandInstance {
    pub fn new(working_dir: PathBuf, command: String) -> Self {
        let short_path = short_path(&format!("{}", working_dir.display()));
        let short_path = if let Some(stripped) =
            short_path.strip_prefix(&get_working_directory().short_path())
        {
            format!(".{}", stripped)
        } else {
            short_path
        };

        Self {
            command,
            workdir: working_dir,
            logger: Default::default(),
            status: CommandStatus::Pending,
            process: None,
            stdout_monitor: None,
            stderr_monitor: None,
            display_path: short_path,
        }
    }

    pub fn details(&self, id: String) -> CommandInstanceDetails {
        CommandInstanceDetails {
            id,
            status: self.status.clone(),
            command: self.command.clone(),
            workdir: self.display_path.clone(),
        }
    }

    pub fn get_logs(&mut self, cursor: usize) -> anyhow::Result<Vec<LogItem>> {
        let logger = self
            .logger
            .lock()
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        let items = logger.get_logs(cursor);

        Ok(items)
    }

    pub async fn run(
        &mut self,
        environment: String,
        variables: HashMap<String, String>,
    ) -> anyhow::Result<()> {
        match &self.status {
            CommandStatus::Running { .. } => self.stop(false).await,
            _ => Ok(()),
        }?;

        let (mut command, arg_prefix) = if cfg!(target_os = "windows") {
            (Command::new("cmd"), "/C")
        } else {
            (Command::new("sh"), "-c")
        };

        command
            .current_dir(self.workdir.clone())
            .arg(arg_prefix)
            .arg(&self.command)
            .envs(variables)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped());

        let mut child = {
            let group = get_process_group().await;
            group.spawn(command)?
        };
        let process_id = child
            .id()
            .ok_or_else(|| anyhow::anyhow!("Failed to get process ID"))?;
        self.status = CommandStatus::Running {
            process_id,
            environment,
        };

        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| anyhow::anyhow!("Failed to attach stdout"))?;
        let stderr = child
            .stderr
            .take()
            .ok_or_else(|| anyhow::anyhow!("Failed to attach stderr"))?;

        let stdout_logger = self.logger.clone();
        let stderr_logger = self.logger.clone();

        let stdout_task = tokio::spawn(async move {
            let mut reader = BufReader::new(stdout).lines();

            while let Ok(Some(line)) = reader.next_line().await {
                let mut logger = stdout_logger.lock().unwrap();
                println!("{}", line);
                logger.push(LogSource::StdOut, line);
            }
        });

        let stderr_task = tokio::spawn(async move {
            let mut reader = BufReader::new(stderr).lines();

            while let Ok(Some(line)) = reader.next_line().await {
                let mut logger = stderr_logger.lock().unwrap();
                eprintln!("{}", line);
                logger.push(LogSource::StdError, line);
            }
        });

        self.process = Some(child);
        self.stdout_monitor = Some(stdout_task);
        self.stderr_monitor = Some(stderr_task);

        Ok(())
    }

    pub fn run_machine(&mut self, variables: HashMap<String, String>) {
        let (mut command, arg_prefix) = if cfg!(target_os = "windows") {
            (StdCommand::new("cmd"), "/C")
        } else {
            (StdCommand::new("sh"), "-c")
        };

        command
            .current_dir(self.workdir.clone())
            .arg(arg_prefix)
            .arg(&self.command)
            .envs(variables);

        #[cfg(unix)]
        {
            let error = command.exec();
            panic!("Failed to start the command: {:?}", error);
        }

        #[cfg(not(unix))]
        {
            let mut child = match command.spawn() {
                Ok(child) => child,
                Err(err) => {
                    panic!("Failed to start the command: {:?}", err)
                }
            };
            let exit_status = match child.wait() {
                Ok(status) => status,
                Err(err) => {
                    panic!("Failed to wait for the command: {:?}", err)
                }
            };

            exit(exit_status.code().unwrap_or(0));
        }
    }

    pub fn update_status(&mut self) -> anyhow::Result<()> {
        // We only want to evaluate the status if the instance is still running
        let environment = match &self.status {
            CommandStatus::Running { environment, .. } => environment,
            _ => return Ok(()),
        };

        let process = self
            .process
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("Process not found"))?;

        self.status = match process.try_wait() {
            Ok(None) => {
                return Ok(());
            }
            Ok(Some(status)) => CommandStatus::Stopped {
                environment: environment.clone(),
                exit_code: status.code(),
            },
            Err(e) => CommandStatus::Error {
                environment: environment.clone(),
                message: e.to_string(),
            },
        };

        Ok(())
    }

    pub async fn stop_process(&mut self, force: bool) -> anyhow::Result<()> {
        match self.status {
            CommandStatus::Running { .. } => {}
            _ => return Ok(()),
        }

        let process = match &mut self.process {
            Some(process) => process,
            None => return Err(anyhow::anyhow!("Process not started")),
        };

        if force {
            process.kill().await?;
            return Ok(());
        }

        let pid = process
            .id()
            .ok_or_else(|| anyhow::anyhow!("Process ID not available"))?;

        #[cfg(unix)]
        {
            use nix::sys::signal::{Signal, kill};
            use nix::unistd::Pid;

            let nix_pid = Pid::from_raw(pid as i32);
            kill(nix_pid, Signal::SIGTERM)?;
        }

        #[cfg(not(unix))]
        {
            process.kill().await?;
        }
        Ok(())
    }

    pub async fn stop(&mut self, force: bool) -> anyhow::Result<()> {
        self.stop_process(force).await?;
        self.update_status()?;

        if let CommandStatus::Running { .. } = &self.status {
            return Err(anyhow::anyhow!("Command is still running after kill"));
        };

        for monitor in [self.stderr_monitor.as_mut(), self.stdout_monitor.as_mut()]
            .iter()
            .flatten()
        {
            monitor.abort()
        }

        Ok(())
    }
}
