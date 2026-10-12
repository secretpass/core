import type { PasskeyResidency } from "@/core/web";

export const PASSKEY_RESIDENCIES: {
  value: PasskeyResidency;
  label: string;
  description: string;
}[] = [
  {
    value: "OnDevice",
    label: "On Device",
    description:
      "Pick this option for a good balance between security and support. Passkeys can be used on most devices but won't be synced across devices.",
  },
  {
    value: "SyncedAllowed",
    label: "Sync able Passkeys",
    description:
      "This option is the most widely supported and allows passkeys that can be synced across devices while still supporting on device passkeys. Pick this option unless you have complex security requirements.",
  },
  {
    value: "HardwareKey",
    label: "Hardware Key",
    description:
      "Pick this option if you need absolute control and your organization already has established processes around hardware key security.",
  },
];
