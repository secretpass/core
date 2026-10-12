import {
  AnalyticsExtensions,
  ConversationsExtensions,
} from "posthog-js/dist/extension-bundles";
import { PostHog } from "posthog-js/dist/module.slim.no-external";
import "posthog-js/dist/conversations";
import "posthog-js/dist/exception-autocapture";
import {
  createContext,
  type ReactNode,
  useContext,
  useEffect,
  useState,
} from "react";

import { makeRequest } from "@/utils";

interface AnalyticsOptions {
  ui_host: string;
  api_host: string;
  project_token: string;
}

const AnalyticsContext = createContext<PostHog | null>(null);

export function useAnalytics(): PostHog | null {
  return useContext(AnalyticsContext);
}

export function AnalyticsProvider({ children }: { children: ReactNode }) {
  const [client, setClient] = useState<PostHog | null>(null);

  useEffect(() => {
    makeRequest<AnalyticsOptions>("/api/support").then((response) => {
      if (!response.ok) {
        return;
      }

      const opts = response.json();

      const posthog = new PostHog();
      posthog.init(opts.project_token, {
        autocapture: true,
        ui_host: opts.ui_host,
        api_host: opts.api_host,
        defaults: "2026-06-25",
        person_profiles: "identified_only",
        persistence: "localStorage",
        disable_external_dependency_loading: true,
        __extensionClasses: {
          ...ConversationsExtensions,
          ...AnalyticsExtensions,
        },
        loaded: () => {
          setClient(posthog);
        },
      });
    });
  }, []);

  return (
    <AnalyticsContext.Provider value={client}>
      {children}
    </AnalyticsContext.Provider>
  );
}
