import { ProgressCircle, type ProgressCircleProps } from "@heroui/react";

export function LoadingCircle(props: ProgressCircleProps) {
  return (
    <ProgressCircle {...props}>
      <ProgressCircle.Track>
        <ProgressCircle.TrackCircle />
        <ProgressCircle.FillCircle />
      </ProgressCircle.Track>
    </ProgressCircle>
  );
}
