import { Button, Card, CardContent } from "@heroui/react";
import { IconArrowLeft, IconFolder } from "@tabler/icons-react";
import { useState } from "react";
import { CreateProject } from "@/features/setup/project";
import { CreatePasskeyScreen } from "../auth/create-passkey";

export enum ActiveView {
  ProjectSetup = "project-setup",
  UserSetup = "user-setup",
  Dashboard = "dashboard",
}

// This modal is used to set up new projects or user's for existing projects
export function NewProjectSetup({
  onAuthComplete,
  directory,
}: {
  onAuthComplete: (public_key_id: string) => void;
  directory: string;
}) {
  const [active_view, setActiveView] = useState<ActiveView>(
    ActiveView.Dashboard,
  );

  return (
    <div className="content flex flex-col items-center justify-center">
      <Card className="w-full max-w-6xl">
        {active_view === ActiveView.Dashboard && (
          <Card.Header className="text-center mb-8">
            <Card.Title>Welcome!</Card.Title>
            <Card.Description>
              You're running secretpass locally
            </Card.Description>
          </Card.Header>
        )}
        {active_view === ActiveView.UserSetup && (
          <Card.Header>
            <Card.Title className="flex gap-2 items-center">
              <Button
                isIconOnly
                size="sm"
                variant="ghost"
                onClick={() => setActiveView(ActiveView.Dashboard)}
              >
                <IconArrowLeft />
              </Button>{" "}
              Create Passkey
            </Card.Title>
            <Card.Description>
              Setup a passkey for an existing project
            </Card.Description>
          </Card.Header>
        )}
        {active_view === ActiveView.ProjectSetup && (
          <Card.Header>
            <Card.Title className="flex gap-2 items-center">
              <Button
                isIconOnly
                size="sm"
                variant="ghost"
                onClick={() => setActiveView(ActiveView.Dashboard)}
              >
                <IconArrowLeft />
              </Button>{" "}
              New Project
            </Card.Title>
            <Card.Description>
              Setup a new local Secretpass project under{" "}
              <span>{directory}</span>
            </Card.Description>
          </Card.Header>
        )}
        {active_view === ActiveView.Dashboard && (
          <Card.Content className="flex mt-4 gap-8 flex-col md:flex-row">
            <Button
              fullWidth
              size="lg"
              onClick={() => setActiveView(ActiveView.ProjectSetup)}
            >
              Setup a new project
            </Button>
            <Button
              fullWidth
              size="lg"
              onClick={() => setActiveView(ActiveView.UserSetup)}
            >
              Create a passkey for a project
            </Button>
          </Card.Content>
        )}
        {active_view === ActiveView.ProjectSetup && (
          <CardContent className="mt-4">
            <CreateProject
              onAuthComplete={onAuthComplete}
              directory={directory}
            />
          </CardContent>
        )}
        {active_view === ActiveView.UserSetup && (
          <CardContent className="mt-4">
            <CreatePasskeyScreen />
          </CardContent>
        )}

        {active_view === ActiveView.Dashboard && (
          <Card.Footer className="mt-10">
            <div className="text-muted text-sm flex gap-1 items-center justify-center w-full">
              <IconFolder className="size-5" /> <pre>{directory}</pre>
            </div>
          </Card.Footer>
        )}
      </Card>
    </div>
  );
}
