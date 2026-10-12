import { toast } from "@heroui/react";
import {
  createContext,
  type ReactNode,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useState,
} from "react";
import { ErrorDisplay, useError } from "@/components";
import type { LoginParams, UserApplicationState, UserLevel } from "@/core/web";
import { resolveRequest } from "@/utils";

type GetPackageFunction = (
  level: UserLevel,
  environments: string[],
) => Record<string, Record<string, string>>;

type RefreshStateFunction = (message?: string) => Promise<void>;

type GetEnvPackageFunction = (
  environment: string,
) => Record<string, string> | undefined;

export interface SecretManagerState extends UserApplicationState {
  readonly refreshState: RefreshStateFunction;
  readonly login_params: LoginParams;
  readonly getSecretsPackage: GetPackageFunction;
  readonly getEnvSecretsPackage: GetEnvPackageFunction;
}

const SecretManagerContext = createContext<SecretManagerState>(
  JSON.parse("null"),
);

export function useSecretManager() {
  return useContext(SecretManagerContext);
}

export function SecretManagerProvider({
  public_key_id,
  children,
}: {
  public_key_id: string;
  children: ReactNode;
}) {
  const error = useError();
  const [app_state, setAppState] = useState<UserApplicationState | null>();

  const refreshState: RefreshStateFunction = useCallback(
    async (message?: string) => {
      const result = await error.catch("Error reading application state", () =>
        resolveRequest<UserApplicationState>(
          `/api/project/state/${public_key_id}`,
        ),
      );
      if (result.ok) {
        setAppState(result.result);
        if (message) {
          toast(message, { variant: "success" });
        }
      } else {
        setAppState(null);
      }
    },
    [error.catch, public_key_id],
  );

  const getEnvSecretsPackage: GetEnvPackageFunction = useCallback(
    (env: string) => {
      return app_state?.lock_package[env]?.encrypted_secrets;
    },
    [app_state],
  );

  const getSecretsPackage: GetPackageFunction = useCallback(
    (
      level: UserLevel,
      environments: string[],
    ): Record<string, Record<string, string>> => {
      const lock_package: Record<string, Record<string, string>> = {};
      if (!app_state) {
        return lock_package;
      }
      if (level === "Admin") {
        environments = Object.keys(app_state.lock_package);
      }
      for (const env of environments) {
        lock_package[env] = app_state.lock_package[env].encrypted_secrets;
      }
      return lock_package;
    },
    [app_state],
  );

  const state: SecretManagerState | null = useMemo(() => {
    if (!app_state) {
      return null;
    }

    const login_params: LoginParams = {
      user: app_state.user,
      project: app_state.config.project,
      public_keys: [app_state.public_key],
    };

    return {
      ...app_state,
      login_params,
      refreshState,
      getSecretsPackage,
      getEnvSecretsPackage,
    };
  }, [app_state, getSecretsPackage, getEnvSecretsPackage, refreshState]);

  useEffect(() => {
    refreshState();
  }, [refreshState]);

  if (error.value || !state) {
    return (
      <div className="content flex items-center justify-center">
        <ErrorDisplay value={error.value} />
      </div>
    );
  }

  return (
    <SecretManagerContext.Provider value={state}>
      {children}
    </SecretManagerContext.Provider>
  );
}
