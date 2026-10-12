import { Toast } from "@heroui/react";
import { useEffect, useMemo, useState } from "react";
import initCore, { type SecretpassProject } from "@/core/web";
import { Navbar } from "@/features/navigation";
import { NewProjectSetup } from "@/features/setup";
import { SecretManagerProvider, SecretManagerView } from "@/manager";
import { CWD_HEADER } from "./constants";
import { useAnalytics } from "./features/analytics/provider";
import { LoginDashboard } from "./features/auth/dash";
import { makeRequest, useComplexState } from "./utils";

export function App() {
  const state = useComplexState<{
    loading: boolean;
    directory: string | null;
    project: SecretpassProject | null;
  }>(() => ({ loading: true, project: null, directory: null }));

  const [public_key_id, setPublicKeyId] = useState<string | null>(null);
  const analytics = useAnalytics();

  useEffect(() => {
    state.update({ loading: true });

    initCore()
      .then(() => makeRequest<SecretpassProject>("/api/project"))
      .then((response) => {
        state.update({
          directory: response.headers.get(CWD_HEADER),
        });
        if (response.ok) {
          const project = response.json();

          analytics?.group("local-project", project.id, {
            name: project.name,
            algorithm: project.algorithm,
            residency: project.residency,
          });
          state.update({ project: project });
        }
      })
      .finally(() => state.update({ loading: false }));
  }, [state.update, analytics]);

  const is_new_project = useMemo(
    () => !state.loading && state.project === null && public_key_id === null,
    [state.loading, state.project, public_key_id],
  );
  const is_pending_login = useMemo(
    () => !state.loading && state.project !== null && public_key_id === null,
    [state.loading, state.project, public_key_id],
  );
  const is_logged_in = useMemo(
    () => !state.loading && public_key_id !== null,
    [state.loading, public_key_id],
  );

  return (
    <main className="min-h-screen min-w-screen">
      <Toast.Provider />
      <Navbar />

      {state.loading && (
        <div className="grow flex justify-center items-center text-xl">
          Loading...
        </div>
      )}
      {is_new_project && (
        <NewProjectSetup
          directory={state.directory as string}
          onAuthComplete={setPublicKeyId}
        />
      )}
      {is_logged_in && (
        <SecretManagerProvider public_key_id={public_key_id as string}>
          <SecretManagerView />
        </SecretManagerProvider>
      )}
      {is_pending_login && (
        <LoginDashboard
          directory={state.directory as string}
          project={state.project as SecretpassProject}
          onAuthComplete={setPublicKeyId}
        />
      )}
    </main>
  );
}
