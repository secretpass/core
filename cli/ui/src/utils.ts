import { useState } from "react";
import { create } from "zustand";
import type { SecretpassProject } from "./core/web";

export type ComplexState<T extends object> = T & {
  readonly reset: () => void;
  readonly update: (value: Partial<T>) => void;
};

export function useComplexState<T extends object>(
  init: (setState: (update: Partial<T>) => void, getState: () => T) => T,
): ComplexState<T> {
  const [useZustandState] = useState(() =>
    create<ComplexState<T>>((setState, getState) => ({
      ...init(setState as (update: Partial<T>) => void, getState),
      update: (value: Partial<T>) =>
        setState(value as Partial<ComplexState<T>>),
      reset: () => {
        setState(init(setState as (update: Partial<T>) => void, getState));
      },
    })),
  );
  return useZustandState();
}

export async function makeRequest<T = unknown>(
  input: RequestInfo | URL,
  init?: RequestInit,
) {
  const headers =
    init?.headers ??
    (init?.body
      ? {
          "Content-Type": "application/json",
        }
      : undefined);
  const response = await fetch(input, { ...init, headers });

  const text = await response.text();
  return {
    ok: response.ok,
    status: response.status,
    text,
    headers: response.headers,
    json() {
      return JSON.parse(text) as T;
    },
  };
}

export async function resolveRequest<T = unknown>(
  input: RequestInfo | URL,
  init?: RequestInit,
): Promise<T> {
  const response = await makeRequest<T>(input, init);

  if (response.ok) {
    if (!response.text) {
      return undefined as unknown as T;
    }
    return response.json();
  }

  return Promise.reject(response.text);
}

export function assertProjectEqual(a: SecretpassProject, b: SecretpassProject) {
  if (a.id !== b.id) {
    throw new Error(
      `Project ID mismatch ${a.id}(${a.name}) != ${b.id}(${b.name})`,
    );
  }
  if (a.algorithm !== b.algorithm) {
    throw new Error(
      `Encryption algorithm mismatch ${a.algorithm} != ${b.algorithm}`,
    );
  }

  if (a.residency !== b.residency) {
    throw new Error(
      `Passkey residency mismatch ${a.residency} != ${b.residency}`,
    );
  }
  if (a.created_at !== b.created_at) {
    throw new Error(
      `Project creation date mismatch ${a.created_at} != ${b.created_at}`,
    );
  }
}

export function clone<T>(object: T, update?: (cloned: T) => T): T {
  const cloned: T = JSON.parse(JSON.stringify(object));

  if (update) {
    return update(cloned);
  }
  return cloned;
}
