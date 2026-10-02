import {
  Alert,
  Button,
  Description,
  Input,
  Label,
  TextField,
} from "@heroui/react";
import { IconFingerprint } from "@tabler/icons-react";
import { useCallback, useMemo, useState } from "react";
import * as z from "zod";
import { LoadingCircle } from "@/components/loader";
import {
  type ProcessUserLoginResponse,
  process_user_login,
  type SecretpassProject,
  type UserLockPackage,
} from "@/core/web";
import type { SecretMangerState } from "@/manager";
import { makeRequest } from "@/utils";

const email_schema = z.email();

export function LoginScreen({
  project,
  onAuthComplete,
}: {
  project: SecretpassProject;
  onAuthComplete: (state: SecretMangerState) => void;
}) {
  const [username, setUsername] = useState("");
  const [is_loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const is_valid = useMemo(() => email_schema.validate(username), [username]);

  const completeLogin = useCallback(() => {
    setError(null);
    const url = new URL("/api/user/lock-package", window.location.origin);
    url.search = new URLSearchParams({ username }).toString();

    setLoading(true);
    makeRequest<UserLockPackage>(url)
      .then((response) => {
        if (response.ok) {
          return response;
        }
        throw response.text;
      })
      .then(async (response) => {
        const directory = response.headers.get(
          "x-current-working-directory",
        ) as string;
        const lock_package = response.json();
        const result: ProcessUserLoginResponse = JSON.parse(
          await process_user_login(JSON.stringify(lock_package)),
        );

        onAuthComplete({
          user: lock_package.user,
          config: result.config,
          directory,
          public_key: result.public_key,
          public_keys: result.public_keys,
        });
      })
      .catch((err) => setError(String(err)))
      .finally(() => setLoading(false));
  }, [username, onAuthComplete]);

  return (
    <div className="flex flex-col gap-8 grow">
      <TextField>
        <Label htmlFor="id">Email address</Label>
        <Description>
          Your email address as registered under {project.name}
        </Description>
        <Input
          name="username"
          variant="secondary"
          spellCheck={false}
          type="email"
          value={username}
          onChange={(evt) => setUsername(evt.currentTarget.value)}
        />
      </TextField>

      {error && (
        <Alert status="danger">
          <Alert.Indicator />
          <Alert.Content>
            <Alert.Title>Error attempting to login</Alert.Title>
            <Alert.Description>{error}</Alert.Description>
          </Alert.Content>
        </Alert>
      )}

      <div className="flex justify-end">
        <Button
          isDisabled={is_loading || !is_valid}
          aria-label="Register Passkey"
          size="lg"
          onClick={completeLogin}
        >
          {!is_loading && <IconFingerprint />}
          {is_loading && (
            <LoadingCircle aria-label="Loading" size="sm" isIndeterminate />
          )}
          Authorize and Login
        </Button>
      </div>
    </div>
  );
}
