import { Button, IconChevronLeft, IconChevronRight } from "@heroui/react";
import type { ReactNode } from "react";
import type { Stepper } from "@/components/stepper";

export interface ControlButtonProps {
  next?: "active" | "disabled";
  previous?: "active" | "disabled";
  stepper: Stepper;
  completeOnNext?: boolean;
  completeButton?: ReactNode;
}

export function ControlButtons({
  next,
  previous,
  stepper,
  completeOnNext,
  completeButton,
}: ControlButtonProps) {
  return (
    <>
      <div className="grow" />
      <footer className="flex justify-between gap-4 sticky bottom-0 bg-surface">
        {previous && (
          <Button
            size="lg"
            aria-label="Previous Step"
            onClick={() => stepper.previousStep()}
            variant="ghost"
            isDisabled={previous === "disabled"}
          >
            <IconChevronLeft />
            Previous
          </Button>
        )}

        <div className="grow" />
        {next && (
          <Button
            size="lg"
            aria-label="Next Step"
            variant="primary"
            onClick={() => stepper.nextStep(completeOnNext)}
            isDisabled={next === "disabled"}
          >
            Next
            <IconChevronRight />
          </Button>
        )}
        {completeButton}
      </footer>
    </>
  );
}
