import { IconTerminal2 } from "@tabler/icons-react";
import { Activity } from "react";
import { ErrorDisplay, type ErrorValue } from "@/components";
import { ProcessManagerChips } from "@/features/console/console-header";
import { InstanceDashboard } from "@/features/console/instance/dashboard";
import { NewCommand } from "@/features/console/new-command";
import { CommandInstancesSidebar } from "@/features/console/sidebar";
import { DashboardLayout } from "@/manager/components/layout";
import { useInstances } from "./provider";

export interface ConsoleDashboardProps {
  is_active: boolean;
  error: ErrorValue | null;
  onSelection: () => void;
}

export function ConsoleDashboard({
  is_active,
  error,
  onSelection,
}: ConsoleDashboardProps) {
  const { active, instances } = useInstances();

  return (
    <DashboardLayout
      title="Process Manager"
      is_active={is_active}
      Icon={IconTerminal2}
      onSelection={onSelection}
      chips={<ProcessManagerChips />}
    >
      <ErrorDisplay value={error} />
      <section className="h-full flex">
        {!error && <CommandInstancesSidebar />}

        <Activity mode={active === "new" && !error ? "visible" : "hidden"}>
          <NewCommand />
        </Activity>

        {instances.map((instance) => (
          <Activity
            key={instance.id}
            mode={active === instance.id && !error ? "visible" : "hidden"}
          >
            <InstanceDashboard
              key={instance.id}
              instance={instance}
              is_active={active === instance.id}
            />
          </Activity>
        ))}
      </section>
    </DashboardLayout>
  );
}
