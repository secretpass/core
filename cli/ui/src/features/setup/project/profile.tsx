import {
  Alert,
  Button,
  Description,
  Input,
  Label,
  TextField,
} from "@heroui/react";
import { IconFingerprint } from "@tabler/icons-react";
import { type MouseEventHandler, useMemo } from "react";
import * as z from "zod";
import type { Stepper } from "@/components/stepper";
import type { RegistrationState } from "@/features/setup/project/types.ts";
import type { ComplexState } from "@/utils.ts";
import { ControlButtons } from "./controls";

const profileSchema = z.object({
  user_id: z.uuidv4(),
  user_display_name: z.string().min(1),
  user_email_address: z.email(),
});

export function AdminProfile({
  state,
  stepper,
  onComplete,
  error,
}: {
  error: string | null;
  state: ComplexState<RegistrationState>;
  stepper: Stepper;
  onComplete: MouseEventHandler;
}) {
  const is_valid = useMemo(() => profileSchema.validate(state), [state]);
  return (
    <div className="flex flex-col gap-6 grow">
      <TextField isDisabled>
        <Label htmlFor="user_id">User Id</Label>
        <Input
          name="user_id"
          variant="secondary"
          disabled
          spellCheck={false}
          value={state.user_id}
        />
      </TextField>

      <TextField isRequired>
        <Label htmlFor="user_name">Full Name</Label>
        <Description>
          Your full name, as the admin you'll have access to all environments
          and secrets
        </Description>
        <Input
          name="user_name"
          variant="secondary"
          placeholder="Full Name..."
          value={state.user_display_name}
          onChange={(e) => state.update({ user_display_name: e.target.value })}
        />
      </TextField>

      <TextField>
        <Label htmlFor="email_address">Email Address</Label>
        <Input
          name="email_address"
          variant="secondary"
          placeholder="Email Address..."
          value={state.user_email_address}
          onChange={(e) => state.update({ user_email_address: e.target.value })}
        />
      </TextField>

      {error && (
        <Alert status="danger">
          <Alert.Indicator />
          <Alert.Content>
            <Alert.Title>Error registering passkey</Alert.Title>
            <Alert.Description>{error}</Alert.Description>
          </Alert.Content>
        </Alert>
      )}

      <ControlButtons
        stepper={stepper}
        previous="active"
        completeOnNext
        completeButton={
          <Button
            aria-label="Register Passkey"
            size="lg"
            isDisabled={!is_valid}
            onClick={onComplete}
          >
            <IconFingerprint />
            Register Passkey
          </Button>
        }
      />
    </div>
  );
}
