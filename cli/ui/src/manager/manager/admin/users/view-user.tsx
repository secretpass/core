import { Chip, Description } from "@heroui/react";
import {
  IconBotId,
  IconShield,
  IconUser,
  IconUserShield,
} from "@tabler/icons-react";
import clsx from "clsx";
import type { SecretpassUser, UserLevel } from "@/core/web";

function UserIcon({
  level,
  className,
}: {
  level: UserLevel;
  className?: string;
}) {
  return (
    <>
      {level === "Standard" && <IconUser className={className} />}
      {level === "Admin" && <IconUserShield className={className} />}
      {level === "Machine" && <IconBotId className={className} />}
    </>
  );
}

export function ViewUser({ user }: { user: SecretpassUser }) {
  return (
    <div
      className={clsx(
        "flex items-center gap-4 px-4 pb-4 pt-2 my-4 rounded-2xl bg-background",
      )}
    >
      <UserIcon level={user.level} className="size-8 text-muted" />

      <div className="grow flex flex-col gap-1">
        <div className="flex items-center gap-4 mb-2">
          <span className="grow text-base">{user.name}</span>
          {user.level === "Admin" && (
            <Chip color="accent">
              <IconShield className="size-4" /> {user.level}
            </Chip>
          )}
          {user.level !== "Admin" && (
            <div className="flex gap-2 flex-wrap">
              {user.access?.map((env) => (
                <Chip key={env} color="accent">
                  {env}
                </Chip>
              ))}
            </div>
          )}
        </div>
        <div className="flex justify-between">
          <Description>{user.username ?? "bot"}</Description>
          <Description>{user.id}</Description>
        </div>
      </div>
    </div>
  );
}
