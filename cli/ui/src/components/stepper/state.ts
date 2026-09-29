import { useState } from "react";
import { create } from "zustand";
import { immer } from "zustand/middleware/immer";

export enum StepperStepStatus {
  Pending,
  Completed,
  Failed,
}

export interface StepperStep {
  id: string;
  status: StepperStepStatus;
  label: string;
  next?: string;
  previous?: string;
}

export interface Stepper {
  steps: StepperStep[];
  active: string;
  completed?: boolean;
  current(): StepperStep;
  nextStep(complete?: boolean): void;
  previousStep(): void;
}

export function useStepper(first_step: string, steps: StepperStep[]): Stepper {
  const [useZustandState] = useState(() =>
    create(
      immer<Stepper>((set, getState) => ({
        steps,
        active: first_step,
        completed: false,
        current: () => {
          const state = getState();
          return state.steps.find(
            (step: StepperStep) => step.id === state.active,
          ) as StepperStep;
        },
        nextStep: (complete?: boolean) =>
          set((state) => {
            const step = state.steps.find(
              (step: StepperStep) => step.id === state.active,
            ) as StepperStep;
            if (step && complete) {
              step.status = StepperStepStatus.Completed;
            }
            if (step?.next) {
              state.active = step.next;
            } else if (complete) {
              state.active = "";
              state.completed = true;
            }
          }),
        previousStep: () =>
          set((state) => {
            const step = state.steps.find(
              (step: StepperStep) => step.id === state.active,
            );
            if (step?.previous) {
              state.active = step.previous;
            }
          }),
      })),
    ),
  );

  return useZustandState();
}
