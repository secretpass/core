import { Alert, Button, ProgressCircle } from "@heroui/react";
import {
  IconArrowRight,
  IconFingerprint,
  IconFingerprintScan,
  IconFolder,
  IconShieldCog,
} from "@tabler/icons-react";
import { useCallback, useState } from "react";
import {
  authorize_and_build_public_key,
  type LoginParams,
  type Passkey,
  type SecretpassProject,
} from "@/core/web";
import type { RegistrationState } from "@/features/setup/project/types.ts";
import { useSecretManager } from "@/manager";
import { type ComplexState, useUncaughtWasmError } from "@/utils.ts";

interface CompletionScreenProps {
  state: ComplexState<RegistrationState>;
  passkey: Passkey;
  project: SecretpassProject;
}

export function CompletionScreen({
  state,
  passkey,
  project,
}: CompletionScreenProps) {
  const manager = useSecretManager();
  const [is_loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const wasm_error = useUncaughtWasmError();

  const completeRegistration = useCallback(() => {
    wasm_error.clear();
    setError(null);
    const params: LoginParams = {
      passkey,
      project,
    };
    authorize_and_build_public_key(
      JSON.stringify(params),
      `${state.user_display_name} - Passkey 0`,
    )
      .then((public_key_json) => {
        const public_key = JSON.parse(public_key_json);
        const data = {
          project,
          environments: state.environments,
          user: {
            id: state.user_id,
            username: state.user_email_address,
            name: state.user_display_name,
            level: "Admin",
            added_on: state.created,
          },
          public_key,
        };

        setLoading(true);

        fetch("/api/config", {
          method: "POST",
          body: JSON.stringify(data),
          headers: {
            "Content-Type": "application/json",
          },
        })
          .then((response) => {
            if (response.ok) {
              return response.json();
            }
            setError("Failed to create project");
          })
          .then((config) => {
            if (config) {
              manager.update({ config });
            }
          })
          .finally(() => setLoading(false));
      })
      .catch((error) => {
        setError(String(error));
      });
  }, [state, passkey, project, manager.update, wasm_error.clear]);

  return (
    <div className="flex flex-col justify-center items-center gap-10 grow">
      <h1 className="text-2xl text-muted">
        Create{" "}
        <span className="font-semibold text-foreground">
          {state.project_name} Project Name
        </span>{" "}
      </h1>
      <div className="flex items-center gap-16">
        <div className="flex flex-col gap-4 items-center">
          <IconFingerprint className="size-10 text-success" />
          <div className="text-muted">Admin Registration</div>
        </div>
        <IconArrowRight className="w-10 text-muted" />
        <div className="flex flex-col gap-4 items-center">
          <IconShieldCog className="size-10 text-warning" />
          <div className="text-muted">Save Project</div>
        </div>
      </div>
      <kbd className="font-mono text-white bg-black px-6 py-4 flex items-center gap-4">
        <IconFolder />
        {manager.directory}/.spass
      </kbd>

      {(error || wasm_error.message) && (
        <Alert status="danger">
          <Alert.Indicator />
          <Alert.Content>
            <Alert.Title>Error registering passkey</Alert.Title>
            <Alert.Description>{error || wasm_error.message}</Alert.Description>
          </Alert.Content>
        </Alert>
      )}

      <Button
        onClick={completeRegistration}
        isDisabled={is_loading}
        aria-label="Register Passkey"
        size="lg"
      >
        {!is_loading && <IconFingerprintScan />}
        {is_loading && (
          <ProgressCircle aria-label="Loading" size="sm" isIndeterminate>
            <ProgressCircle.Track>
              <ProgressCircle.TrackCircle />
              <ProgressCircle.FillCircle />
            </ProgressCircle.Track>
          </ProgressCircle>
        )}
        Authorize and Create Project
      </Button>
    </div>
  );
}
