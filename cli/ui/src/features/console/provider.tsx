import {
  createContext,
  type ReactNode,
  useContext,
  useMemo,
  useState,
} from "react";
import type { CommandInstanceDetails } from "@/core/web";

interface InstancesContextProps {
  children: ReactNode;
  instances: CommandInstanceDetails[];
  refreshInstances: () => Promise<boolean>;
}

interface InstancesContextState
  extends Omit<InstancesContextProps, "children"> {
  active: string;
  setActive: (active: string) => void;
  instances: CommandInstanceDetails[];
  refreshInstances: () => Promise<boolean>;
}

const InstancesContext = createContext<InstancesContextState>(
  JSON.parse("null"),
);

export function useInstances() {
  return useContext(InstancesContext);
}

export function InstancesProvider({
  children,
  instances,
  refreshInstances,
}: InstancesContextProps) {
  const [active, setActive] = useState<string>("new");
  const state: InstancesContextState = useMemo(
    () => ({ instances, refreshInstances, active, setActive }),
    [instances, refreshInstances, active],
  );

  return (
    <InstancesContext.Provider value={state}>
      {children}
    </InstancesContext.Provider>
  );
}
