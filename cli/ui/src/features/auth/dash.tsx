import { Button, Card } from "@heroui/react";
import { IconArrowLeft } from "@tabler/icons-react";
import { useState } from "react";
import type { SecretpassProject } from "@/core/web";
import type { SecretMangerState } from "@/manager";
import { LoginScreen } from "./login";

export enum ActiveView {
  ProjectLogin = "project-login",
  UserSetup = "user-setup",
  Dashboard = "dashboard",
}

export function LoginDashboard({
  project,
  directory,
  onAuthComplete,
}: {
  directory: string;
  project: SecretpassProject;
  onAuthComplete: (state: SecretMangerState) => void;
}) {
  const [active_view, setActiveView] = useState<ActiveView>(
    ActiveView.Dashboard,
  );

  return (
    <div className="grow flex flex-col items-center justify-center">
      <Card className={"w-full max-w-xl"}>
        {active_view === ActiveView.Dashboard && (
          <Card.Header className="text-center">
            <Card.Title>Welcome!</Card.Title>
            <Card.Description>
              You're running secretpass locally
            </Card.Description>
          </Card.Header>
        )}
        {active_view === ActiveView.UserSetup && (
          <Card.Header>
            <Card.Title>Create Passkey</Card.Title>
            <Card.Description>
              Setup a passkey for an existing project
            </Card.Description>
          </Card.Header>
        )}
        {active_view === ActiveView.ProjectLogin && (
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
              Login
            </Card.Title>
            <Card.Description>
              Login to local Secretpass project{" "}
              <span className="font-semibold">{project.name}</span> under{" "}
              <span className="font-mono">{directory}</span>
            </Card.Description>
          </Card.Header>
        )}
        {active_view === ActiveView.Dashboard && (
          <Card.Content className="flex mt-4 gap-8 flex-col md:flex-row">
            <Button
              fullWidth
              size="lg"
              onClick={() => setActiveView(ActiveView.ProjectLogin)}
            >
              Login to {project.name}
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
        {active_view === ActiveView.ProjectLogin && (
          <Card.Content className="mt-4">
            <LoginScreen project={project} onAuthComplete={onAuthComplete} />
          </Card.Content>
        )}
      </Card>
    </div>
  );
}
