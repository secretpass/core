import type { OperationMode } from "./constants";
import type {
  SecretpassEnvironment,
  SecretpassProject,
  SecretpassUser,
} from "./core/web";

export interface SecretManagerConfig {
  mode: OperationMode;
  project: SecretpassProject;
  users: SecretpassUser[];
  environments: SecretpassEnvironment[];
}
