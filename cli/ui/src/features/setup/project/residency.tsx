import {
  Alert,
  Description,
  Label,
  Link,
  Radio,
  RadioGroup,
} from "@heroui/react";
import type { Stepper } from "@/components/stepper";
import type { PasskeyResidency } from "@/core/web";
import type { ComplexState } from "@/utils.ts";
import { ControlButtons } from "./controls";

const RESIDENCIES: {
  value: PasskeyResidency;
  label: string;
  description: string;
}[] = [
  {
    value: "OnDevice",
    label: "On Device",
    description:
      "Pick this option for a good balance between security and support. Passkeys can be used on most devices but won't be synced across devices.",
  },
  {
    value: "SyncedAllowed",
    label: "Sync able Passkeys",
    description:
      "This option is the most widely supported and allows passkeys that can be synced across devices while still supporting on device passkeys. Pick this option unless you have complex security requirements.",
  },
  {
    value: "HardwareKey",
    label: "Hardware Key",
    description:
      "Pick this option if you need absolute control and your organization already has established processes around hardware key security.",
  },
];

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

        {RESIDENCIES.map((def) => (
          <Radio
            className="p-2 rounded-lg flex flex-col gap-2 bg-gray-200 data-[selected=true]:bg-gray-300 dark:bg-gray-700 dark:data-[selected=true]:bg-gray-800"
            value={def.value}
            aria-label={def.label}
            key={def.value}
          >
            <Radio.Content aria-label="On Device">
              <Radio.Control aria-label="On Device">
                <Radio.Indicator aria-label="On Device" />
              </Radio.Control>
              {def.label}
            </Radio.Content>
            <Description>{def.description}</Description>
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
