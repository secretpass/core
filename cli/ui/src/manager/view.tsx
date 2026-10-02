import { Card, IconChevronUp } from "@heroui/react";
import { IconShieldHalfFilled, IconTerminal2 } from "@tabler/icons-react";
import clsx from "clsx";
import { useMemo, useState } from "react";
import {
  ProcessManagerChips,
  ProcessManagerTabs,
} from "./components/console-header";
import {
  SecretManagerChips,
  SecretManagerTabs,
} from "./components/manager-header";
import { useSecretManager } from "./provider";

export function SecretManagerView() {
  const manager = useSecretManager();
  const [active_view, setActiveView] = useState<"console" | "manager">(
    "manager",
  );

  const console_active = useMemo(
    () => active_view === "console",
    [active_view],
  );
  const manager_active = useMemo(
    () => active_view === "manager",
    [active_view],
  );

  return (
    <div
      className={clsx(
        "grow flex items-center gap-4 py-4 transition-all duration-700",
        manager_active && "flex-col",
        console_active && "flex-col-reverse",
      )}
    >
      <Card
        className={clsx(
          "container h-4/5 transition-all duration-700 ease-in-out",
          manager_active && "grow",
          !manager_active &&
            "hover:bg-background-secondary cursor-pointer group",
        )}
        onClick={!manager_active ? () => setActiveView("manager") : undefined}
      >
        <Card.Header>
          <Card.Title
            className={clsx(
              "flex justify-between items-end",
              !manager_active && "gap-4",
            )}
          >
            <div
              className={clsx(
                "grow flex gap-1 text-xl items-center select-none",
                manager_active && "border-b",
              )}
            >
              <IconShieldHalfFilled />
              Secret Manager -{" "}
              <span className="text-muted">{manager.config?.project.name}</span>
            </div>
            {!manager_active && (
              <>
                <SecretManagerChips />
                <IconChevronUp />
              </>
            )}
            {manager_active && <SecretManagerTabs />}
          </Card.Title>
        </Card.Header>
      </Card>

      <Card
        className={clsx(
          "container h-4/5 transition-all duration-700 ease-in-out",
          console_active && "grow",
          !console_active && "hover:bg-background-secondary cursor-pointer",
        )}
        onClick={!console_active ? () => setActiveView("console") : undefined}
      >
        <Card.Header>
          <Card.Title
            className={clsx(
              "flex justify-between items-end",
              !console_active && "gap-4",
            )}
          >
            <div
              className={clsx(
                "grow flex gap-1 text-xl items-center select-none",
                console_active && "border-b",
              )}
            >
              <IconTerminal2 />
              Process Manager -{" "}
              <span className="text-muted">{manager.config?.project.name}</span>
            </div>
            {!console_active && (
              <>
                <ProcessManagerChips />
                <IconChevronUp />
              </>
            )}
            {console_active && <ProcessManagerTabs />}
          </Card.Title>
        </Card.Header>
      </Card>
    </div>
  );
}
