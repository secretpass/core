import { Description, Input, Label, TextArea, TextField } from "@heroui/react";
import type { Stepper } from "@/components/stepper";
import type { ComplexState } from "@/utils.ts";
import { ControlButtons } from "./controls";

export function ProjectOverview({
  state,
  stepper,
}: {
  state: ComplexState<{ id: string; name: string; description: string }>;
  stepper: Stepper;
}) {
  return (
    <div className="flex flex-col gap-6 grow">
      <TextField isDisabled>
        <Label htmlFor="id">Project Id</Label>
        <Description>This value cannot be changed, now or later</Description>
        <Input
          name="id"
          variant="secondary"
          disabled
          spellCheck={false}
          value={state.id}
        />
      </TextField>

      <TextField isRequired>
        <Label htmlFor="name">Project Name</Label>
        <Description>
          If changed later, previous passkeys will still show the previous name
        </Description>
        <Input
          name="name"
          variant="secondary"
          placeholder="My Top Secret Project"
          value={state.name}
          onChange={(e) => state.update({ name: e.target.value })}
        />
      </TextField>

      <TextField>
        <Label htmlFor="description">Description</Label>
        <TextArea
          name="description"
          variant="secondary"
          placeholder="A brief description of the project"
          value={state.description}
          onChange={(e) => state.update({ description: e.target.value })}
        />
      </TextField>

      <ControlButtons
        stepper={stepper}
        next={state.name ? "active" : "disabled"}
        completeOnNext
      />
    </div>
  );
}
