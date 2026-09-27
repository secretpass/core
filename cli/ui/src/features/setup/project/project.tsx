import { Form } from "@heroui/react";
import { Activity, useCallback, useState } from "react";
import { StepperView } from "@/components/stepper";
import {
  type EncryptionAlgorithm,
  type Passkey,
  type PasskeyResidency,
  register_user,
  SecretpassProject,
} from "@/core/web";
import { useSecretManager } from "@/manager";
import { useComplexState } from "@/utils.ts";
import { EncryptionAlgorithmSelector } from "./algorithms";
import { EnvironmentDefinitions, type SetupEnvironments } from "./environment";
import { ProjectOverview } from "./overview";
import { AdminProfile, type ProfileSchema } from "./profile";
import { PasskeyResidencySelector } from "./residency";
import { useProjectStepper } from "./steps";

export function CreateProject() {
  const [error, setError] = useState<string | null>(null);
  const [passkey, setPasskey] = useState<Passkey | null>(null);
  const stepper = useProjectStepper();
  const manager = useSecretManager();
  const overview = useComplexState({
    id: manager.config?.project.id || "",
    name: manager.config?.project.name || "",
    description: manager.config?.project.description || "",
  });

  const algorithm = useComplexState<{
    algorithm: EncryptionAlgorithm;
  }>({
    algorithm: manager.config?.project.algorithm || "ECC",
  });

  const residency = useComplexState<{
    residency: PasskeyResidency;
  }>({
    residency: manager.config?.project.residency || "SyncedAllowed",
  });

  const environments = useComplexState<SetupEnvironments>({
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
  });

  const profile = useComplexState<ProfileSchema>({
    user_id: crypto.randomUUID(),
    email_address: "",
    name: "",
  });

  const onComplete = useCallback(() => {
    setError(null);

    const project = new SecretpassProject(
      overview.id,
      overview.name,
      overview.description,
      algorithm.algorithm,
      residency.residency,
      new Date().toISOString(),
    );

    register_user(project, profile.user_id, profile.email_address, profile.name)
      .then((passkey) => {
        stepper.nextStep(true);
        setPasskey(passkey);
      })
      .catch((err) => {
        setError(String(err));
      });
  }, [stepper, overview, algorithm, residency, profile]);

  return (
    <div className="relative flex w-full min-w-xl gap-6">
      <StepperView stepper={stepper} />

      <Form className="grow flex flex-col w-2/3 px-4 py-6 rounded-2xl gap-6 min-h-80 max-h-[70vh] overflow-y-auto">
        <Activity mode={stepper.active === "overview" ? "visible" : "hidden"}>
          <ProjectOverview state={overview} stepper={stepper} />
        </Activity>

        <Activity mode={stepper.active === "encryption" ? "visible" : "hidden"}>
          <EncryptionAlgorithmSelector state={algorithm} stepper={stepper} />
        </Activity>

        <Activity mode={stepper.active === "residency" ? "visible" : "hidden"}>
          <PasskeyResidencySelector state={residency} stepper={stepper} />
        </Activity>

        <Activity
          mode={stepper.active === "environments" ? "visible" : "hidden"}
        >
          <EnvironmentDefinitions stepper={stepper} state={environments} />
        </Activity>

        <Activity mode={stepper.active === "profile" ? "visible" : "hidden"}>
          <AdminProfile
            error={error}
            stepper={stepper}
            state={profile}
            onComplete={onComplete}
          />
        </Activity>
      </Form>
    </div>
  );
}
