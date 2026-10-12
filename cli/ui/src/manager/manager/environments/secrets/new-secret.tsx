import { Button, Input, Label, TextArea, TextField } from "@heroui/react";
import { IconFingerprint, IconX } from "@tabler/icons-react";
import clsx from "clsx";
import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import z from "zod";
import { SAFE_NAME_SCHEMA } from "@/constants";
import type { SecretpassEnvironment } from "@/core/web";
import { useCreateOrUpdateSecret } from "./shared";

interface NewSecretProps {
  environment: SecretpassEnvironment;
  onDismiss: () => void;
}

const newSecretSchema = z.object({
  name: SAFE_NAME_SCHEMA,
  value: z.string().min(1),
});

export function NewSecret({ environment, onDismiss }: NewSecretProps) {
  const [name, setSecretName] = useState<string>("");
  const [description, setSecretDescription] = useState<string>("");
  const [value, setSecretValue] = useState<string>("");

  const is_valid = useMemo(
    () => newSecretSchema.validate({ name, value }),
    [name, value],
  );

  const name_is_valid = useMemo(() => SAFE_NAME_SCHEMA.validate(name), [name]);

  const reset = useCallback(() => {
    setSecretValue("");
    setSecretName("");
    setSecretDescription("");
  }, []);

  const ref = useRef<HTMLDivElement>(null);
  useEffect(() => {
    ref.current?.scrollIntoView();
  }, []);

  const createSecret = useCreateOrUpdateSecret(environment);

  return (
    <div
      ref={ref}
      className={clsx(
        "flex py-2 px-4 my-6 flex-col gap-4 rounded-2xl bg-background-tertiary",
      )}
    >
      <div className="text-xl flex justify-between items-center">
        Create Secret
        <Button
          isIconOnly
          variant="ghost"
          className="text-muted hover:text-danger"
          onClick={onDismiss}
        >
          <IconX className="size-4" />
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
          onClick={() =>
            createSecret(name, description, value).then(reset).then(onDismiss)
          }
        >
          <IconFingerprint /> Authorize and create
        </Button>
      </div>
    </div>
  );
}
