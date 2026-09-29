import { useCallback, useEffect, useState } from "react";
import { create } from "zustand";
import { immer } from "zustand/middleware/immer";

export type ComplexState<T extends object> = T & {
  update: (value: Partial<T>) => void;
  complexUpdate: (update: (state: T) => void) => void;
};

export function useComplexState<T extends object>(
  defaultValue: T,
): ComplexState<T> {
  const [useZustandState] = useState(() =>
    create(
      immer<ComplexState<T>>((set) => ({
        ...defaultValue,
        update: (value: Partial<T>) => set((state) => ({ ...state, ...value })),
        complexUpdate: (updater) => {
          set((state) => updater(state as ComplexState<T>));
        },
      })),
    ),
  );
  return useZustandState();
}

export function useUncaughtWasmError() {
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const listener = (event: ErrorEvent) => {
      const stack = (event.error?.stack as string) || "";
      if (stack.includes(".wasm:wasm-function[")) {
        setError(event.message.replace("Uncaught Error: ", ""));
      }
    };
    window.addEventListener("error", listener);
    return () => {
      window.removeEventListener("error", listener);
    };
  }, []);

  const clear = useCallback(() => setError(null), []);

  return { message: error, clear };
}
