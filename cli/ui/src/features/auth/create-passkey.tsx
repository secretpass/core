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
import {
  decode_registration_config,
  type RegistrationParams,
  register_key_from_config,
} from "@/core/web";
import { useAnalytics } from "../analytics/provider";

export function CreatePasskeyScreen() {
  const [config, setConfig] = useState("");
  const [name, setName] = useState("");
  const [public_key_config, setPublicKeyConfig] = useState<string | null>(null);
  const analytics = useAnalytics();

  const error = useError();
  const [decoded_config, setDecodedConfig] =
    useState<RegistrationParams | null>(null);

  useEffect(() => {
    error.catch(
      "Error decoding config",
      async () => {
        setDecodedConfig(null);
        if (!config) {
          return;
        }
        const result: RegistrationParams = JSON.parse(
          decode_registration_config(config),
        );
        setDecodedConfig(result);
      },
      "user_passkey_config_decode",
    );
  }, [config, error.catch, error]);

  const register = useCallback(() => {
    error.catch(
      "Error registering new passkey",
      async () => {
        if (!decoded_config) {
          return;
        }

        const pk_config = await register_key_from_config(name, config);

        analytics?.group("local-project", decoded_config?.project.id, {
          name: decoded_config?.project.name,
          algorithm: decoded_config?.project.algorithm,
          residency: decoded_config?.project.residency,
        });
        analytics?.identify(decoded_config.user.id, {
          email: decoded_config.user.username,
          name: decoded_config.user.name,
          level: decoded_config.user.level,
        });
        analytics?.capture("user_passkey_registration");
        setPublicKeyConfig(pk_config);
      },
      "user_passkey_registration",
    );
  }, [name, config, error.catch, decoded_config, analytics]);

  return (
    <div className="flex flex-col gap-8 grow">
      {!public_key_config && (
        <TextField>
          <Label htmlFor="config">Key Generation Config</Label>
          <Description>
            Paste your key generation config as shared by your admin
          </Description>
          <TextArea
            rows={5}
            name="config"
            variant="secondary"
            spellCheck={false}
            value={config}
            onChange={(evt) => setConfig(evt.currentTarget.value)}
          />
        </TextField>
      )}

      {!!decoded_config && (
        <div>
          <div className="flex justify-between">
            <div className="text-lg font-semibold">
              {decoded_config.project.name}
            </div>
            <div className="text-sm text-muted">
              {decoded_config.project.id}
            </div>
          </div>
          <Description>{decoded_config.project.name}</Description>

          <hr className="my-4" />

          <TextField isDisabled className="mt-4">
            <Label>Name</Label>
            <Input variant="secondary" value={decoded_config.user.name} />
          </TextField>

          <TextField isDisabled className="mt-4">
            <Label>Name</Label>
            <Input variant="secondary" value={decoded_config.user.username} />
          </TextField>

          <TextField isRequired className="mt-4">
            <Label>Passkey</Label>
            <Input
              variant="secondary"
              placeholder="My passkey..."
              value={name}
              onChange={(evt) => setName(evt.currentTarget.value)}
            />
            <Description>
              Provide a descriptive name that can help you easily identify this
              passkey
            </Description>
          </TextField>
        </div>
      )}

      <ErrorDisplay value={error.value} />
      {!public_key_config && (
        <div className="flex justify-end">
          <Button
            onClick={register}
            isDisabled={!decoded_config || !name}
            aria-label="Register Passkey"
            size="lg"
          >
            <IconFingerprint />
            Register Passkey
          </Button>
        </div>
      )}

      {!!public_key_config && (
        <TextField className="mb-6">
          <Label className="text-success">Public Key Config</Label>
          <Description className="text-success mb-2">
            Share this key with your admin, to complete registration and give
            access to your passkey.
            <ul className="list-disc mx-6">
              <li>
                This config is not sensitive, it can be shared over open
                channels
              </li>
              <li>
                You can close this screen, the config facilitates an async
                process
              </li>
            </ul>
          </Description>
          <TextArea
            variant="secondary"
            readOnly
            onFocus={(evt) => evt.currentTarget.select()}
            className="field-sizing-content resize-none max-h-32"
            value={public_key_config}
          />
        </TextField>
      )}
    </div>
  );
}
