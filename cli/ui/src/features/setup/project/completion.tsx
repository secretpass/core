import { Button, Description } from "@heroui/react";
import { IconFingerprint, IconFolder } from "@tabler/icons-react";
import { useCallback } from "react";
import { ErrorDisplay, useError } from "@/components";
import { LoadingCircle } from "@/components/loader";
import {
  type RegistrationParams,
  register_user,
  type SecretpassProject,
  type SecretpassUser,
  type StoredPublicKey,
} from "@/core/web";
import { useAnalytics } from "@/features/analytics/provider";
import type { RegistrationState } from "@/features/setup/project/types.ts";
import type { SecretManagerConfig } from "@/types";
import { type ComplexState, resolveRequest } from "@/utils.ts";

interface CompletionScreenProps {
  state: ComplexState<RegistrationState>;
  directory: string;
  onAuthComplete: (public_key_id: string) => void;
}

export function CompletionScreen({
  state,
  onAuthComplete,
  directory,
}: CompletionScreenProps) {
  const error = useError();
  const analytics = useAnalytics();

  const completeRegistration = useCallback(async () => {
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
    const params = JSON.stringify({
      project,
      user,
    } as RegistrationParams);

    const reg_result = await error.catch(
      "Error registering your passkey",
      () => register_user(state.key_name, params),
      "new_project_register_passkey",
    );
    if (!reg_result.ok) {
      return;
    }

    const public_key: StoredPublicKey = JSON.parse(reg_result.result);

    const create_result = await error.catch(
      "Error creating project",
      () =>
        resolveRequest<SecretManagerConfig>("/api/project", {
          method: "POST",
          body: JSON.stringify({
            project,
            user,
            environments: state.environments,
            public_key,
          }),
        }),
      "create_new_project",
    );

    if (!create_result.ok) {
      return;
    }

    onAuthComplete(public_key.id);

    analytics?.group("local-project", project.id, {
      name: project.name,
      algorithm: project.algorithm,
      residency: project.residency,
    });
    analytics?.identify(user.id, {
      email: user.username,
      name: user.name,
      level: user.level,
    });
    analytics?.capture("create_project");

    if (analytics?.conversations?.isAvailable()) {
      analytics?.conversations?.sendMessage(
        `Congratulations ${user}!\n\nYou're on your way to securing your secrets and owning the storage.\nPlease feel free to ping us if you get stuck, we'll be happy to assist you.\n\nOtherwise, keep your secrets!\n`,
        {
          email: user.username,
          name: user.name,
        },
      );
    }
  }, [state, onAuthComplete, analytics, error.catch]);

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

      <ErrorDisplay value={error.value} />

      <Button
        onClick={completeRegistration}
        isDisabled={error.is_pending}
        aria-label="Register Passkey"
        size="lg"
      >
        {!error.is_pending && <IconFingerprint />}
        {error.is_pending && (
          <LoadingCircle aria-label="Loading" size="sm" isIndeterminate />
        )}
        Authorize and Create Project
      </Button>
    </div>
  );
}
