import {
  IconCircleCheck,
  IconCircleDashed,
  IconCircleX,
  IconProgress,
} from "@tabler/icons-react";
import clsx from "clsx";
import { type Stepper, type StepperStep, StepperStepStatus } from "./state";

export function StepperView({ stepper }: { stepper: Stepper }) {
  return (
    <div className="flex flex-col px-4 py-6 bg-background rounded-2xl w-1/3! gap-4">
      {stepper.steps.map((item) => (
        <StepperItemView
          key={item.id}
          is_active={item.id === stepper.active}
          item={item}
        />
      ))}
    </div>
  );
}

function StepperItemView({
  item,
  is_active,
}: {
  item: StepperStep;
  is_active: boolean;
}) {
  const is_completed = item.status === StepperStepStatus.Completed;
  const is_pending = item.status === StepperStepStatus.Pending && !is_active;

  return (
    <div
      className={clsx(
        "flex items-center gap-2 px-4 py-2 rounded-xl select-none bg-surface",
        is_completed && "font-light",
        is_active && "font-semibold",
        is_pending && "text-muted",
      )}
    >
      {is_active && !is_completed && (
        <IconProgress className="size-5 text-blue-500" />
      )}
      {is_pending && <IconCircleDashed className="size-5 text-muted" />}
      {is_completed && (
        <IconCircleCheck
          className={clsx(
            "size-5",
            is_active ? "text-blue-500" : "text-success",
          )}
        />
      )}
      {item.status === StepperStepStatus.Failed && (
        <IconCircleX className="size-5 text-danger" />
      )}
      {item.label}
    </div>
  );
}
