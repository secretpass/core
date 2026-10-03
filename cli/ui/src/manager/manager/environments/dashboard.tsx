import { useMemo, useState } from "react";
import { useSecretManager } from "@/manager/provider";
import { SingleEnvironmentDashboard } from "./environment";
import { EnvironmentsSidebar } from "./sidebar";

export function SecretsTabDashboard() {
  const manager = useSecretManager();
  const [active, setActiveEnvironment] = useState<string | null>(
    manager?.config?.environments?.[0]?.name || null,
  );

  const environment = useMemo(
    () => manager.config.environments.find((env) => env.name === active),
    [manager.config.environments, active],
  );

  return (
    <section className="h-full flex">
      <EnvironmentsSidebar
        selected={active}
        onSelection={setActiveEnvironment}
      />

      {!!environment && (
        <SingleEnvironmentDashboard environment={environment} />
      )}
    </section>
  );
}
