import { useCallback, useEffect, useState } from "react";
import { useError } from "@/components";
import type { CommandInstanceDetails } from "@/core/web";
import { InstancesProvider } from "@/features/console/provider";
import { resolveRequest } from "@/utils";
import { ConsoleDashboard } from "../features/console/dashboard";
import { SecretManagerDashboard } from "./manager/dashboard";

export function SecretManagerLayout() {
  const [active_view, setActiveView] = useState<"console" | "manager">(
    "manager",
  );

  const instance_error = useError();

  const [instances, setInstances] = useState<CommandInstanceDetails[] | null>(
    null,
  );
  const refreshCommandInstances = useCallback(async () => {
    const result = await instance_error.catch(
      "Error fetching command instances",
      () => resolveRequest<CommandInstanceDetails[]>("/api/run/instances"),
    );

    if (!result.ok) {
      // Return true if we have a pending action, e.g. error
      return true;
    }

    setInstances(result.result);
    // We have a pending command that needs to be authorized
    return result.result[0]?.status === "Pending";
  }, [instance_error.catch]);

  useEffect(() => {
    // On first run, set active view as console if there's a pending action
    refreshCommandInstances().then((is_pending) => {
      if (is_pending) {
        setActiveView("console");
      }
    });

    // Refresh every 2.5 seconds unless manually refreshed
    const interval = setInterval(() => refreshCommandInstances(), 2500);

    return () => clearInterval(interval);
  }, [refreshCommandInstances]);

  return (
    <div className="content py-4 transition-all duration-700">
      <SecretManagerDashboard
        is_active={active_view === "manager"}
        onSelection={() => setActiveView("manager")}
      />

      {instances !== null && (
        <InstancesProvider
          instances={instances}
          refreshInstances={refreshCommandInstances}
        >
          <ConsoleDashboard
            error={instance_error.value}
            is_active={active_view === "console"}
            onSelection={() => setActiveView("console")}
          />
        </InstancesProvider>
      )}
    </div>
  );
}
