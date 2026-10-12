import { Description } from "@heroui/react";
import clsx from "clsx";
import { useMemo } from "react";
import type { SecretpassEnvironment } from "@/core/web";
import { useSecretManager } from "@/manager/provider";
import { CreateEnvironment } from "./new-environment";

export function EnvironmentsSidebar({
  selected,
  onSelection,
}: {
  selected: string | null;
  onSelection: (env: string) => void;
}) {
  const manager = useSecretManager();
  const environments = useMemo(
    () =>
      manager.config.environments.filter(
        (env) => !!manager.lock_package[env.name],
      ),
    [manager.config.environments, manager.lock_package],
  );

  return (
    <div className="bg-background h-full py-4 px-2 rounded-2xl w-1/3 overflow-y-auto relative grid-cols-1">
      {environments.map((env) => (
        <EnvironmentItem
          key={env.name}
          is_active={selected === env.name}
          environment={env}
          onSelection={onSelection}
        />
      ))}

      {manager.user.level === "Admin" && (
        <CreateEnvironment setActiveEnvironment={onSelection} />
      )}
    </div>
  );
}

function EnvironmentItem({
  environment,
  is_active,
  onSelection,
}: {
  is_active: boolean;
  environment: SecretpassEnvironment;
  onSelection: (env: string) => void;
}) {
  return (
    <button
      className={clsx(
        "flex flex-col w-full mb-3 cursor-pointer py-2 px-4 text-left gap-1 rounded-2xl transition-all duration-300",
        is_active && "bg-surface",
        !is_active && "hover:bg-background-secondary hover:rounded-2xl",
      )}
      onClick={() => onSelection(environment.name)}
      type="button"
    >
      <span
        className={clsx(
          "font-mono",
          is_active && "font-semibold",
          !is_active && "font-light",
        )}
      >
        {environment.name}
      </span>
      <Description className="line-clamp-1 truncate">
        {environment.description}
      </Description>
    </button>
  );
}
