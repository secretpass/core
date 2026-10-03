import clsx from "clsx";
import { useState } from "react";

import { ConsoleDashboard } from "./console/dashboard";
import { SecretManagerDashboard } from "./manager/dashboard";

export function SecretManagerLayout() {
  const [active_view, setActiveView] = useState<"console" | "manager">(
    "manager",
  );

  return (
    <div
      className={clsx(
        "content gap-4 py-4 transition-all duration-700 max-h-full",
        active_view === "manager" && "flex-col",
        active_view === "console" && "flex-col-reverse",
      )}
    >
      <SecretManagerDashboard
        is_active={active_view === "manager"}
        onSelection={() => setActiveView("manager")}
      />

      <ConsoleDashboard
        is_active={active_view === "console"}
        onSelection={() => setActiveView("console")}
      />
    </div>
  );
}
