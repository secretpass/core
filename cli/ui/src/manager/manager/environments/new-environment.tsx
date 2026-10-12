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
import { ErrorDisplay, useError } from "@/components";
import { SAFE_NAME_SCHEMA } from "@/constants";
import { authorize_operation, type LoginParams } from "@/core/web";
import { useSecretManager } from "@/manager/provider";
import type { SecretManagerConfig } from "@/types";
import { resolveRequest } from "@/utils";

export function CreateEnvironment({
  setActiveEnvironment,
}: {
  setActiveEnvironment: (env: string) => void;
}) {
  const manager = useSecretManager();
  const [active, setActive] = useState(false);
  const [name, setName] = useState("");
  const [description, setDescription] = useState("");
  const secret_names = useMemo(
    () => manager.config.environments.map((env) => env.name.toLowerCase()),
    [manager.config.environments],
  );
  const error = useError();

  const is_valid = useMemo(
    () =>
      SAFE_NAME_SCHEMA.validate(name) &&
      !secret_names.includes(name.toLowerCase()),
    [name, secret_names],
  );

  const reset = useCallback(() => {
    setActive(false);
    setName("");
    setDescription("");
  }, []);
  const createEnvironment = useCallback(async () => {
    const config: SecretManagerConfig = JSON.parse(
      JSON.stringify(manager.config),
    );

    config.environments.push({
      name,
      description,
    });

    const params: LoginParams = {
      user: manager.user,
      project: config.project,
      public_keys: [manager.public_key],
    };
    const auth_result = await error.catch("Authorizing error", () =>
      authorize_operation(JSON.stringify(params)),
    );
    if (!auth_result.ok) {
      return;
    }

    const update_result = await error.catch("Error updating config", () =>
      resolveRequest("/api/project", {
        method: "PATCH",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ config }),
      }),
    );

    if (!update_result.ok) {
      return;
    }

    await manager.refreshState(`Environment ${name} created successfully`);

    setActiveEnvironment(name);
    reset();
  }, [manager, description, name, reset, setActiveEnvironment, error.catch]);

  return (
    <div
      className={clsx(
        "flex flex-col gap-4 py-2 mt-6 rounded-2xl",
        active && "bg-background-tertiary px-4",
        !active && "bg-background",
      )}
    >
      {active && (
        <>
          <div className="text-xl flex justify-between items-center">
            Create Environment
            <Button isIconOnly variant="ghost" onClick={() => reset()}>
              <IconX className="size-6 hover:text-danger" />
            </Button>
          </div>
          <TextField isInvalid={!!name && !is_valid}>
            <Label>Secret name</Label>
            <Input
              placeholder="MY-ENV..."
              value={name}
              onChange={(evt) => setName(evt.currentTarget.value)}
            />
          </TextField>
          <TextField>
            <Label>Description</Label>
            <TextArea
              placeholder="Help other's identify the purpose of this environment"
              value={description}
              onChange={(evt) => setDescription(evt.currentTarget.value)}
            />
          </TextField>
          <Button isDisabled={!is_valid} fullWidth onClick={createEnvironment}>
            <IconFingerprint /> Authorize and Create
          </Button>
        </>
      )}

      <ErrorDisplay value={error.value} />

      {!active && (
        <Button fullWidth variant="tertiary" onClick={() => setActive(true)}>
          <IconPlus /> New Environment
        </Button>
      )}
    </div>
  );
}
