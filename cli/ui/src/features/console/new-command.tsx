import {
  Button,
  Description,
  InputGroup,
  Label,
  TextField,
  toast,
} from "@heroui/react";
import {
  IconFolder,
  IconPlayerPlayFilled,
  IconTerminal,
} from "@tabler/icons-react";
import { useCallback, useMemo } from "react";
import { ErrorDisplay, useError } from "@/components";
import { PendingAuthIcon } from "@/components/loader";
import { EnvironmentSelector } from "@/components/select";
import {
  type CommandInstanceDetails,
  type DecryptEnvPackageParams,
  decrypt_env_package,
} from "@/core/web";
import { useInstances } from "@/features/console/provider";
import { resolveRequest, useComplexState } from "@/utils";
import { useSecretManager } from "../../manager/provider";

interface NewCommandState {
  command: string;
  working_dir: string;
  environment: string;
}

export function NewCommand() {
  const { directory, getEnvSecretsPackage, login_params } = useSecretManager();
  const { refreshInstances, setActive } = useInstances();

  const error = useError();
  const state = useComplexState<NewCommandState>(() => ({
    command: "",
    working_dir: "",
    environment: "",
  }));

  const secrets_package = useMemo(
    () => getEnvSecretsPackage(state.environment),
    [getEnvSecretsPackage, state.environment],
  );

  const is_valid = useMemo(
    () => !!secrets_package && !!state.command,
    [state.command, secrets_package],
  );

  const runCommand = useCallback(async () => {
    if (!secrets_package) {
      return;
    }

    const decryption_params: DecryptEnvPackageParams = {
      login_params,
      secrets_package,
    };
    const decryption_result = await error.catch(
      "Error decrypting secrets",
      () => decrypt_env_package(JSON.stringify(decryption_params)),
    );
    if (!decryption_result.ok) {
      return;
    }

    const params = {
      command: state.command,
      environment: state.environment,
      variables: JSON.parse(decryption_result.result),
      working_dir: state.working_dir ? state.working_dir : null,
    };

    const run_result = await error.catch("Error running command", () =>
      resolveRequest<CommandInstanceDetails>("/api/run/instance", {
        method: "POST",
        body: JSON.stringify(params),
      }),
    );
    if (!run_result.ok) {
      return;
    }

    toast(`Started -> \`${state.command}\``, { variant: "success" });
    refreshInstances().then(() => {
      state.reset();
      setActive(run_result.result.id);
    });
  }, [
    secrets_package,
    login_params,
    error.catch,
    state,
    refreshInstances,
    setActive,
  ]);

  return (
    <div className="px-4 h-full w-2/3 relative overflow-y-auto z-30">
      <div className="flex flex-col bg-surface top-0 sticky">
        <div className="flex justify-between items-start mt-6">
          <div className="text-2xl font-bold">Run new command</div>
        </div>
        <Description className="mt-1">
          Start a new command with secrets injected
        </Description>
        <hr className="my-4" />
      </div>

      <EnvironmentSelector<string>
        disabled={error.is_pending}
        variant="secondary"
        value={state.environment}
        onChange={(environment) => state.update({ environment })}
        label="Environment"
        description="Select the environment to inject secrets from for this command"
      />

      <TextField className="mt-6" variant="secondary">
        <Label>Working Directory</Label>
        <InputGroup className="font-mono">
          <InputGroup.Prefix>
            <IconFolder /> {directory}/
          </InputGroup.Prefix>
          <InputGroup.Input
            className="field-sizing-content resize-none"
            value={state.working_dir}
            onChange={(evt) =>
              state.update({ working_dir: evt.currentTarget.value })
            }
          />
        </InputGroup>
        <Description>
          The working directory for this command, defaults to the project
          directory, can be relative to the project directory.
        </Description>
      </TextField>

      <TextField className="mt-6" variant="secondary">
        <Label>Command</Label>
        <InputGroup className="font-mono dark bg-black">
          <InputGroup.Prefix>
            <IconTerminal />
          </InputGroup.Prefix>
          <InputGroup.TextArea
            className="field-sizing-content resize-none whitespace-pre-wrap"
            placeholder="e.g.: npm run start"
            value={state.command}
            onChange={(evt) =>
              state.update({ command: evt.currentTarget.value })
            }
          />
        </InputGroup>
        <Description>
          Enter any valid shell command, secrets will be injected as environment
          variables from the environment you select
        </Description>
      </TextField>

      <ErrorDisplay value={error.value} />

      <div className="mt-8 flex justify-end w-full">
        <Button isDisabled={error.is_pending || !is_valid} onClick={runCommand}>
          <PendingAuthIcon is_pending={error.is_pending} />
          Run Command
          <IconPlayerPlayFilled />
        </Button>
      </div>
    </div>
  );
}
