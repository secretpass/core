import {
  ProgressBar,
  type ProgressBarProps,
  ProgressCircle,
  type ProgressCircleProps,
} from "@heroui/react";
import { IconFingerprint } from "@tabler/icons-react";

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

export function PendingAuthIcon({
  is_pending,
  className,
}: {
  className?: string;
  is_pending: boolean;
}) {
  return (
    <>
      {is_pending && <LoadingCircle className={className} />}
      {!is_pending && <IconFingerprint className={className} />}
    </>
  );
}

export function LoadingBar(props: ProgressBarProps) {
  return (
    <ProgressBar {...props}>
      <ProgressBar.Output />
      <ProgressBar.Track>
        <ProgressBar.Fill />
      </ProgressBar.Track>
    </ProgressBar>
  );
}
