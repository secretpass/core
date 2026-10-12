import { Button, Description } from "@heroui/react";
import { IconUserKey } from "@tabler/icons-react";
import { useState } from "react";
import { useSecretManager } from "@/manager/provider";
import { NewUser } from "./new-user";
import { ViewUser } from "./view-user";

export function UsersDashboard() {
  const manager = useSecretManager();
  const [creating, setCreating] = useState(false);

  return (
    <div className="px-4 h-full w-2/3 relative overflow-y-auto">
      <div className="sticky top-0">
        <div className="flex justify-between items-start mt-6">
          <div className="text-2xl font-bold">Manage Users</div>
          {manager.user.level === "Admin" && (
            <Button variant="secondary" onClick={() => setCreating(true)}>
              <IconUserKey />
              New User / Public Key
            </Button>
          )}
        </div>
        <Description className="mt-1">
          Manage users and customize access
        </Description>
        <hr className="my-4" />
      </div>

      {manager.config.users.map((user) => (
        <ViewUser key={user.id} user={user} />
      ))}

      {creating && <NewUser onDismiss={() => setCreating(false)} />}
    </div>
  );
}
