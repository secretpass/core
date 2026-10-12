import { Alert, Description, Label, Radio, RadioGroup } from "@heroui/react";
import type { Stepper } from "@/components/stepper";
import { ENCRYPTION_ALGORITHMS } from "@/constants";
import type { EncryptionAlgorithm } from "@/core/web";
import type { ComplexState } from "@/utils.ts";
import { ControlButtons } from "./controls";

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

        {ENCRYPTION_ALGORITHMS.map((def) => (
          <Radio
            className="p-2 rounded-lg flex flex-col gap-2 bg-gray-200 data-[selected=true]:bg-gray-300 dark:bg-gray-700 dark:data-[selected=true]:bg-gray-800"
            value={def.value}
            aria-label={def.label}
            key={def.value}
            id={def.value}
          >
            <Radio.Content aria-label={def.label}>
              <Radio.Control className="mx-2" aria-label={def.label}>
                <Radio.Indicator aria-label={def.label} />
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
