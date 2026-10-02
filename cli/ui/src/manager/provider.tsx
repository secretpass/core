import { createContext, type ReactNode, useContext } from "react";
import type { SecretpassUser, StoredPublicKey } from "@/core/web";
import type { SecretManagerConfig } from "@/types";
import { type ComplexState, useComplexState } from "@/utils";

export interface SecretMangerState {
  directory: string;
  user: SecretpassUser;
  config: SecretManagerConfig;
  public_key: StoredPublicKey;
  public_keys: StoredPublicKey[];
}

const SecretManagerContext = createContext<ComplexState<SecretMangerState>>(
  JSON.parse("null"),
);

export function useSecretManager() {
  return useContext(SecretManagerContext);
}

export function SecretManagerProvider({
  state,
  children,
}: {
  state: SecretMangerState;
  children: ReactNode;
}) {
  const complex_state = useComplexState<SecretMangerState>(state);

  return (
    <SecretManagerContext.Provider value={complex_state}>
      {children}
    </SecretManagerContext.Provider>
  );
}
