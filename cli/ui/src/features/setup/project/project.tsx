import { Form } from "@heroui/react";
import { Activity } from "react";
import { StepperView } from "@/components/stepper";
import { CompletionScreen } from "@/features/setup/project/completion.tsx";
import type { RegistrationState } from "@/features/setup/project/types.ts";
import type { SecretMangerState } from "@/manager";
import { useComplexState } from "@/utils.ts";
import { EncryptionAlgorithmSelector } from "./algorithms";
import { EnvironmentDefinitions } from "./environment";
import { ProjectOverview } from "./overview";
import { AdminProfile } from "./profile";
import { PasskeyResidencySelector } from "./residency";
import { useProjectStepper } from "./steps";

export function CreateProject({
  onAuthComplete,
  directory,
}: {
  onAuthComplete: (state: SecretMangerState) => void;
  directory: string;
}) {
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
    key_name: "Passkey 0",
  });

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
          <AdminProfile stepper={stepper} state={state} />
        </Activity>

        {stepper.completed && (
          <CompletionScreen
            onAuthComplete={onAuthComplete}
            directory={directory}
            state={state}
          />
        )}
      </Form>
    </div>
  );
}
