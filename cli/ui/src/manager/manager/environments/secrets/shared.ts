import { useCallback } from "react";
import {
  type EncryptSecretParams,
  encrypt_secret,
  type SecretpassEnvironment,
} from "@/core/web";
import { useSecretManager } from "@/manager/provider";
import type { SecretManagerConfig } from "@/types";
import { makeRequest } from "@/utils";

export function useCreateOrUpdateSecret(environment: SecretpassEnvironment) {
  const manager = useSecretManager();

  return useCallback(
    async (name: string, description: string, value: string) => {
      const config: SecretManagerConfig = JSON.parse(
        JSON.stringify(manager.config),
      );
      const params: EncryptSecretParams = {
        login_params: {
          user: manager.user,
          project: config.project,
          public_keys: [manager.public_key],
        },
        public_keys: manager.lock_package[environment.name].public_keys,
        secret_value: value,
      };

      const secret_value_json = await encrypt_secret(JSON.stringify(params));
      const secret_value: Record<string, string> =
        JSON.parse(secret_value_json);
      const response = await makeRequest(
        `/api/secrets/${environment.name}/${name}`,
        {
          method: "PUT",
          body: JSON.stringify({
            description,
            data_type: "Text",
            value: secret_value,
          }),
          headers: {
            "Content-Type": "application/json",
          },
        },
      );

      if (!response.ok) {
        return Promise.reject(response.text);
      }

      await manager.refreshState();
    },
    [environment, manager],
  );
}
