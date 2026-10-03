import { Card, IconChevronUp } from "@heroui/react";
import type { TablerIcon } from "@tabler/icons-react";
import clsx from "clsx";
import { Activity, type ReactNode } from "react";
import { useSecretManager } from "../provider";

export interface DashboardLayoutProps {
  is_active: boolean;
  Icon: TablerIcon;
  title: string;
  chips: ReactNode;
  tabs: ReactNode;
  children: ReactNode;
  onSelection: () => void;
}

export function DashboardLayout({
  is_active,
  onSelection,
  Icon,
  title,
  chips,
  children,
  tabs,
}: DashboardLayoutProps) {
  const manager = useSecretManager();
  return (
    <Card
      className={clsx(
        "container mx-auto my-2 transition-all duration-700 ease-in-out relative",
        is_active && "dashboard-active",
        !is_active &&
          "dashboard-inactive hover:bg-background-secondary cursor-pointer group",
      )}
      onClick={!is_active ? () => onSelection() : undefined}
    >
      <Card.Header className="sticky top-0">
        <Card.Title
          className={clsx(
            "flex justify-between items-end",
            !is_active && "gap-4",
          )}
        >
          <div
            className={clsx(
              "grow flex gap-1 text-xl items-center select-none",
              is_active && "border-b",
            )}
          >
            <Icon />
            {title} -{" "}
            <span className="text-muted">{manager.config.project.name}</span>
          </div>
          {!is_active && (
            <>
              {chips}
              <IconChevronUp />
            </>
          )}

          <Activity mode={is_active ? "visible" : "hidden"}>{tabs}</Activity>
        </Card.Title>
      </Card.Header>
      <Activity mode={is_active ? "visible" : "hidden"}>
        <Card.Content className="dashboard-content-height">
          {children}
        </Card.Content>
      </Activity>
    </Card>
  );
}
