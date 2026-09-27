import { useEffect } from "react";
import initCore from "@/core/web";
import { Navbar } from "@/features/navigation";
import { NewProjectSetup } from "@/features/setup";
import { useSecretManager } from "@/manager";

export function App() {
  const manager = useSecretManager();

  useEffect(() => {
    initCore().then(() => {
      manager.fetch().then();
    });
  }, [manager.fetch]);

  return (
    <main className="flex flex-col min-h-screen min-w-screen">
      <Navbar />

      {manager.config === null && (
        <div className="grow flex justify-center items-center text-xl">
          Loading...
        </div>
      )}
      {manager.config?.is_new && <NewProjectSetup />}
    </main>
  );
}
