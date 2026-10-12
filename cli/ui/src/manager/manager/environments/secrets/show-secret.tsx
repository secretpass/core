import {
  Button,
  Description,
  Label,
  TextArea,
  TextField,
  Tooltip,
  toast,
} from "@heroui/react";
import {
  IconFingerprint,
  IconLock,
  IconLockOpen,
  IconTrash,
} from "@tabler/icons-react";
import clsx from "clsx";
import { useCallback, useMemo, useState } from "react";
import {
  authorize_operation,
  type DecryptSecretParams,
  decrypt_secret,
  type LoginParams,
  type SecretDefinition,
  type SecretpassEnvironment,
} from "@/core/web";
import { useSecretManager } from "@/manager/provider";
import { resolveRequest } from "@/utils";
import { useCreateOrUpdateSecret } from "./shared";

interface ViewSecretProps {
  environment: SecretpassEnvironment;
  secret: SecretDefinition;
}

export function ViewSecret({ environment, secret }: ViewSecretProps) {
  const manager = useSecretManager();
  const [is_locked, setIsLocked] = useState(true);

  const [decrypted_value, setDecryptedValue] = useState<string>("");
  const [value, setSecretValue] = useState<string>("");

  const is_valid = useMemo(
    () => value.length > 1 && value !== decrypted_value,
    [value, decrypted_value],
  );

  const reset = useCallback(() => {
    setIsLocked(true);
    setSecretValue("");
    setDecryptedValue("");
  }, []);

  const onComplete = useCallback(() => {
    setDecryptedValue(value);
    toast(`${secret.name} updated successfully`, { variant: "success" });
  }, [value, secret.name]);

  const updateSecret = useCreateOrUpdateSecret(environment);

  const unlock = useCallback(() => {
    const encrypted_value =
      manager.lock_package[environment.name]?.encrypted_secrets?.[secret.name];
    if (!encrypted_value) {
      setIsLocked(false);
      setSecretValue("");
      setDecryptedValue("");
      return;
    }

    const params: DecryptSecretParams = {
      login_params: {
        user: manager.user,
        project: manager.config.project,
        public_keys: [manager.public_key],
      },
      encrypted_secret: encrypted_value,
    };

    decrypt_secret(JSON.stringify(params))
      .then((plain_secret) => {
        setSecretValue(plain_secret);
        setDecryptedValue(plain_secret);
        setIsLocked(false);
      })
      .catch((err) => toast(String(err), { variant: "danger" }));
  }, [manager, manager.lock_package, environment.name, secret.name]);

  const deleteSecret = useCallback(() => {
    const params: LoginParams = {
      user: manager.user,
      project: manager.config.project,
      public_keys: [manager.public_key],
    };
    authorize_operation(JSON.stringify(params))
      .then(() =>
        resolveRequest(`/api/secrets/${environment.name}/${secret.name}`, {
          method: "DELETE",
        }),
      )
      .then(() => manager.refreshState(`${secret.name} deleted`))
      .then(onComplete)
      .catch((err) =>
        toast(`Error deleting secret: ${err}`, {
          variant: "danger",
        }),
      );
  }, [manager, environment.name, secret.name, onComplete]);

  return (
    <div
      className={clsx(
        "flex flex-col py-2 px-4 duration-500 gap-2 bg-background rounded-xl",
      )}
    >
      <div
        className={clsx(
          "text-lg flex justify-between items-center font-mono",
          is_locked && "font-light",
        )}
      >
        {secret.name}
        <div>
          {!is_locked && (
            <Tooltip>
              <Button
                isIconOnly
                variant="ghost"
                size="sm"
                className="text-muted hover:text-danger"
                onDoubleClick={deleteSecret}
              >
                <IconTrash className="size-6" />
              </Button>
              <Tooltip.Content>Double click to delete</Tooltip.Content>
            </Tooltip>
          )}
          <Button
            isIconOnly
            variant="ghost"
            size="sm"
            className={clsx(
              "text-muted",
              is_locked && "hover:text-warning",
              !is_locked && "hover:text-success",
            )}
            onClick={is_locked ? unlock : reset}
          >
            {is_locked && <IconLock className="size-5" />}
            {!is_locked && <IconLockOpen className="size-5" />}
          </Button>
        </div>
      </div>
      {!!secret.description && <Description>{secret.description}</Description>}
      {!is_locked && (
        <>
          <TextField className="mb-4">
            <Label>Secret value</Label>
            <TextArea
              placeholder="Top secret information here..."
              value={value}
              onChange={(evt) => setSecretValue(evt.currentTarget.value)}
            />
          </TextField>
          {value !== decrypted_value && (
            <div className="flex justify-end mb-4">
              <Button
                type="button"
                isDisabled={!is_valid}
                onClick={() =>
                  updateSecret(secret.name, secret.description, value)
                }
              >
                <IconFingerprint /> Authorize and update
              </Button>
            </div>
          )}
        </>
      )}
    </div>
  );
}
