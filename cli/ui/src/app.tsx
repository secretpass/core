import { useEffect, useMemo, useState } from "react";
import initCore, { type SecretpassProject } from "@/core/web";
import { Navbar } from "@/features/navigation";
import { NewProjectSetup } from "@/features/setup";
import {
  SecretManagerProvider,
  SecretManagerView,
  type SecretMangerState,
} from "@/manager";
import { CWD_HEADER } from "./constants";
import { LoginDashboard } from "./features/auth/dash";
import { makeRequest, useComplexState } from "./utils";

export function App() {
  const state = useComplexState<{
    loading: boolean;
    directory: string | null;
    project: SecretpassProject | null;
  }>({ loading: true, project: null, directory: null });

  const [sm_state, setSecretManagerState] = useState<SecretMangerState | null>(
    null,
  );

  useEffect(() => {
    state.update({ loading: true });

    initCore()
      .then(() => makeRequest<SecretpassProject>("/api/project"))
      .then((response) => {
        state.update({
          directory: response.headers.get(CWD_HEADER),
        });
        if (response.ok) {
          state.update({ project: response.json() });
        }
      })
      .finally(() => state.update({ loading: false }));
  }, [state.update]);

  const is_new_project = useMemo(
    () => !state.loading && state.project === null && sm_state === null,
    [state.loading, state.project, sm_state],
  );
  const is_pending_login = useMemo(
    () => !state.loading && state.project !== null && sm_state === null,
    [state.loading, state.project, sm_state],
  );
  const is_logged_in = useMemo(
    () => !state.loading && sm_state !== null,
    [state.loading, sm_state],
  );

  return (
    <main className="flex flex-col min-h-screen min-w-screen">
      <Navbar />

      {state.loading && (
        <div className="grow flex justify-center items-center text-xl">
          Loading...
        </div>
      )}
      {is_new_project && (
        <NewProjectSetup
          directory={state.directory as string}
          onAuthComplete={setSecretManagerState}
        />
      )}
      {is_logged_in && (
        <SecretManagerProvider state={sm_state as SecretMangerState}>
          <SecretManagerView />
        </SecretManagerProvider>
      )}
      {is_pending_login && (
        <LoginDashboard
          directory={state.directory as string}
          project={state.project as SecretpassProject}
          onAuthComplete={setSecretManagerState}
        />
      )}
    </main>
  );
}
