import {
  Button,
  Description,
  Input,
  Label,
  TextArea,
  TextField,
} from "@heroui/react";
import { IconFingerprint } from "@tabler/icons-react";
import { useCallback, useState } from "react";
import { ErrorDisplay, useError } from "@/components";
import { EnvironmentSelector } from "@/components/select";
import {
  type GenerateMachineKeyParams,
  type GenerateMachineKeyResult,
  generate_machine_key,
  type SecretpassUser,
} from "@/core/web";
import { useSecretManager } from "@/manager/provider";
import { resolveRequest, useComplexState } from "@/utils";

interface GenerationState {
  name: string;
  environments: string[];
}

export function GeneratePrivateKey() {
  const manager = useSecretManager();
  const [private_key, setPrivateKey] = useState<string | null>(null);
  const error = useError();
  const details = useComplexState<GenerationState>(() => ({
    name: "",
    environments: [],
  }));

  const reset = useCallback(() => {
    details.reset();
    setPrivateKey(null);
  }, [details.reset]);

  const generate = useCallback(async () => {
    const secrets_package = manager.getSecretsPackage(
      "Machine",
      details.environments,
    );

    const params: GenerateMachineKeyParams = {
      login_params: manager.login_params,
      secrets_package,
    };

    const gen_results = await error.catch("Error generating private key", () =>
      generate_machine_key(details.name, JSON.stringify(params)),
    );
    if (!gen_results.ok) {
      return;
    }

    const result: GenerateMachineKeyResult = JSON.parse(gen_results.result);
    const user: SecretpassUser = {
      id: result.public_key.id,
      name: details.name,
      level: "Machine",
      access: details.environments,
      created_at: new Date().toISOString(),
    };
    const data = {
      user,
      public_key: result.public_key,
      secret_values: result.secrets_package,
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

    manager.refreshState().then(() => {
      setPrivateKey(result.private_key);
    });
  }, [manager, details, error.catch]);

  return (
    <div className="flex flex-col gap-6">
      <TextField isDisabled={!!private_key}>
        <Label>Key name</Label>
        <Input
          placeholder="My Secret name"
          value={details.name}
          onChange={(evt) => details.update({ name: evt.currentTarget.value })}
        />
      </TextField>

      <EnvironmentSelector<string[]>
        multiselect
        disabled={!!private_key}
        value={details.environments}
        label="Environment"
        onChange={(env) => details.update({ environments: env })}
        description="The environments this key is allowed to access."
      />

      {!!private_key && (
        <TextField>
          <Label className="text-success">Private Key</Label>
          <TextArea
            readOnly
            onFocus={(evt) => evt.currentTarget.select()}
            rows={5}
            value={private_key}
            className="field-sizing-content resize-none"
          />
          <Description>
            Store this key securely in your CI, cloud provider or other options.
            You can use to access secrets without human intervention by:
            <br />
            <div className="px-4 py-2 my-2 bg-black text-white font-mono font-semibold">
              spass --private-key="my-private-key" --env="
              {details.environments[0] ?? ""}" -- your command here
            </div>
            <ul className="list-disc mx-6 text-danger">
              <li>
                This config is sensitive and <strong>should not</strong> be
                shared over open channels
              </li>
              <li>If compromised, delete the key to remove it's access.</li>
            </ul>
          </Description>
        </TextField>
      )}

      <ErrorDisplay value={error.value} />

      <div className="flex justify-end">
        {!!private_key && (
          <Button variant="ghost" onClick={reset}>
            Reset
          </Button>
        )}
        {!private_key && (
          <Button
            isDisabled={!details.name || details.environments.length === 0}
            onClick={generate}
          >
            <IconFingerprint />
            Generate Private Key
          </Button>
        )}
      </div>
    </div>
  );
}
