import { Button, Description, Input, Label, TextField } from "@heroui/react";
import { IconFingerprint } from "@tabler/icons-react";
import { useCallback, useMemo, useState } from "react";
import * as z from "zod";
import { ErrorDisplay, useError } from "@/components";
import { LoadingCircle } from "@/components/loader";
import {
  authorize_operation,
  type LoginParams,
  type SecretpassProject,
} from "@/core/web";
import { resolveRequest } from "@/utils";
import { useAnalytics } from "../analytics/provider";

const email_schema = z.email();

export function LoginScreen({
  project,
  onAuthComplete,
}: {
  project: SecretpassProject;
  onAuthComplete: (public_key_id: string) => void;
}) {
  const analytics = useAnalytics();
  const [username, setUsername] = useState("");
  const error = useError();

  const is_valid = useMemo(() => email_schema.validate(username), [username]);

  const completeLogin = useCallback(async () => {
    const url = new URL("/api/user/login-params", window.location.origin);
    url.search = new URLSearchParams({ username }).toString();

    const login_details_result = await error.catch(
      "Error fetching login details",
      () => resolveRequest<LoginParams>(url),
      "fetch_login_details",
    );

    if (!login_details_result.ok) {
      return;
    }

    const lock_package = login_details_result.result;
    const auth_result = await error.catch(
      "Error authorizing login",
      () => authorize_operation(JSON.stringify(lock_package)),
      "authorize_login",
    );
    if (!auth_result.ok) {
      return;
    }

    analytics?.identify(lock_package.user.id, {
      email: lock_package.user.username,
      name: lock_package.user.name,
      level: lock_package.user.level,
    });

    analytics?.capture("user_logged_in");

    onAuthComplete(auth_result.result);
  }, [username, onAuthComplete, analytics, error.catch]);

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

      <ErrorDisplay value={error.value} />

      <div className="flex justify-end">
        <Button
          isDisabled={error.is_pending || !is_valid}
          aria-label="Register Passkey"
          size="lg"
          onClick={completeLogin}
        >
          {!error.is_pending && <IconFingerprint />}
          {error.is_pending && (
            <LoadingCircle aria-label="Loading" size="sm" isIndeterminate />
          )}
          Authorize and Login
        </Button>
      </div>
    </div>
  );
}
