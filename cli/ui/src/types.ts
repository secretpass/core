import type { OperationMode } from "./constants";
import type { SecretpassProject } from "./core/web";

export interface SecretManagerConfig {
  mode: OperationMode;
  project: SecretpassProject;
  is_new?: boolean;
}
