import { Button, IconChevronRight } from "@heroui/react";
import clsx from "clsx";
import {
  type RefObject,
  useCallback,
  useEffect,
  useRef,
  useState,
} from "react";
import { ErrorDisplay, useError } from "@/components";
import { LoadingBar } from "@/components/loader";
import type { CommandInstanceDetails, LogItem } from "@/core/web";
import { resolveRequest } from "@/utils";

export function InstanceLogs({
  instance,
  buffer_size,
  ref,
}: {
  instance: CommandInstanceDetails;
  buffer_size: number;
  ref: RefObject<HTMLDivElement | null>;
}) {
  const error = useError();
  const [{ logs, cursor }, setLogs] = useState<{
    logs: LogItem[];
    cursor: number;
  }>({ logs: [], cursor: 0 });
  const bottom_ref = useRef<HTMLDivElement>(null);

  const [bottom_visible, setBottomVisible] = useState<boolean>(false);

  const fetchLatestLogs = useCallback(async () => {
    const result = await error.catch("Error fetching logs", () =>
      resolveRequest<LogItem[]>(
        `/api/run/instance/${instance.id}/logs/${cursor}`,
      ),
    );
    if (!result.ok) {
      return;
    }
    if (!result.result.length) {
      return;
    }

    setLogs((current) => {
      current.logs.push(...result.result);
      const last = current.logs[current.logs.length - 1];

      if (current.logs.length > buffer_size) {
        current.logs = current.logs.slice(-buffer_size);
      }
      return { logs: current.logs, cursor: last?.[0] ?? 0 };
    });
  }, [cursor, instance.id, error.catch, buffer_size]);

  useEffect(() => {
    if (!ref?.current || !bottom_ref?.current) {
      return;
    }

    const scrollObserver = new IntersectionObserver(
      (entries) => {
        setBottomVisible(entries[0].intersectionRatio >= 0);
      },
      { root: ref.current },
    );

    scrollObserver.observe(bottom_ref.current);
    return () => scrollObserver.disconnect();
  }, [ref]);

  useEffect(() => {
    if (!bottom_visible) {
      return;
    }

    fetchLatestLogs();
    const interval = setInterval(fetchLatestLogs, 1000);
    return () => clearInterval(interval);
  }, [bottom_visible, fetchLatestLogs]);

  return (
    <div className="flex flex-col gap-4">
      <ErrorDisplay value={error.value} />

      {logs.map((item) => (
        <LogItemDisplay key={item[0]} item={item} />
      ))}

      <div ref={bottom_ref} className="flex items-center gap-4 justify-center">
        {error.is_pending && (
          <LoadingBar size="sm" isIndeterminate className="flex-1" />
        )}
        <Button
          isDisabled={error.is_pending}
          variant="ghost"
          onClick={fetchLatestLogs}
        >
          fetch more
        </Button>
        {error.is_pending && (
          <LoadingBar size="sm" isIndeterminate className="flex-1" />
        )}
      </div>
    </div>
  );
}

function LogItemDisplay({ item }: { item: LogItem }) {
  const [, date, source, line] = item;
  return (
    <div className="flex items-start font-mono gap-2">
      <IconChevronRight className={"text-muted"} />
      <span className="text-muted">{date.substring(11, 22)}</span>
      <pre
        className={clsx(
          "pt-1",
          source === "StdOut" && "text-success",
          source === "StdError" && "text-danger",
        )}
      >
        {line}
      </pre>
    </div>
  );
}
