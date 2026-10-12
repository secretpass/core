import {
  Alert,
  Description,
  Label,
  Link,
  Radio,
  RadioGroup,
} from "@heroui/react";
import type { Stepper } from "@/components/stepper";
import { PASSKEY_RESIDENCIES } from "@/constants";
import type { PasskeyResidency } from "@/core/web";
import type { ComplexState } from "@/utils.ts";
import { ControlButtons } from "./controls";

export function PasskeyResidencySelector({
  stepper,
  state,
}: {
  stepper: Stepper;
  state: ComplexState<{ residency: PasskeyResidency }>;
}) {
  return (
    <div className="flex flex-col gap-6">
      <RadioGroup
        isRequired
        value={state.residency}
        onChange={(value) =>
          state.update({ residency: value as PasskeyResidency })
        }
        aria-label="Passkey Residency"
      >
        <Label>Passkey Residency</Label>
        <Description>
          <p>
            Passkeys can be stored on the user's device, hardware security key,
            or in synced password managers/keychains.
          </p>
          <ul className="list-disc list-inside mt-1">
            <li>
              Apple Devices -
              <Link href="https://support.apple.com/en-ae/guide/security/sec59b0b31ff/web">
                The Secure Enclave
              </Link>
            </li>
            <li>
              Windows Linux and other platforms -
              <Link>Trusted Platform Module (TPM) chip</Link>
            </li>
            <li>
              Hardware Security Keys -{" "}
              <Link href="https://fidoalliance.org/passkeys/">
                Fido2 Protocol Compliant
              </Link>
            </li>
          </ul>
        </Description>

        {PASSKEY_RESIDENCIES.map((def) => (
          <Radio
            className="p-2 rounded-lg flex flex-col gap-2 bg-gray-200 data-[selected=true]:bg-gray-300 dark:bg-gray-700 dark:data-[selected=true]:bg-gray-800"
            value={def.value}
            aria-label={def.label}
            key={def.value}
          >
            <Radio.Content aria-label="On Device">
              <Radio.Control className="mx-2" aria-label="On Device">
                <Radio.Indicator aria-label="On Device" />
              </Radio.Control>
              <div className="flex flex-col gap-0.5">
                {def.label}
                <Description>{def.description}</Description>
              </div>
            </Radio.Content>
          </Radio>
        ))}
      </RadioGroup>

      <Alert status="warning" className="shadow-none">
        <Alert.Indicator />
        <Alert.Content>
          <Alert.Title>
            This value cannot be changed once the project is setup has been
            completed.
          </Alert.Title>
        </Alert.Content>
      </Alert>
      <ControlButtons
        stepper={stepper}
        next="active"
        previous="active"
        completeOnNext
      />
    </div>
  );
}
