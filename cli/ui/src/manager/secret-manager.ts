import { create } from "zustand";
import type { SecureSession } from "@/core/web";
import type { SecretManagerConfig } from "@/types";

export type SecretManger = {
  is_new: boolean;
  directory: string | null;
  config: SecretManagerConfig | null;
  session: SecureSession | null;
};

interface SecretMangerExtended extends SecretManger {
  update(state: Partial<SecretManger>): void;
  fetch(): Promise<void>;
}

export const useSecretManager = create<SecretMangerExtended>((set) => ({
  is_new: false,
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
      fetch("api/config").then((response) => {
        if (response.ok) {
          response.json().then((config) => set({ config, is_new: false }));
        }
        if (response.status === 404) {
          set({ is_new: true });
        }
      }),
    ]);
  },
}));
