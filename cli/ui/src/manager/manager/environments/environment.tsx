import { Alert, Description } from "@heroui/react";
import { useCallback, useEffect, useState } from "react";
import type { EnvironmentLockPackage, SecretpassEnvironment } from "@/core/web";
import { useSecretManager } from "@/manager/provider";
import { makeRequest } from "@/utils";
import { NewSecret } from "./secrets/new-secret";
import { ViewSecret } from "./secrets/show-secret";

export function SingleEnvironmentDashboard({
  environment,
}: {
  environment: SecretpassEnvironment;
}) {
  const manager = useSecretManager();
  const [env_package, setLockPackage] = useState<EnvironmentLockPackage | null>(
    null,
  );
  const [error, setError] = useState<string | null>(null);

  const fetchLockPackage = useCallback(() => {
    setError(null);
    makeRequest<EnvironmentLockPackage>(
      `/api/environment/${environment.name}/lock-package/${manager.public_key.id}`,
    )
      .then((response) => {
        if (!response.ok) {
          setLockPackage(null);
          setError(response.text);
          return;
        }

        setLockPackage(response.json());
      })
      .catch((err) => setError(`Error loading lock package: ${err}`));
  }, [environment.name, manager.public_key.id]);

  useEffect(() => {
    fetchLockPackage();
  }, [fetchLockPackage]);

  return (
    <div className="px-4 h-full w-2/3 relative overflow-y-auto">
      <div className="flex justify-between items-start mt-6">
        <div className="text-2xl font-mono font-bold">{environment.name}</div>
      </div>
      <Description className="mt-1">{environment.description}</Description>
      <hr className="my-4" />

      {error && (
        <Alert className="dark:bg-background" status="danger">
          <Alert.Indicator />
          <Alert.Content>
            <Alert.Title>Error registering passkey</Alert.Title>
            <Alert.Description>{error}</Alert.Description>
          </Alert.Content>
        </Alert>
      )}
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
        {env_package !== null &&
          environment.secrets?.map((secret) => (
            <ViewSecret
              key={secret.name}
              encrypted_value={
                env_package?.encrypted_secrets[secret.name] || null
              }
              environment={environment}
              onComplete={fetchLockPackage}
              public_keys={env_package.public_keys}
              secret={secret}
            />
          ))}
      </div>
      {env_package !== null && (
        <NewSecret
          onComplete={fetchLockPackage}
          environment={environment}
          public_keys={env_package.public_keys}
        />
      )}
    </div>
  );
}
