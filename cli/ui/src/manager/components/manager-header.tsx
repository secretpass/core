import { Chip, Tabs } from "@heroui/react";
import {
  IconFlask,
  IconSettings,
  IconShield,
  IconUser,
} from "@tabler/icons-react";
import { useMemo } from "react";
import { useSecretManager } from "../provider";

export type ActiveVIew = "secrets" | "administration";

export function SecretManagerTabs({
  active_view,
  setActiveView,
}: {
  active_view: ActiveVIew;
  setActiveView: (view: ActiveVIew) => void;
}) {
  return (
    <Tabs
      variant="secondary"
      selectedKey={active_view}
      onSelectionChange={(key) => setActiveView(key as ActiveVIew)}
    >
      <Tabs.ListContainer>
        <Tabs.List aria-label="Secret Manager">
          <Tabs.Tab id="secrets">
            <span className="flex gap-1 items-center">
              <IconShield className="size-4" />
              Secrets
            </span>
            <Tabs.Indicator />
          </Tabs.Tab>
          <Tabs.Tab id="administration">
            <span className="flex gap-1 items-center">
              <IconSettings className="size-4" />
              Administration
            </span>
            <Tabs.Indicator />
          </Tabs.Tab>
        </Tabs.List>
      </Tabs.ListContainer>
    </Tabs>
  );
}

export function SecretManagerChips() {
  const manager = useSecretManager();

  const secrets = useMemo(() => {
    return (
      manager.config?.environments.reduce(
        (count, env) => count + (env.secrets?.length ?? 0),
        0,
      ) ?? 0
    );
  }, [manager.config?.environments]);

  return (
    <div className="flex gap-2">
      <Chip variant="soft" color="accent" size="md">
        <IconShield className="size-4" />
        <Chip.Label>{secrets} secrets</Chip.Label>
      </Chip>
      <Chip variant="soft" color="accent" size="md">
        <IconFlask className="size-4" />
        <Chip.Label>
          {manager.config?.environments.length ?? 0} environments
        </Chip.Label>
      </Chip>
      <Chip variant="soft" color="accent" size="md">
        <IconUser className="size-4" />
        <Chip.Label>{manager.config?.users.length ?? 0} users</Chip.Label>
      </Chip>
    </div>
  );
}
