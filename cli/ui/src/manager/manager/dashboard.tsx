import { IconShieldHalfFilled } from "@tabler/icons-react";
import { Activity, useState } from "react";
import { DashboardLayout } from "../components/layout";
import {
  type ActiveVIew,
  SecretManagerChips,
  SecretManagerTabs,
} from "../components/manager-header";
import { SecretsTabDashboard } from "./environments/dashboard";

export interface SecretManagerDashboardProps {
  is_active: boolean;
  onSelection: () => void;
}

export function SecretManagerDashboard({
  is_active,
  onSelection,
}: SecretManagerDashboardProps) {
  const [active_view, setActiveView] = useState<ActiveVIew>("secrets");
  return (
    <DashboardLayout
      title="Secret Manager"
      is_active={is_active}
      Icon={IconShieldHalfFilled}
      onSelection={onSelection}
      tabs={
        <SecretManagerTabs
          active_view={active_view}
          setActiveView={setActiveView}
        />
      }
      chips={<SecretManagerChips />}
    >
      <Activity mode={active_view === "secrets" ? "visible" : "hidden"}>
        <SecretsTabDashboard />
      </Activity>
    </DashboardLayout>
  );
}
