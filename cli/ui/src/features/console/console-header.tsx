import { Chip } from "@heroui/react";
import {
  IconAlertHexagonFilled,
  IconAlertTriangleFilled,
  IconCircleCheckFilled,
  IconInfoCircleFilled,
  IconProgress,
  IconRefresh,
} from "@tabler/icons-react";
import { useMemo } from "react";
import type { CommandStatus } from "@/core/web";
import { useInstances } from "@/features/console/provider";

export function ProcessManagerChips() {
  const { instances } = useInstances();

  const error = useMemo(() => {
    return instances.filter(
      (instance) => (instance.status as CommandStatus.Error | undefined)?.Error,
    ).length;
  }, [instances]);

  const success = useMemo(() => {
    return instances.filter(
      (instance) =>
        (instance.status as CommandStatus.Stopped | undefined)?.Stopped
          ?.exit_code === 0,
    ).length;
  }, [instances]);

  const failed = useMemo(() => {
    return instances.filter((instance) => {
      const stopped = (instance.status as CommandStatus.Stopped | undefined)
        ?.Stopped;
      if (!stopped) {
        return false;
      }
      return stopped.exit_code !== 0;
    }).length;
  }, [instances]);

  const running = useMemo(() => {
    return instances.filter(
      (instance) =>
        (instance.status as CommandStatus.Running | undefined)?.Running,
    ).length;
  }, [instances]);

  const pending = useMemo(() => {
    return instances.filter((instance) => instance.status === "Pending").length;
  }, [instances]);

  return (
    <div className="flex gap-2">
      {instances.length === 0 && (
        <Chip variant="soft" color="accent" size="md">
          <IconInfoCircleFilled className="size-4" />
          <Chip.Label>no active instances</Chip.Label>
        </Chip>
      )}
      {pending > 0 && (
        <Chip variant="soft" color="accent" size="md">
          <IconProgress className="size-4" />
          <Chip.Label>{pending} pending</Chip.Label>
        </Chip>
      )}
      {running > 0 && (
        <Chip variant="soft" color="success" size="md">
          <IconRefresh className="size-4 animate-spin" />
          <Chip.Label>{running} running</Chip.Label>
        </Chip>
      )}
      {success > 0 && (
        <Chip variant="soft" color="success" size="md">
          <IconCircleCheckFilled className="size-4" />
          <Chip.Label>{success} success</Chip.Label>
        </Chip>
      )}
      {failed > 0 && (
        <Chip variant="soft" color="warning" size="md">
          <IconAlertTriangleFilled className="size-4" />
          <Chip.Label>{failed} failed</Chip.Label>
        </Chip>
      )}
      {error > 0 && (
        <Chip variant="soft" color="danger" size="md">
          <IconAlertHexagonFilled className="size-4" />
          <Chip.Label>{error} error</Chip.Label>
        </Chip>
      )}
    </div>
  );
}
