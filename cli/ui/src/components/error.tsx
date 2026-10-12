import { Alert } from "@heroui/react";
import { Activity, useCallback, useState } from "react";
import { useAnalytics } from "@/features/analytics/provider";

export type ErrorValue = { title: string; content: string };

export type UnsafeFunction<T> = () => Promise<T>;
export type UnsafeEvaluationResult<T> =
  | { ok: true; result: T }
  | { ok: false; error: unknown };

export function useError() {
  const [value, setValue] = useState<ErrorValue | null>(null);
  const [is_pending, setIsPending] = useState<boolean>(false);
  const analytics = useAnalytics();
  const catch_error = useCallback(
    async <T,>(
      title: string,
      unsafe: UnsafeFunction<T>,
      action?: string,
    ): Promise<UnsafeEvaluationResult<T>> => {
      setIsPending(true);
      try {
        const result = await unsafe();
        setValue(null);
        return {
          ok: true,
          result,
        };
      } catch (error) {
        setValue({ title, content: String(error) });
        analytics?.captureException(error, { action });
        return {
          ok: false,
          error,
        };
      } finally {
        setIsPending(false);
      }
    },
    [analytics],
  );

  return {
    value,
    is_pending,
    catch: catch_error,
  };
}

export function ErrorDisplay({ value }: { value: ErrorValue | null }) {
  return (
    <Activity mode={value ? "visible" : "hidden"}>
      <Alert status="danger" className="max-w-full">
        <Alert.Indicator />
        <Alert.Content className="max-w-full">
          <Alert.Title>{value?.title}</Alert.Title>
          <Alert.Description className="max-w-full overflow-x-auto">
            {value?.content}
          </Alert.Description>
        </Alert.Content>
      </Alert>
    </Activity>
  );
}
