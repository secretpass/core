import {
  Button,
  IconPlus,
  Input,
  Label,
  TextArea,
  TextField,
} from "@heroui/react";
import { IconFingerprint, IconX } from "@tabler/icons-react";
import clsx from "clsx";
import { useCallback, useMemo, useState } from "react";
import z from "zod";
import { SAFE_NAME_SCHEMA } from "@/constants";
import type { SecretpassEnvironment, StoredPublicKey } from "@/core/web";
import { useCreateOrUpdateSecret } from "./shared";

interface NewSecretProps {
  onComplete: () => void;
  environment: SecretpassEnvironment;
  public_keys: StoredPublicKey[];
}

const newSecretSchema = z.object({
  name: SAFE_NAME_SCHEMA,
  value: z.string().min(1),
});

export function NewSecret({
  onComplete,
  environment,
  public_keys,
}: NewSecretProps) {
  const [is_active, setIsActive] = useState(false);

  const [name, setSecretName] = useState<string>("");
  const [description, setSecretDescription] = useState<string>("");
  const [value, setSecretValue] = useState<string>("");

  const is_valid = useMemo(
    () => newSecretSchema.validate({ name, value }),
    [name, value],
  );

  const name_is_valid = useMemo(() => SAFE_NAME_SCHEMA.validate(name), [name]);

  const reset = useCallback(() => {
    setIsActive(false);
    setSecretValue("");
    setSecretName("");
    setSecretDescription("");
  }, []);

  const createSecret = useCreateOrUpdateSecret(environment, public_keys, () => {
    reset();
    onComplete();
  });

  return (
    <div
      className={clsx(
        "sticky bottom-0 flex transition-all py-2 px-4 duration-500 mt-6",
        !is_active && "justify-end bg-surface",
        is_active &&
          "flex-col gap-4 shadow-lg rounded-2xl bg-background-tertiary",
      )}
    >
      {is_active && (
        <>
          <div className="text-xl flex justify-between items-center">
            Create Secret
            <Button isIconOnly variant="ghost" onClick={() => reset()}>
              <IconX className="size-6 hover:text-danger" />
            </Button>
          </div>
          <TextField isInvalid={!!name && !name_is_valid}>
            <Label>Secret name</Label>
            <Input
              placeholder="MY-SECRET..."
              value={name}
              onChange={(evt) => setSecretName(evt.currentTarget.value)}
            />
          </TextField>
          <TextField>
            <Label>Description</Label>
            <Input
              placeholder="Help other's identify the purpose of this secret"
              value={description}
              onChange={(evt) => setSecretDescription(evt.currentTarget.value)}
            />
          </TextField>
          <TextField>
            <Label>Secret value</Label>
            <TextArea
              placeholder="Top secret information here..."
              value={value}
              onChange={(evt) => setSecretValue(evt.currentTarget.value)}
            />
          </TextField>
          <div className="flex justify-end my-2">
            <Button
              variant="primary"
              isDisabled={!is_valid}
              onClick={() => createSecret(name, description, value)}
            >
              <IconFingerprint /> Authorize and create
            </Button>
          </div>
        </>
      )}
      {!is_active && (
        <Button variant="secondary" onClick={() => setIsActive(true)}>
          <IconPlus /> Add Secret
        </Button>
      )}
    </div>
  );
}
