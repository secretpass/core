import { create } from "zustand";
import type { SecureSession } from "@/core/web";
import type { SecretManagerConfig } from "@/types";

export type SecretManger = {
  directory: string | null;
  config: SecretManagerConfig | null;
  session: SecureSession | null;
};

interface SecretMangerExtended extends SecretManger {
  update(state: Partial<SecretManger>): void;
  fetch(): Promise<void>;
}

export const useSecretManager = create<SecretMangerExtended>((set) => ({
  directory: null,
  config: null,
  session: null,
  update(state) {
    set(state);
  },
  async fetch() {
    await Promise.all([
      fetch("api/utils/cwd")
        .then((response) => response.text())
        .then((directory) => {
          set({ directory });
        }),
      fetch("api/project")
        .then((response) => response.json())
        .then((config) => {
          set({ config });
        }),
    ]);
  },
}));
