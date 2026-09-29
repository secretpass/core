use std::env;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

#[derive(Debug, Clone)]
pub struct WorkingDirectory {
    path: String,
    is_cloud: bool,
}

impl WorkingDirectory {
    pub fn new(defined_path: Option<String>, is_cloud: bool) -> Self {
        let bin_wd = env::current_dir().unwrap();
        let path = if let Some(user_wd) = defined_path {
            let path = Path::new(user_wd.as_str());
            if !path.is_dir() {
                panic!("Working directory {user_wd} not found");
            }

            user_wd
        } else {
            format!("{}", bin_wd.display())
        };
        Self { path, is_cloud }
    }

    pub fn path(&self) -> &Path {
        Path::new(&self.path)
    }

    pub fn short_path(&self) -> String {
        let path = self.path();

        if let Some(home_dir) = home::home_dir()
            && let Ok(relative_path) = path.strip_prefix(&home_dir)
        {
            let short_path = Path::new("~").join(relative_path);
            return format!("{}", short_path.display());
        }

        self.path.clone()
    }

    pub fn project_dir(&self) -> PathBuf {
        self.path().join(".spass")
    }

    pub fn config_path(&self) -> PathBuf {
        self.project_dir().join("config.yaml")
    }

    pub fn public_keys_path(&self) -> PathBuf {
        self.project_dir().join("public-keys.json")
    }

    pub fn secrets_path(&self, user_id: String, key_id: String) -> PathBuf {
        self.project_dir()
            .join(format!("secrets/{}/{}.lock", user_id, key_id))
    }
}

static WORKING_DIRECTORY: OnceLock<WorkingDirectory> = OnceLock::new();

pub fn get_working_directory() -> WorkingDirectory {
    WORKING_DIRECTORY.get().unwrap().clone()
}

pub fn setup_working_directory(defined_path: Option<String>, is_cloud: bool) {
    let wd = WorkingDirectory::new(defined_path, is_cloud);
    WORKING_DIRECTORY.set(wd).unwrap();
}
