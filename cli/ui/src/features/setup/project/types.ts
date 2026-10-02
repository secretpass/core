import * as z from "zod";
import type { EncryptionAlgorithm, PasskeyResidency } from "@/core/web";

export const NewEnvironmentItemSchema = z.object({
  name: z
    .string()
    .min(1)
    .regex(/^[a-zA-Z_][a-zA-Z0-9_-]*$/, {
      message:
        "Must be a valid environment variable name (letters, numbers, underscores, cannot start with a number).",
    }),
  description: z.string().optional(),
});

export const NewEnvironmentsSchema = z
  .array(NewEnvironmentItemSchema)
  .min(1)
  .refine(
    (items) => new Set(items.map((item) => item.name)).size === items.length,
  );

export type NewEnvironment = z.infer<typeof NewEnvironmentItemSchema>;

export interface RegistrationState {
  id: string;
  project_name: string;
  description: string;
  algorithm: EncryptionAlgorithm;
  residency: PasskeyResidency;
  environments: z.infer<typeof NewEnvironmentsSchema>;
  created: string;

  user_id: string;
  user_display_name: string;
  user_email_address: string;

  key_name: string;
}
