import { Button, Card, CardContent } from "@heroui/react";
import { IconFolder } from "@tabler/icons-react";
import { useState } from "react";
import { CreateProject } from "@/features/setup/project";
import { useSecretManager } from "@/manager";

export enum SetupMode {
  Project = "project",
  User = "user",
}

// This modal is used to set up new projects or user's for existing projects
export function NewProjectSetup({ mode }: { mode?: SetupMode }) {
  const manager = useSecretManager();
  const [setup_mode, setSetupMode] = useState<SetupMode | undefined>(mode);

  return (
    <div className="grow flex flex-col items-center justify-center">
      <Card className={setup_mode ? "w-full max-w-6xl" : "w-full max-w-2xl"}>
        {setup_mode === undefined && (
          <Card.Header className="text-center">
            <Card.Title>Welcome!</Card.Title>
            <Card.Description>
              You're running secretpass locally
            </Card.Description>
          </Card.Header>
        )}
        {setup_mode === SetupMode.User && (
          <Card.Header>
            <Card.Title>Create Passkey</Card.Title>
            <Card.Description>
              Setup a passkey for an existing project
            </Card.Description>
          </Card.Header>
        )}
        {setup_mode === SetupMode.Project && (
          <Card.Header>
            <Card.Title>New Project</Card.Title>
            <Card.Description>
              Setup a new local Secretpass project under{" "}
              <span>{manager.directory}</span>
            </Card.Description>
          </Card.Header>
        )}
        {setup_mode === undefined && (
          <Card.Content className="flex mt-4 gap-8 flex-col md:flex-row">
            <Button
              fullWidth
              size="lg"
              onClick={() => setSetupMode(SetupMode.Project)}
            >
              Setup a new project
            </Button>
            <Button
              fullWidth
              size="lg"
              onClick={() => setSetupMode(SetupMode.User)}
            >
              Create a passkey for a project
            </Button>
          </Card.Content>
        )}
        {setup_mode === SetupMode.Project && (
          <CardContent className="mt-4">
            <CreateProject />
          </CardContent>
        )}

        {setup_mode === undefined && (
          <Card.Footer className="mt-10">
            <div className="text-muted text-sm flex gap-1 items-center justify-center w-full">
              <IconFolder className="size-5" /> <pre>{manager.directory}</pre>
            </div>
          </Card.Footer>
        )}
      </Card>
    </div>
  );
}
