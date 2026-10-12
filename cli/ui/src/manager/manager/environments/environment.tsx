import { Alert, Button, Description, IconPlus } from "@heroui/react";
import { useState } from "react";
import type { SecretpassEnvironment } from "@/core/web";
import { NewSecret } from "./secrets/new-secret";
import { ViewSecret } from "./secrets/show-secret";

export function SingleEnvironmentDashboard({
  environment,
}: {
  environment: SecretpassEnvironment;
}) {
  const [creating, setCreating] = useState(!environment.secrets?.length);

  return (
    <div className="px-4 h-full w-2/3 relative overflow-y-auto z-30">
      <div className="flex flex-col bg-surface top-0 sticky">
        <div className="flex justify-between items-start mt-6">
          <div className="text-2xl font-mono font-bold">{environment.name}</div>
          <Button
            size="sm"
            variant="secondary"
            onClick={() => setCreating(true)}
          >
            <IconPlus /> Add Secret
          </Button>
        </div>
        <Description className="mt-1">{environment.description}</Description>
        <hr className="my-4" />
      </div>

      {!environment.secrets?.length && (
        <Alert className="dark:bg-background" status="warning">
          <Alert.Indicator />
          <Alert.Content>
            <Alert.Title className="font-mono">
              secrets.length === 0
            </Alert.Title>
            <Alert.Description>
              No secrets added to this environment
            </Alert.Description>
          </Alert.Content>
        </Alert>
      )}
      <div className="flex flex-col gap-4">
        {environment.secrets?.map((secret) => (
          <ViewSecret
            key={secret.name}
            environment={environment}
            secret={secret}
          />
        ))}
      </div>
      {creating && (
        <NewSecret
          environment={environment}
          onDismiss={() => setCreating(false)}
        />
      )}
    </div>
  );
}
