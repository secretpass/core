import { Description } from "@heroui/react";
import {
  IconShieldCog,
  IconUserCog,
  type TablerIcon,
} from "@tabler/icons-react";
import clsx from "clsx";

export type AdminView = "project" | "users";

const VIEWS: {
  id: AdminView;
  Icon: TablerIcon;
  label: string;
  description: string;
}[] = [
  {
    id: "project",
    Icon: IconShieldCog,
    label: "Project Overview",
    description: "Basic project details and key management settings",
  },
  {
    id: "users",
    Icon: IconUserCog,
    label: "Manage Users",
    description: "Manage users, public keys and environment access",
  },
];

export function AdministrationSidebar({
  active_view,
  setActiveView,
}: {
  active_view: AdminView;
  setActiveView: (view: AdminView) => void;
}) {
  return (
    <div className="bg-background h-full py-4 px-2 rounded-2xl w-1/3 overflow-y-auto relative grid-cols-1">
      {VIEWS.map((view) => (
        <button
          key={view.id}
          className={clsx(
            "flex items-center gap-4 w-full mb-3 cursor-pointer py-2 px-4 text-left rounded-2xl transition-all duration-300",
            active_view === view.id && "bg-surface",
            active_view !== view.id &&
              "hover:bg-background-secondary hover:rounded-2xl",
          )}
          onClick={() => setActiveView(view.id)}
          type="button"
        >
          <view.Icon />

          <div className="flex flex-col">
            <span
              className={clsx(
                active_view === view.id && "font-semibold",
                active_view !== view.id && "font-light",
              )}
            >
              {view.label}
            </span>
            <Description>{view.description}</Description>
          </div>
        </button>
      ))}
    </div>
  );
}
