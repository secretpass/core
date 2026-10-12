import { Chip, IconPlus } from "@heroui/react";
import { IconCpu, IconFlask, IconFolder } from "@tabler/icons-react";
import clsx from "clsx";
import type { CommandInstanceDetails } from "@/core/web";
import { useInstances } from "@/features/console/provider";
import {
  InstanceStatusIcon,
  useProcessEnvironment,
  useProcessId,
} from "@/features/console/shared";

export function CommandInstancesSidebar() {
  const { active, setActive, instances } = useInstances();
  return (
    <div className="bg-background h-full py-4 px-2 divide-y rounded-2xl w-1/3 overflow-y-auto relative grid-cols-1">
      {instances.map((instance) => (
        <CommandNavItem
          key={instance.id}
          details={instance}
          is_active={instance.id === active}
          onSelection={setActive}
        />
      ))}

      <CreateButton
        key="new"
        is_active={active === "new"}
        onSelection={setActive}
      />
    </div>
  );
}

function CreateButton({
  is_active,
  onSelection,
}: {
  is_active: boolean;
  onSelection: (id: string) => void;
}) {
  return (
    <button
      className={clsx(
        "flex items-center w-full mb-3 cursor-pointer py-2 px-4 text-left gap-4 rounded-2xl transition-all duration-300",
        is_active && "bg-surface",
        !is_active && " hover:bg-background-secondary hover:rounded-2xl",
      )}
      onClick={() => onSelection("new")}
      type="button"
    >
      <IconPlus /> Run Command
    </button>
  );
}

function CommandNavItem({
  details,
  is_active,
  onSelection,
}: {
  is_active: boolean;
  details: CommandInstanceDetails;
  onSelection: (id: string) => void;
}) {
  const process_id = useProcessId(details.status);
  const environment = useProcessEnvironment(details.status);

  return (
    <button
      className={clsx(
        "flex items-center w-full mb-3 cursor-pointer py-2 px-4 text-left gap-4 rounded-2xl transition-all duration-300",
        is_active && "bg-surface",
        !is_active && "hover:bg-background-secondary hover:rounded-2xl",
      )}
      onClick={() => onSelection(details.id)}
      type="button"
    >
      <InstanceStatusIcon status={details.status} className="" />
      <div className="grow flex flex-col gap-1">
        <span
          className={clsx(
            "font-mono",
            is_active && "font-semibold",
            !is_active && "font-light",
          )}
        >
          {details.command}
        </span>
        <div className="flex gap-1 items-center flex-wrap">
          <span className="flex items-center gap-1 text-xm text-muted font-mono">
            <IconFolder className="size-4" />
            {details.workdir}
          </span>
          <div className="grow" />

          {environment !== null && (
            <Chip color="accent">
              <IconFlask className="size-4" />
              {environment}
            </Chip>
          )}
          {process_id !== null && (
            <Chip color="accent" variant="primary">
              <IconCpu className="size-4" />
              {process_id}
            </Chip>
          )}
        </div>
      </div>
    </button>
  );
}
