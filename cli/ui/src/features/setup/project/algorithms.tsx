import { Alert, Description, Label, Radio, RadioGroup } from "@heroui/react";
import type { Stepper } from "@/components/stepper";
import type { EncryptionAlgorithm } from "@/core/web";
import type { ComplexState } from "@/utils.ts";
import { ControlButtons } from "./controls";

const ALGORITHMS: {
  value: EncryptionAlgorithm;
  label: string;
  description: string;
}[] = [
  {
    value: "ECC",
    label: "Elliptic-curve Cryptography (ECC - X25519)",
    description:
      'Fast and secure key exchange algorithm, it\'s not considered "safe" against future quantum computer attacks.',
  },
  {
    value: "KEM",
    label: "Key Encapsulation Mechanism (KEM - Kyber)",
    description:
      "Post-quantum attack safe; has relatively larger keys and cipher outputs but relatively fast computation in comparison to X25519.",
  },
  {
    value: "Hybrid",
    label: "Hybrid (ECC + KEM)",
    description:
      "A combination of these 2 algorithms ensuring safety if either of the algorithms is ever compromised.",
  },
];

export function EncryptionAlgorithmSelector({
  stepper,
  state,
}: {
  stepper: Stepper;
  state: ComplexState<{ algorithm: EncryptionAlgorithm }>;
}) {
  return (
    <div className="flex flex-col gap-6">
      <RadioGroup
        isRequired
        value={state.algorithm}
        onChange={(value) =>
          state.update({ algorithm: value as EncryptionAlgorithm })
        }
        aria-label="Encryption Algorithm"
      >
        <Label>Encryption Algorithm</Label>
        <Description>The algorithm used to secure your secrets.</Description>

        {ALGORITHMS.map((def) => (
          <Radio
            className="p-2 rounded-lg flex flex-col gap-2 bg-gray-200 data-[selected=true]:bg-gray-300 dark:bg-gray-700 dark:data-[selected=true]:bg-gray-800"
            value={def.value}
            aria-label={def.label}
            key={def.value}
          >
            <Radio.Content aria-label={def.label}>
              <Radio.Control aria-label={def.label}>
                <Radio.Indicator aria-label={def.label} />
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
