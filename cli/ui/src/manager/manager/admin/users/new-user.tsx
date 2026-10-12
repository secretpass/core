import { Button, Tabs } from "@heroui/react";
import {
  IconLockCheck,
  IconLockCode,
  IconLockPlus,
  IconX,
} from "@tabler/icons-react";
import { useEffect, useRef } from "react";
import { CompleteRegistration } from "./complete-registration";
import { GeneratePrivateKey } from "./private-key";
import { StartRegistration } from "./start-registration";

export function NewUser({ onDismiss }: { onDismiss: () => void }) {
  const ref = useRef<HTMLDivElement>(null);

  useEffect(() => {
    ref.current?.scrollIntoView();
  }, []);

  return (
    <div
      ref={ref}
      className={"flex flex-col p-4 mt-3 mb-6 rounded-3xl bg-background"}
    >
      <div className="text-xl mb-4 flex justify-between items-center text-muted">
        Setup user/public key
        <Button
          isIconOnly
          size="sm"
          variant="ghost"
          className="text-muted hover:text-danger"
          onClick={onDismiss}
        >
          <IconX className="size-4" />
        </Button>
      </div>

      <Tabs>
        <Tabs.ListContainer>
          <Tabs.List>
            <Tabs.Tab id="generate">
              <div className="flex gap-4 items-center">
                <IconLockPlus className="size-6" />
                Start passkey registration
              </div>
              <Tabs.Indicator />
            </Tabs.Tab>
            <Tabs.Tab id="complete">
              <div className="flex gap-4 items-center">
                <IconLockCheck className="size-6" />
                Complete passkey registration
              </div>
              <Tabs.Indicator />
            </Tabs.Tab>
            <Tabs.Tab id="machine">
              <div className="flex gap-4 items-center">
                <IconLockCode className="size-6" />
                Generate Machine Key
              </div>
              <Tabs.Indicator />
            </Tabs.Tab>
          </Tabs.List>
        </Tabs.ListContainer>
        <Tabs.Panel id="generate">
          <StartRegistration />
        </Tabs.Panel>
        <Tabs.Panel id="machine">
          <GeneratePrivateKey />
        </Tabs.Panel>
        <Tabs.Panel id="complete">
          <CompleteRegistration onComplete={onDismiss} />
        </Tabs.Panel>
      </Tabs>
    </div>
  );
}
