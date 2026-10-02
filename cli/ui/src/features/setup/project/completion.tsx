import { Alert, Button, Description } from "@heroui/react";
import { IconFingerprint, IconFolder } from "@tabler/icons-react";
import { useCallback, useState } from "react";
import { LoadingCircle } from "@/components/loader";
import { CWD_HEADER } from "@/constants";
import {
  type RegistrationParams,
  register_user,
  type SecretpassProject,
  type SecretpassUser,
  type StoredPublicKey,
} from "@/core/web";
import type { RegistrationState } from "@/features/setup/project/types.ts";
import type { SecretMangerState } from "@/manager";
import type { SecretManagerConfig } from "@/types";
import { type ComplexState, makeRequest } from "@/utils.ts";

interface CompletionScreenProps {
  state: ComplexState<RegistrationState>;
  directory: string;
  onAuthComplete: (state: SecretMangerState) => void;
}

export function CompletionScreen({
  state,
  onAuthComplete,
  directory,
}: CompletionScreenProps) {
  const [is_loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const completeRegistration = useCallback(() => {
    setError(null);

    const project: SecretpassProject = {
      id: state.id,
      name: state.project_name,
      description: state.description,
      algorithm: state.algorithm,
      residency: state.residency,
      created_at: state.created,
    };

    const user: SecretpassUser = {
      id: state.user_id,
      username: state.user_email_address,
      name: state.user_display_name,
      level: "Admin",
      created_at: state.created,
    };
    const params: RegistrationParams = {
      name: state.key_name,
      project,
      user,
    };

    register_user(JSON.stringify(params))
      .then(async (public_key_json) => {
        setLoading(true);
        const public_key: StoredPublicKey = JSON.parse(public_key_json);

        const response = await makeRequest<SecretManagerConfig>(
          "/api/project",
          {
            method: "POST",
            body: JSON.stringify({
              project,
              user,
              environments: state.environments,
              public_key,
            }),
            headers: {
              "Content-Type": "application/json",
            },
          },
        );

        if (response.ok) {
          const config = response.json();
          const directory = response.headers.get(CWD_HEADER) as string;
          onAuthComplete({
            config,
            directory,
            public_key,
            public_keys: [public_key],
            user,
          });
        } else {
          setError(`Failed to create project: ${response.text}`);
        }
      })
      .catch((err) => {
        setError(String(err));
      })
      .finally(() => setLoading(false));
  }, [state, onAuthComplete]);

  return (
    <div className="flex flex-col justify-center items-center gap-10 grow">
      <div className="text-center">
        <h1 className="text-2xl text-muted">
          Create{" "}
          <span className="font-semibold text-foreground">
            {state.project_name}
          </span>{" "}
        </h1>
        <Description>{state.description}</Description>
      </div>

      <kbd className="font-mono text-white bg-black px-6 py-4 flex items-center gap-4">
        <IconFolder />
        {directory}/.spass
      </kbd>

      {error && (
        <Alert status="danger">
          <Alert.Indicator />
          <Alert.Content>
            <Alert.Title>Error registering passkey</Alert.Title>
            <Alert.Description>{error}</Alert.Description>
          </Alert.Content>
        </Alert>
      )}

      <Button
        onClick={completeRegistration}
        isDisabled={is_loading}
        aria-label="Register Passkey"
        size="lg"
      >
        {!is_loading && <IconFingerprint />}
        {is_loading && (
          <LoadingCircle aria-label="Loading" size="sm" isIndeterminate />
        )}
        Authorize and Create Project
      </Button>
    </div>
  );
}
