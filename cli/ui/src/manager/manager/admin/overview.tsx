import {
  Description,
  Input,
  Label,
  Link,
  TextArea,
  TextField,
} from "@heroui/react";
import { IconExternalLink } from "@tabler/icons-react";
import { useMemo } from "react";
import { ENCRYPTION_ALGORITHMS, PASSKEY_RESIDENCIES } from "@/constants";
import { useSecretManager } from "@/manager/provider";

export function ProjectOverview() {
  const manager = useSecretManager();

  const residency = useMemo(
    () =>
      PASSKEY_RESIDENCIES.find(
        (res) => res.value === manager.config.project.residency,
      ),
    [manager.config.project.residency],
  );

  const algorithm = useMemo(
    () =>
      ENCRYPTION_ALGORITHMS.find(
        (alg) => alg.value === manager.config.project.algorithm,
      ),
    [manager.config.project.algorithm],
  );

  return (
    <div className="px-4 h-full w-2/3 relative overflow-y-auto">
      <div className="flex justify-between items-start mt-6">
        <div className="text-2xl font-bold">Project Overview</div>
      </div>
      <Description className="mt-1">Basic project details</Description>
      <hr className="my-4" />

      <TextField className={"mt-6"}>
        <Label htmlFor="id">Project Id</Label>
        <Input
          name="id"
          variant="secondary"
          disabled
          spellCheck={false}
          value={manager.config.project.id}
        />
      </TextField>

      <TextField className={"mt-6"}>
        <Label htmlFor="name">Project Name</Label>
        <Input
          disabled
          name="name"
          variant="secondary"
          placeholder="My Top Secret Project"
          value={manager.config.project.name}
        />
      </TextField>

      <TextField className={"mt-6"}>
        <Label htmlFor="description">Description</Label>
        <TextArea
          disabled
          name="description"
          variant="secondary"
          placeholder="A brief description of the project"
          value={manager.config.project.description}
        />
      </TextField>
      <TextField className="mt-6" aria-label="Encryption Algorithm">
        <Label>Encryption Algorithm</Label>
        <Input
          disabled
          name="name"
          variant="secondary"
          placeholder="My Top Secret Project"
          value={algorithm?.label}
        />
        <Description className="font-semibold mb-1">
          {algorithm?.description}
        </Description>
      </TextField>

      <TextField className="mt-6" aria-label="Passkey Residency">
        <Label>Passkey Residency</Label>
        <Input
          disabled
          name="name"
          variant="secondary"
          placeholder="My Top Secret Project"
          value={residency?.label}
        />
        <Description className="font-semibold mb-1">
          {residency?.description}
        </Description>
        <Description>
          <p>
            Passkeys can be stored on the user's device, hardware security key,
            or in synced password managers/keychains.
          </p>
          <ul className="list-disc list-inside mt-1">
            <li>
              Apple Devices -
              <Link
                target="_blank"
                href="https://support.apple.com/en-ae/guide/security/sec59b0b31ff/web"
              >
                The Secure Enclave <IconExternalLink className="size-4" />
              </Link>
            </li>
            <li>
              Windows Linux and other platforms -
              <Link
                target="_blank"
                href="https://learn.microsoft.com/en-us/windows/security/hardware-security/tpm/trusted-platform-module-overview"
              >
                Trusted Platform Module (TPM) chip{" "}
                <IconExternalLink className="size-4" />
              </Link>
            </li>
            <li>
              Hardware Security Keys -{" "}
              <Link target="_blank" href="https://fidoalliance.org/passkeys/">
                Fido2 Protocol Compliant <IconExternalLink className="size-4" />
              </Link>
            </li>
          </ul>
        </Description>
      </TextField>
    </div>
  );
}
