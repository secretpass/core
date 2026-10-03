import { toast } from "@heroui/react";
import { useCallback } from "react";
import {
  type EncryptSecretParams,
  encrypt_secret,
  type SecretpassEnvironment,
  type StoredPublicKey,
} from "@/core/web";
import { useSecretManager } from "@/manager/provider";
import type { SecretManagerConfig } from "@/types";
import { makeRequest } from "@/utils";

export function useCreateOrUpdateSecret(
  environment: SecretpassEnvironment,
  public_keys: StoredPublicKey[],
  onComplete: () => void,
) {
  const manager = useSecretManager();

  return useCallback(
    (name: string, description: string, value: string) => {
      const config: SecretManagerConfig = JSON.parse(
        JSON.stringify(manager.config),
      );
      const params: EncryptSecretParams = {
        login_params: {
          user: manager.user,
          project: config.project,
          public_keys: [manager.public_key],
        },
        public_keys,
        secret_value: value,
      };

      encrypt_secret(JSON.stringify(params))
        .then((value_json) => {
          const secret_value: Record<string, string> = JSON.parse(value_json);
          return makeRequest(`/api/secrets/${environment.name}/${name}`, {
            method: "PUT",
            body: JSON.stringify({
              description,
              data_type: "Text",
              value: secret_value,
            }),
            headers: {
              "Content-Type": "application/json",
            },
          });
        })
        .then((response) => {
          if (!response.ok) {
            toast(response.text, { variant: "danger" });
            return;
          }

          let env = config.environments.find(
            (env) => env.name === environment.name,
          );
          if (!env) {
            env = environment;
            config.environments.push(environment);
          }
          if (!env.secrets) {
            env.secrets = [];
          }

          const secret = env.secrets.find((sec) => sec.name === name);
          if (!secret) {
            env.secrets.push({
              name,
              description,
              data_type: "Text",
            });
          } else {
            secret.description = description;
          }

          manager.update({ config });
          onComplete();
        })
        .catch((err) => toast(String(err), { variant: "danger" }));
    },
    [
      environment,
      manager.public_key,
      manager.user,
      public_keys,
      onComplete,
      manager.update,
      manager.config,
    ],
  );
}
