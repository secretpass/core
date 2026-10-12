import { Alert, Button, Chip } from "@heroui/react";
import { IconCpu, IconFolder, IconTerminal } from "@tabler/icons-react";
import { useRef, useState } from "react";
import type { CommandInstanceDetails } from "@/core/web";
import { InstanceStatusIcon, useProcessId } from "../shared";
import { InstanceLogs } from "./logs";

export function InstanceDashboard({
  instance,
}: {
  instance: CommandInstanceDetails;
  is_active: boolean;
}) {
  const process_id = useProcessId(instance.status);
  const [buffer_size, _setBufferSize] = useState(1024);
  const ref = useRef<HTMLDivElement>(null);

  return (
    <div
      ref={ref}
      className="dark bg-slate-900 rounded-2xl px-4 h-full w-2/3 relative overflow-y-auto z-30"
    >
      <div className="flex flex-col top-0 gap-1.5 sticky pt-4 pb-2 bg-slate-900">
        <div className="flex flex-col gap-1 px-4 py-2 rounded-xl font-mono bg-slate-950 text-white">
          <div className="text-muted text-right flex items-center gap-2">
            <InstanceStatusIcon status={instance.status} className="size-5" />
            <div className="grow" />
            <Chip>
              <IconFolder className="size-4" /> {instance.workdir}
            </Chip>
            {process_id !== null && (
              <Chip>
                <IconCpu className="size-4" /> {process_id}
              </Chip>
            )}
          </div>
          <div className="text-green-600 flex items-center gap-2 ">
            <IconTerminal className="text-muted size-5" /> {instance.command}
          </div>
        </div>

        <div className="flex justify-end gap-4">
          <Button>Stop</Button>
          <Button>Kill</Button>
        </div>
      </div>

      {instance.status === "Pending" && (
        <Alert className="dark:bg-background" status="warning">
          <Alert.Indicator />
          <Alert.Content>
            <Alert.Title className="font-mono">Pending</Alert.Title>
            <Alert.Description>
              Select environment and run to start this command
            </Alert.Description>
          </Alert.Content>
        </Alert>
      )}
      <InstanceLogs ref={ref} buffer_size={buffer_size} instance={instance} />
    </div>
  );
}
