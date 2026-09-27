import {
  Alert,
  Button,
  Description,
  Input,
  Label,
  Tooltip,
} from "@heroui/react";
import { IconTrashFilled } from "@tabler/icons-react";
import clsx from "clsx";
import { useMemo } from "react";
import * as z from "zod";
import type { Stepper } from "@/components/stepper";
import { ControlButtons } from "@/features/setup/project/controls.tsx";
import type { ComplexState } from "@/utils.ts";

const environmentItemSchema = z.object({
  name: z
    .string()
    .min(1)
    .regex(/^[a-zA-Z_][a-zA-Z0-9_-]*$/, {
      message:
        "Must be a valid environment variable name (letters, numbers, underscores, cannot start with a number).",
    }),
  description: z.string().optional(),
});

const environmentsSchema = z.object({
  environments: z
    .array(environmentItemSchema)
    .min(1)
    .refine(
      (items) => new Set(items.map((item) => item.name)).size === items.length,
    ),
});

export type SetupEnvironments = z.infer<typeof environmentsSchema>;

export function EnvironmentDefinitions({
  stepper,
  state,
}: {
  stepper: Stepper;
  state: ComplexState<SetupEnvironments>;
}) {
  const is_valid = useMemo(() => environmentsSchema.validate(state), [state]);

  return (
    <div className="flex flex-col gap-6 grow">
      <div className="flex flex-col gap-2">
        <Label>Secret Environment</Label>
        <Description>
          Organize your secrets into different environments, environment access
          can be restricted per user/machine.
        </Description>
      </div>

      {state.environments.map((env, i) => (
        <EnvironmentEditor
          key={`env-${
            // biome-ignore lint/suspicious/noArrayIndexKey: Names can be changed during this process
            i
          }`}
          index={i}
          env={env}
          state={state}
        />
      ))}

      <Alert status="success" className="shadow-none">
        <Alert.Indicator />
        <Alert.Content>
          <Alert.Title>
            Environments can be added, edited or deleted accordingly after the
            project setup has been completed.
          </Alert.Title>
        </Alert.Content>
      </Alert>

      <div className="flex justify-end">
        <Button
          aria-label="Add Environment"
          isDisabled={!is_valid && state.environments.length > 0}
          variant="ghost"
          onClick={() =>
            state.complexUpdate((nextState) => {
              nextState.environments.push({ name: "", description: undefined });
              nextState.environments = [...nextState.environments];
            })
          }
        >
          Add Environment
        </Button>
      </div>

      <ControlButtons
        stepper={stepper}
        previous="active"
        next={is_valid ? "active" : "disabled"}
        completeOnNext
      />
    </div>
  );
}

function EnvironmentEditor({
  env,
  index,
  state,
}: {
  index: number;
  env: SetupEnvironments["environments"][number];
  state: ComplexState<SetupEnvironments>;
}) {
  const is_valid = useMemo(() => {
    if (!environmentItemSchema.validate(env)) {
      return false;
    }
    const duplicates = state.environments.filter((e) => e.name === env.name);
    return duplicates.length === 1;
  }, [env, state]);
  return (
    <div
      className={clsx(
        "p-2 rounded-lg flex flex-col gap-2 bg-gray-300 dark:bg-gray-700",
        is_valid ? "" : "border border-danger",
      )}
    >
      <div className="flex gap-2 items-center">
        <Input
          aria-label="Environment name"
          placeholder="Env name"
          className="grow"
          value={env.name}
          onChange={(e) => {
            state.complexUpdate((state) => {
              state.environments[index].name = e.target.value;
            });
          }}
        />
        <Tooltip>
          <Button
            aria-label="Delete Environment"
            variant="ghost"
            className="text-muted hover:text-danger"
            isIconOnly
            onClick={() => {
              state.complexUpdate((nextState) => {
                nextState.environments.splice(index, 1);
              });
            }}
          >
            <IconTrashFilled />
          </Button>
          <Tooltip.Content>Delete {env.name || "environment"}</Tooltip.Content>
        </Tooltip>
      </div>
      <Input
        aria-label="Environment description"
        placeholder="Description"
        variant="secondary"
        value={env.description || ""}
        onChange={(e) => {
          state.complexUpdate((nextState) => {
            nextState.environments[index].description = e.target.value
              ? e.target.value
              : undefined;
          });
        }}
      />
    </div>
  );
}
