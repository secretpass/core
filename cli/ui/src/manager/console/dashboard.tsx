import { IconTerminal2 } from "@tabler/icons-react";
import {
  ProcessManagerChips,
  ProcessManagerTabs,
} from "../components/console-header";
import { DashboardLayout } from "../components/layout";

export interface ConsoleDashboardProps {
  is_active: boolean;
  onSelection: () => void;
}

export function ConsoleDashboard({
  is_active,
  onSelection,
}: ConsoleDashboardProps) {
  return (
    <DashboardLayout
      title="Process Manager"
      is_active={is_active}
      Icon={IconTerminal2}
      onSelection={onSelection}
      tabs={<ProcessManagerTabs />}
      chips={<ProcessManagerChips />}
    >
      Manager content here
    </DashboardLayout>
  );
}
