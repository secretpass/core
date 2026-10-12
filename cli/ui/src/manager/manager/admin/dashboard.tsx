import { Activity, useState } from "react";
import { AdministrationSidebar, type AdminView } from "./admin-sidebar";
import { ProjectOverview } from "./overview";
import { UsersDashboard } from "./users/dashboard";

export function AdministrationTabDashboard() {
  const [active_view, setActiveView] = useState<AdminView>("project");

  return (
    <section className="h-full flex">
      <AdministrationSidebar
        active_view={active_view}
        setActiveView={setActiveView}
      />
      <Activity mode={active_view === "project" ? "visible" : "hidden"}>
        <ProjectOverview />
      </Activity>

      <Activity mode={active_view === "users" ? "visible" : "hidden"}>
        <UsersDashboard />
      </Activity>
    </section>
  );
}
