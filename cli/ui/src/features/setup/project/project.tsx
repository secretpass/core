import { Form } from "@heroui/react";
import { Activity, useCallback, useState } from "react";
import { StepperView } from "@/components/stepper";
import {
  type Passkey,
  register_user,
  type RegistrationParams,
  type SecretpassProject,
} from "@/core/web";
import { CompletionScreen } from "@/features/setup/project/completion.tsx";
import type { RegistrationState } from "@/features/setup/project/types.ts";
import { useComplexState, useUncaughtWasmError } from "@/utils.ts";
import { EncryptionAlgorithmSelector } from "./algorithms";
import { EnvironmentDefinitions } from "./environment";
import { ProjectOverview } from "./overview";
import { AdminProfile } from "./profile";
import { PasskeyResidencySelector } from "./residency";
import { useProjectStepper } from "./steps";

export function CreateProject() {
  const [error, setError] = useState<string | null>(null);
  const wasm_error = useUncaughtWasmError();
  const [passkey, setPasskey] = useState<{
    passkey: Passkey;
    project: SecretpassProject;
  } | null>(null);
  const stepper = useProjectStepper();

  const state = useComplexState<RegistrationState>({
    id: crypto.randomUUID(),
    project_name: "",
    description: "",
    algorithm: "ECC",
    residency: "SyncedAllowed",
    created: new Date().toISOString(),
    environments: [
      {
        name: "development",
        description: "For testing by developers before moving to QC",
      },
      {
        name: "staging",
        description: "For testing by QAs before promoting to production",
      },
      {
        name: "production",
        description: "Prod secrets, heavily restricted",
      },
    ],
    user_id: crypto.randomUUID(),
    user_display_name: "",
    user_email_address: "",
  });

  const onComplete = useCallback(() => {
    setError(null);
    wasm_error.clear();

    const project: SecretpassProject = {
      id: state.id,
      name: state.project_name,
      description: state.description,
      algorithm: state.algorithm,
      residency: state.residency,
      created_at: state.created,
    };
    const params: RegistrationParams = {
      project,
      user_id: state.user_id,
      email_address: state.user_email_address,
      display_name: state.user_display_name,
    };

    register_user(JSON.stringify(params))
      .then((passkey_json) => {
        const passkey: Passkey = JSON.parse(passkey_json);
        stepper.nextStep(true);
        setPasskey({ passkey, project });
      })
      .catch((err) => {
        setError(String(err));
      });
  }, [stepper, state, wasm_error]);

  return (
    <div className="relative flex w-full min-w-xl gap-6">
      <StepperView stepper={stepper} />

      <Form className="grow flex flex-col w-2/3 px-4 py-6 rounded-2xl gap-6 min-h-80 max-h-[70vh] overflow-y-auto">
        <Activity
          mode={
            stepper.active === "overview" && !stepper.completed
              ? "visible"
              : "hidden"
          }
        >
          <ProjectOverview state={state} stepper={stepper} />
        </Activity>

        <Activity
          mode={
            stepper.active === "encryption" && !stepper.completed
              ? "visible"
              : "hidden"
          }
        >
          <EncryptionAlgorithmSelector state={state} stepper={stepper} />
        </Activity>

        <Activity
          mode={
            stepper.active === "residency" && !stepper.completed
              ? "visible"
              : "hidden"
          }
        >
          <PasskeyResidencySelector state={state} stepper={stepper} />
        </Activity>

        <Activity
          mode={
            stepper.active === "environments" && !stepper.completed
              ? "visible"
              : "hidden"
          }
        >
          <EnvironmentDefinitions stepper={stepper} state={state} />
        </Activity>

        <Activity
          mode={
            stepper.active === "profile" && !stepper.completed
              ? "visible"
              : "hidden"
          }
        >
          <AdminProfile
            error={error || wasm_error.message}
            stepper={stepper}
            state={state}
            onComplete={onComplete}
          />
        </Activity>

        {passkey && (
          <CompletionScreen
            state={state}
            project={passkey.project as SecretpassProject}
            passkey={passkey.passkey as Passkey}
          />
        )}
      </Form>
    </div>
  );
}
