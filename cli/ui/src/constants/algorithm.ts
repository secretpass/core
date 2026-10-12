import type { EncryptionAlgorithm } from "@/core/web";

export const ENCRYPTION_ALGORITHMS: {
  value: EncryptionAlgorithm;
  label: string;
  description: string;
}[] = [
  {
    value: "ECC",
    label: "Elliptic-curve Cryptography (ECC - X25519)",
    description:
      'Fast and secure key exchange algorithm, it\'s not considered "safe" against future quantum computer attacks.',
  },
  {
    value: "KEM",
    label: "Key Encapsulation Mechanism (KEM - Kyber)",
    description:
      "Post-quantum attack safe; has relatively larger keys and cipher outputs but relatively fast computation in comparison to X25519.",
  },
  {
    value: "Hybrid",
    label: "Hybrid (ECC + KEM)",
    description:
      "A combination of these 2 algorithms ensuring safety if either of the algorithms is ever compromised.",
  },
];
