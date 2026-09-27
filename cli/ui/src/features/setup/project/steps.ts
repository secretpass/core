import { StepperStepStatus, useStepper } from "@/components/stepper";

export function useProjectStepper() {
  return useStepper("overview", [
    {
      id: "overview",
      label: "Project Overview",
      status: StepperStepStatus.Pending,
      next: "encryption",
    },
    {
      id: "encryption",
      label: "Encryption Algorithm",
      status: StepperStepStatus.Pending,
      previous: "overview",
      next: "residency",
    },
    {
      id: "residency",
      label: "Passkey Residency",
      status: StepperStepStatus.Pending,
      previous: "encryption",
      next: "environments",
    },
    {
      id: "environments",
      label: "Environments",
      status: StepperStepStatus.Pending,
      previous: "residency",
      next: "profile",
    },
    {
      id: "profile",
      label: "Admin Profile",
      status: StepperStepStatus.Pending,
      previous: "environments",
      next: "complete",
    },
  ]);
}
