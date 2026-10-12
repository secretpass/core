import {
  Button,
  Description,
  Input,
  Label,
  TextArea,
  TextField,
  toast,
} from "@heroui/react";
import { IconFingerprint } from "@tabler/icons-react";
import { useCallback, useMemo, useState } from "react";
import * as z from "zod";
import { EnvironmentSelector, Select } from "@/components/select";
import {
  authorize_operation,
  generate_registration_config,
  type RegistrationParams,
  type UserLevel,
} from "@/core/web";
import { useSecretManager } from "@/manager/provider";
import { useComplexState } from "@/utils";

interface GenerationState {
  name: string;
  level: UserLevel;
  username: string;
  environments: string[];
}

const emailSchema = z.email();

export function StartRegistration() {
  const manager = useSecretManager();
  const [config, setConfig] = useState<string | null>(null);
  const new_user = useComplexState<GenerationState>(() => ({
    name: "",
    username: "",
    level: "Standard",
    environments: [],
  }));

  const [existing_user, exclude_keys] = useMemo(() => {
    const user = manager.config.users.find(
      (u) => u.username?.toLowerCase() === new_user.username.toLowerCase(),
    );
    if (!user) {
      return [null, []];
    }
    const keys = manager.public_keys
      .filter((pk) => pk.passkey?.user_id === user.id)
      .map((pk) => pk.passkey?.id as string);
    return [user, keys];
  }, [manager.config.users, new_user.username, manager.public_keys]);

  const is_valid = useMemo(
    () => emailSchema.validate(new_user.username),
    [new_user.username],
  );

  const generate = useCallback(async () => {
    const reg_params: RegistrationParams = {
      user: {
        id: existing_user?.id ?? crypto.randomUUID(),
        name: existing_user?.name ?? new_user.name,
        username: existing_user?.username ?? new_user.username,
        level: existing_user?.level ?? new_user.level,
        access: existing_user?.access ?? new_user.environments,
        created_at: existing_user?.created_at ?? new Date().toISOString(),
      },
      project: manager.config.project,
      exclude_keys,
    };

    try {
      await authorize_operation(JSON.stringify(manager.login_params));
      setConfig(generate_registration_config(JSON.stringify(reg_params)));
    } catch (err) {
      toast(String(err), { variant: "danger" });
    }
  }, [
    manager.config.project,
    existing_user,
    exclude_keys,
    new_user,
    manager.login_params,
  ]);

  return (
    <div className="flex flex-col gap-6">
      <TextField className="mt-4" isDisabled={!!config}>
        <Label>Email Address</Label>
        <Input
          placeholder="user@email.here"
          value={new_user.username}
          onChange={(evt) =>
            new_user.update({ username: evt.currentTarget.value })
          }
        />
        <Description>
          Enter an email of a new or existing user to generate a key setup
          config for a new or existing user.
        </Description>
      </TextField>

      <TextField isDisabled={!!existing_user || !!config}>
        <Label>Name</Label>
        <Input
          placeholder="Name..."
          value={existing_user?.name ?? new_user.name}
          onChange={(evt) => new_user.update({ name: evt.currentTarget.value })}
        />
      </TextField>

      <Select<UserLevel>
        label="Access Level"
        disabled={!!existing_user || !!config}
        value={existing_user?.level ?? new_user.level}
        onChange={(level) => new_user.update({ level })}
        items={[
          { label: "Admin", value: "Admin" },
          { label: "Standard", value: "Standard" },
        ]}
      />

      {(existing_user?.level ?? new_user.level) === "Standard" && (
        <EnvironmentSelector<string[]>
          multiselect
          label="Environment"
          description="The environments this user is allowed to access."
          disabled={!!existing_user || !!config}
          value={new_user.environments}
          onChange={(environments) => new_user.update({ environments })}
        />
      )}

      {!!config && (
        <TextField>
          <Label className="text-success">Public Key Generation Config</Label>
          <Description className="text-success">
            Share this key with {existing_user?.username ?? new_user.username},
            they'll use it to register a new passkey on the login screen and
            share the key with you
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
            readOnly
            onFocus={(evt) => evt.currentTarget.select()}
            value={config}
            className="field-sizing-content resize-none"
          />
        </TextField>
      )}

      {!config && (
        <div className="flex justify-end">
          <Button
            isDisabled={!is_valid || (!new_user.name && !existing_user)}
            onClick={generate}
          >
            <IconFingerprint />
            Generate Config
          </Button>
        </div>
      )}
    </div>
  );
}
