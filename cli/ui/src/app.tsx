import { useEffect, useState } from "react";
import initCore from "@/core/web";
import { Navbar } from "@/features/navigation";
import { NewProjectSetup } from "@/features/setup";
import { useSecretManager } from "@/manager";

export function App() {
  const [loading, setLoading] = useState(false);
  const manager = useSecretManager();

  useEffect(() => {
    setLoading(true);
    initCore()
      .then(() => manager.fetch())
      .finally(() => setLoading(false));
  }, [manager.fetch]);

  return (
    <main className="flex flex-col min-h-screen min-w-screen">
      <Navbar />

      {loading && (
        <div className="grow flex justify-center items-center text-xl">
          Loading...
        </div>
      )}
      {!loading && manager.config === null && <NewProjectSetup />}
    </main>
  );
}
