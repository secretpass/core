import {
  Button,
  Description,
  Input,
  Label,
  TextArea,
  TextField,
} from "@heroui/react";
import { IconFingerprint } from "@tabler/icons-react";
import { useCallback, useEffect, useState } from "react";
import { ErrorDisplay, useError } from "@/components";
import { EnvironmentSelector } from "@/components/select";
import {
  decode_registration_result,
  type KeyCreationResult,
  type ReEncryptPackageParams,
  reencrypt_package,
  type StoredPublicKey,
} from "@/core/web";
import { useSecretManager } from "@/manager/provider";
import type { SecretManagerConfig } from "@/types";
import { assertProjectEqual, resolveRequest } from "@/utils";

export interface CompleteRegistrationProps {
  onComplete: () => void;
}

export function CompleteRegistration({
  onComplete,
}: CompleteRegistrationProps) {
  const manager = useSecretManager();
  const [config, setConfig] = useState<string>("");
  const error = useError();

  const [result, setResult] = useState<KeyCreationResult | null>(null);

  useEffect(() => {
    error.catch("Error decoding config", async () => {
      setResult(null);
      if (!config) {
        return;
      }
      const value = await decode_registration_result(config);
      const decoded_config: KeyCreationResult = JSON.parse(value);
      assertProjectEqual(decoded_config.project, manager.config.project);
      setResult(decoded_config);
    });
  }, [config, error.catch, manager.config.project]);

  const grantSuccess = useCallback(async () => {
    if (!result) {
      return;
    }

    const secrets_package = manager.getSecretsPackage(
      result.user.level,
      result.user.access || [],
    );
    const params: ReEncryptPackageParams = {
      login_params: manager.login_params,
      secrets_package,
      target_key: result.public_key,
    };

    const encryption_result = await error.catch(
      "Error encrypting user secrets",
      () => reencrypt_package(JSON.stringify(params)),
    );
    if (!encryption_result.ok) {
      return;
    }

    const data = {
      user: result.user,
      public_key: result.public_key,
      secret_values: JSON.parse(encryption_result.result),
    };

    const creation_result = await error.catch("Error adding public error", () =>
      resolveRequest("/api/user", {
        method: "POST",
        body: JSON.stringify(data),
      }),
    );

    if (!creation_result.ok) {
      return;
    }

    const config: SecretManagerConfig = JSON.parse(
      JSON.stringify(manager.config),
    );
    const public_keys: StoredPublicKey[] = JSON.parse(
      JSON.stringify(manager.public_keys),
    );
    if (config.users.findIndex((user) => user.id === result.user.id) < 0) {
      config.users.push(result.user);
    }
    public_keys.push(result.public_key);

    manager
      .refreshState(`Public key added successfully for ${result.user.username}`)
      .then(onComplete);
  }, [result, manager, error.catch, onComplete]);

  return (
    <div className="flex flex-col gap-6">
      <TextField>
        <Label>Passkey Config</Label>
        <Description>
          The key shared by the user after they registered a passkey on their
          device
        </Description>
        <TextArea
          onChange={(evt) => setConfig(evt.currentTarget.value)}
          rows={5}
          value={config}
        />
      </TextField>

      {!!result && (
        <>
          <TextField isDisabled>
            <Label>Email Address</Label>
            <Input value={result.user.username} />
          </TextField>

          <TextField isDisabled>
            <Label>Name</Label>
            <Input value={result.user.name} />
          </TextField>
          <TextField isDisabled>
            <Label>Access Level</Label>
            <Input value={result.user.level} />
          </TextField>
          {result.user.level !== "Admin" && (
            <EnvironmentSelector
              disabled
              multiselect
              label="Environment"
              value={result.user.access || []}
            />
          )}
        </>
      )}

      <ErrorDisplay value={error.value} />

      <div className="flex justify-end">
        <Button isDisabled={!result} onClick={grantSuccess}>
          <IconFingerprint />
          Grant access
        </Button>
      </div>
    </div>
  );
}
