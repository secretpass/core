import {
  IconAlertHexagonFilled,
  IconAlertTriangleFilled,
  IconCircleCheckFilled,
  IconHelpTriangleFilled,
  IconProgress,
  IconRefresh,
} from "@tabler/icons-react";
import clsx from "clsx";
import { useMemo } from "react";
import type { CommandStatus } from "@/core/web";

export function InstanceStatusIcon({
  status,
  className,
}: {
  status: CommandStatus;
  className?: string;
}) {
  if (status === "Pending") {
    return <IconProgress className={clsx("text-muted", className)} />;
  }

  if ((status as CommandStatus.Running).Running) {
    return (
      <IconRefresh className={clsx("animate-spin text-blue-500", className)} />
    );
  }

  if ((status as CommandStatus.Error).Error) {
    return (
      <IconAlertHexagonFilled className={clsx("text-danger", className)} />
    );
  }

  const stopped = (status as CommandStatus.Stopped).Stopped;
  if (stopped && stopped.exit_code === 0) {
    return (
      <IconCircleCheckFilled className={clsx("text-success", className)} />
    );
  }

  if (stopped && stopped.exit_code !== 0) {
    return (
      <IconAlertTriangleFilled className={clsx("text-warning", className)} />
    );
  }

  return <IconHelpTriangleFilled className={clsx("text-muted", className)} />;
}

export function useStatusLabel(status: CommandStatus) {
  const label = useMemo(() => {
    if (status === "Pending") {
      return "pending";
    }

    const running = (status as CommandStatus.Running).Running;
    if (running) {
      return "running";
    }

    const error = (status as CommandStatus.Error).Error;
    if (error) {
      return "error";
    }

    const stopped = (status as CommandStatus.Stopped).Stopped;
    if (stopped && stopped.exit_code === 0) {
      return "success";
    }

    if (stopped && stopped.exit_code !== 0) {
      return "failed";
    }

    return "unknown";
  }, [status]);

  return label;
}

export function useProcessId(status: CommandStatus) {
  const pid = useMemo(() => {
    const running = (status as CommandStatus.Running).Running;
    if (running) {
      return running.process_id;
    }
    return null;
  }, [status]);

  return pid;
}

export function useProcessEnvironment(status: CommandStatus) {
  const env = useMemo(() => {
    if (status === "Pending") {
      return null;
    }

    const running = (status as CommandStatus.Running).Running;
    if (running) {
      return running.environment;
    }

    const error = (status as CommandStatus.Error).Error;
    if (error) {
      return error.environment;
    }

    const stopped = (status as CommandStatus.Stopped).Stopped;
    if (stopped) {
      return stopped.environment;
    }

    return null;
  }, [status]);

  return env;
}
