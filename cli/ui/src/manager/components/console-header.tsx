import { Chip, IconPlus, Tabs } from "@heroui/react";
import {
  IconBrandDocker,
  IconBrandNpm,
  IconCheck,
  IconHome,
  IconRefresh,
  IconX,
} from "@tabler/icons-react";

export function ProcessManagerTabs() {
  return (
    <Tabs variant="secondary">
      <Tabs.ListContainer>
        <Tabs.List aria-label="Secret Manager">
          <Tabs.Tab id="overview" className="w-auto">
            <span className="flex gap-1 items-center">
              <IconHome className="size-4" />
              Overview
            </span>
            <Tabs.Indicator />
          </Tabs.Tab>
          <Tabs.Tab id="npm-process" className="w-auto">
            <span className="flex gap-1 items-center">
              <IconBrandNpm className="size-4" />
              npm run
            </span>
            <Tabs.Indicator />
          </Tabs.Tab>
          <Tabs.Tab id="docker-process" className="w-auto">
            <span className="flex gap-1 items-center">
              <IconBrandDocker className="size-4" />
              docker run
            </span>
            <Tabs.Indicator />
          </Tabs.Tab>
          <Tabs.Tab id="new" className="w-auto">
            <span className="flex gap-1 items-center">
              <IconPlus className="size-4" />
              run new
            </span>
            <Tabs.Indicator />
          </Tabs.Tab>
        </Tabs.List>
      </Tabs.ListContainer>
    </Tabs>
  );
}

export function ProcessManagerChips() {
  return (
    <div className="flex gap-2">
      <Chip variant="soft" color="success" size="md">
        <IconRefresh className="size-4 animate-spin" />
        <Chip.Label>6 running</Chip.Label>
      </Chip>
      <Chip variant="soft" color="danger" size="md">
        <IconX className="size-4" />
        <Chip.Label>3 error</Chip.Label>
      </Chip>
      <Chip variant="soft" color="accent" size="md">
        <IconCheck className="size-4" />
        <Chip.Label>1 success</Chip.Label>
      </Chip>
    </div>
  );
}
