# SecretPass

SecretPass is a noncustodial credential management framework with support for local (version control) storage or through
our [managed cloud](https://secretpass.cloud).

## How it works

- **Zero Knowledge**: Raw credentials and keys are never transmitted over the network or stored on local or cloud
  storage.
- **Passkey-Based**: We use passkeys to derive your private keys, your key stays with you.
- **Local Encryption/Decryption**: Data is locked and unlocked right on your device/server.
- **User Control**: If all the users were to lose their keys, your data will be unrecoverable.

### Control Keys

Secretpass supports multiple types of keys based on the device being used to access the secret and the security level.

- **Biometric Passkeys** - Passkeys stored on the user's device:
    - On Windows and Linux devices passkeys are stored on a **Trusted Platform Module (TPM) chip**.
    - On Apple Silicon Macs passkeys are stored in the built-in **Secure Enclave**
        - On Intel-based Macs the passkeys are stored on the **Apple T2 Security Chip**
    - Passkeys can be hardware bound or synced based on the OS and the level of security required.
- **Hardware Keys** - Dedicated hardware keys like YubiKey for the highest level of device bound security.
- **Machine Keys**
    - These are private keys authorized to access specific secrets for machine based workflows e.g. ai agents, your
      code, your CI workflows.
    - Since these keys are not as secure as Biometric Passkeys and Hardware Keys, restrict their scope to only what they
      need to access per environment or per individual secret

### Encryption Algorithms

Secretpass supports two major algorithms that can be used interchangeably based on the security needs of your project.

1. [X25519 + AES256](https://en.wikipedia.org/wiki/Curve25519) - Classical elliptic-curve key exchange algorithm that is
   fast and secure on many types of hardware, it's not safe against future quantum computer attacks.
2. [ML-KEM-768 + AES256](https://en.wikipedia.org/wiki/ML-KEM) - Post-quantum attack safe key-encapsulation mechanism;
   has relatively larger keys and cipher outputs but relatively fast computation in comparison to `X25519`.
3. Hybrid (`X25519` + `ML-KEM-768`) + AES256 - A combination of these 2 algorithms ensuring safety if either of the
   algorithms is ever compromised.
