# Secretpass – Local Secret Storage

Local-based storage allows you to securely store secrets locally and sync them through git as part of your project.
These secrets are secured by passkeys and can be conveniently managed through the web interface bundled with the `spass`
cli.

The project folder `.spass` has the following structure:

- `config.json` - This file holds the primary configuration for the project including the project details, environments
  and users.
- `config.lock` - This file holds multiple copies of `config.json` encrypted with public keys of all users in the
  project.
    - This file is used to verify the authenticity of the project configuration and takes precedence over the flat file.
- `public-keys.json` - This file holds the public keys of all users and machines in the project.
- `public-keys.lock` - This file holds multiple copies of `public-keys.json` encrypted with public keys of all users in
  the project.
    - This file is used to verify the authenticity of the public keys and takes precedence over the flat file.

## Create a new project

The sequence below depicts the registration process when the user runs `spass` for the first time in a project folder
that does not have a
`.spass` folder. It creates a new project with the user as the admin.

```mermaid
---
config:
  theme: redux-dark-color
---
%%{init: { "sequence": {"messageAlign": "left", "noteAlign": "left", "wrap": true} }}%%
sequenceDiagram
    autonumber
    participant Core@{"type": "boundary"} as Wasm Core(rust in browser)
    participant Vault@{"type": "boundary"} as Secure Enclave
    actor User
    participant App@{"type": "control"} as Web App(javascript)
    participant Cli@{"type": "control"} as Spass Cli
    User ->> App: Opens Secret Manager Interface
    App <<->> Cli: [GET]<br/>- /api/project<br/><br/>Response:<br/>- Project does not exist
    App ->> User: Presents project creation interface
    User ->> App: Submits all project data
    App -->> Core: Request creation of a passkey
    activate Core
    Core ->> Vault: Request creation
    activate Vault
    Vault <<->> User: Request approval
    Vault ->> Core: Result:<br/> - Authorization signature<br/> - Passkey public key
    deactivate Vault
    Core -->> App: Action:<br/>- Verify signature<br/><br/>Result:<br/>- Passkey public key
    deactivate Core
    App ->> User: Presents confirmation interface
    User ->> App: Confirm project creation
    App -->> Core: Request derived public key
    activate Core
    Core ->> Vault: Request authorization with PRF extension
    activate Vault
    Vault <<->> User: Request approval
    Vault ->> Core: Result:<br/> - Authorization signature<br/> - PRF results
    deactivate Vault
    Core -->> App: Action:<br/>- Verify signature<br/>- Derive private key from PRF results<br/>- Generate public key from private key<br/><br/>Result:<br/>- Generated public key
    deactivate Core
    App <<->> Cli: [POST]<br/>- /api/project<br/><br/>Repnse:<br/>- Created project
```

## User Login

The sequence below depicts the login process a user goes through when they open an existing project.

```mermaid
---
config:
  theme: redux-dark-color
---
%%{init: { "sequence": {"messageAlign": "left", "noteAlign": "left", "wrap": true} }}%%
sequenceDiagram
    autonumber
    participant Core@{"type": "boundary"} as Wasm Core
    participant Vault@{"type": "boundary"} as Secure Enclave
    actor User
    participant App@{"type": "control"} as Web App
    participant Cli@{"type": "control"} as Spass Cli
    User ->> App: Opens Secret Manager Interface
    App <<->> Cli: [GET]<br/>- /api/project<br/><br/>Response:<br/>- Loaded project details only
    App ->> User: Presents login interface
    User ->> App: Enters their username
    App <<->> Cli: [GET]<br/>- /api/user/keys?username=<br/><br/>Response:<br/>- List of user keys
    App ->> User: Presents key selection UI
    User ->> App: Selects a key
    App <<->> Cli: [GET]<br/>- /api/configs/encrypted/:key_id<br/><br/>Response:<br/>- Encrypted project config<br/>- Encrypted public keys
    App -->> Core: Request:<br/>- Authorization<br/>- Decryption of project config<br/>- Decryption of public keys
    activate Core
    Core ->> Vault: Request authorization with PRF extension
    activate Vault
    Vault <<->> User: Request approval
    Vault ->> Core: Result:<br/> - Authorization signature<br/> - PRF results
    deactivate Vault
    Core -->> App: Action:<br/>- Verify authorization<br/>- Derive private key from PRF results<br/>- Decrypt project config<br/>- Decrypt public keys config<br/>- Calculate config and keys hash<br/><br/>Result:<br/>- Decrypted configs<br/>- Configs hash
    deactivate Core
    App ->> User: Allow the user to interact based on decoded config
    App <<-->> Cli: Future requests must use original config hash as `Authorization`
```

**From this step onwards, this tutorial assumes the user is already logged in**

## View Secret

```mermaid
---
config:
  theme: redux-dark-color
---
%%{init: { "sequence": {"messageAlign": "left", "noteAlign": "left", "wrap": true} }}%%
sequenceDiagram
    autonumber
    participant Core@{"type": "boundary"} as Wasm Core
    participant Vault@{"type": "boundary"} as Secure Enclave
    actor User
    participant App@{"type": "control"} as Web App
    participant Cli@{"type": "control"} as Spass Cli
    User ->> App: Clicks View Secret
    App <<->> Cli: [GET]<br/>- /api/secrets/[env]/[name]/key/:key-id<br/><br/>Response:<br/>- Encrypted secret for the key
    App -->> Core: Request:<br/>- Authorization<br/>- Decrypted secret for the key
    activate Core
    Core ->> Vault: Request authorization with PRF extension
    activate Vault
    Vault <<->> User: Request approval
    Vault ->> Core: Result:<br/>- Authorization signature<br/>- PRF results
    deactivate Vault
    Core -->> App: Action:<br/>- Verify signature<br/>- Derive private key from PRF<br/>- Decrypt Secret with private key<br/><br/>Result:<br/>- Decrypted secret value
    deactivate Core
    App ->> User: Display plain text secret
```

## Add/Modify Secret

```mermaid
---
config:
  theme: redux-dark-color
---
%%{init: { "sequence": {"messageAlign": "left", "noteAlign": "left", "wrap": true} }}%%
sequenceDiagram
    autonumber
    participant Core@{"type": "boundary"} as Wasm Core
    participant Vault@{"type": "boundary"} as Secure Enclave
    actor User
    participant App@{"type": "control"} as Web App
    participant Cli@{"type": "control"} as Spass Cli
    Note over Core, Cli: Refer to View Secret - sequence, this assumes the user can see the plain secret
    User ->> App: Updates the secret value and clicks save
    App <<->> Cli: [GET]<br/>- /api/secrets/[env]/[name]/keys
    App -->> Core: Requests authorization and encryption
    activate Core
    Core ->> Vault: Request authorization
    activate Vault
    Vault <<->> User: Request approval
    Vault ->> Core: Result:<br/>- Authorization signature
    deactivate Vault
    Core -->> App: Action:<br/>- Verify signature<br/>- Encrypt secret with all public keys<br/><br/>Result:<br/>- Encrypted secret values
    deactivate Core
    App ->> Cli: [POST]<br/>- /api/secrets/[env]/[name]
```

## Adding a new public key

While adding a new public key, the process involves 3 steps:

1. Admin generates a setup config and shares it with the user.
2. The user creates a passkey, generates a public key and shares it with the admin.
3. The admin adds the public key to the project, giving the user access.

The process is the same whether the user is new or existing.

### 1. Admin: Generate setup config

```mermaid
---
config:
  theme: redux-dark-color
---
%%{init: { "sequence": {"messageAlign": "left", "noteAlign": "left", "wrap": true} }}%%
sequenceDiagram
    autonumber
    actor User as Admin
    participant App@{"type": "control"} as Web App
    App ->> User: Presents administration interface
    User ->> App: - Selects add public key<br/>- Selects existing user or<br/>- Eneter details for new users<br/>- Select generate config
    App ->> User: - Presents setup config
    Note over App, User: This config needs to be shared with the user, it's not sensitive and can be shared over open channels
    Note over App, User: The admin can leave the screen, the process can be finished independently later
```

### 2. User: Generate public key

```mermaid
---
config:
  theme: redux-dark-color
---
%%{init: { "sequence": {"messageAlign": "left", "noteAlign": "left", "wrap": true} }}%%
sequenceDiagram
    autonumber
    participant Core@{"type": "boundary"} as Wasm Core
    participant Vault@{"type": "boundary"} as Secure Enclave
    actor User
    participant App@{"type": "control"} as Web App
    participant Cli@{"type": "control"} as Spass Cli
    User -->> Cli: Run spass-cli
    User ->> App: Open web interface
    App ->> User: Present options<br/>- Login<br/>- Generate public key
    User ->> App: Selects generate public key
    App ->> User: Presents key generation interface
    Note over User, App: For the user to proceed, they'll need a config shared by the admin
    User ->> App: - Enters shared config<br/>- Selects register passkey
    App -->> Core: Request creation of a new passkey
    activate Core
    Core ->> Vault: Request passkey creation with PRF extensions
    activate Vault
    Vault <<->> User: Request approval
    Vault ->> Core: Result:<br/>- Authorization signature<br/>- Created passkey public key
    deactivate Vault
    Core -->> App: Created passkey public key
    deactivate Core
    App ->> User: Present confirmation interface
    User ->> App: Confirm key creation
    App -->> Core: Request authorization and public key generation
    activate Core
    Core ->> Vault: Request authorization with PRF extension
    activate Vault
    Vault <<-->> User: Request approval
    Vault -->> Core: Result
    Vault ->> Core: Result:<br/>- Authorization signature<br/>- PRF results
    deactivate Vault
    Core -->> App: Action<br/>- Verify signature<br/>- Derive private key from pRF results<br/>- Generate public key from private key<br/><br/>Result:<br/>- Generated public key
    deactivate Core
    App ->> User: Presents the newly generated public key
    Note over User, App: The user needs to share the generated public key with the admin. The key is not sensitive and can be shared over open channels
```

### 3. Admin: complete new key addition

```mermaid
---
config:
  theme: redux-dark-color
---
%%{init: { "sequence": {"messageAlign": "left", "noteAlign": "left", "wrap": true} }}%%
sequenceDiagram
    autonumber
    participant Core@{"type": "boundary"} as Wasm Core
    participant Vault@{"type": "boundary"} as Secure Enclave
    actor User as Admin
    participant App@{"type": "control"} as Web App
    participant Cli@{"type": "control"} as Spass Cli
    App ->> User: Presents administration interface
    User ->> App: Selects finalize new public key addition
    App ->> User: Presents new key addition interface
    Note over App, User: This assumes that the admin already received the public key generated from the user
    User ->> App: - Enters the config shared by the user<br/>- Confirms/modifies user access<br/>Selects confirm addition
    App <<->> Cli: - [GET]<br/>- /api/secrets/encrypted/key_id?env=env1,env2<br/><br/>Response:<br/>- Encrypted secrets for the admin key<br/>- Filtered based on what the user can access
    App -->> Core: Request:<br/>- Authorization with PRF<br/>- Decryption of secrets to be shared<br/>- Encryption of secrets with the new public key
    activate Core
    Core ->> Vault: Request authorization with PRF extension
    activate Vault
    Vault <<->> User: Request approval
    Vault ->> Core: Result:<br/>- Authorization signature<br/>- PRF results
    deactivate Vault
    Core -->> App: Action:<br/>- Verify signature<br/>- Derive admin private key from PRF<br/>- Decrypt secrets with admin private key<br/>- Encrypt decrypted secrets with the new public key<br/><br/>Result:<br/>- Secrets encrypted with the new public key
    deactivate Core
    App <<->> Cli: [POST]<br/>- /api/keys<br/>- User and public key details<br/>- Encrypted secrets
    Note over User, App: Once synced, the user can access the project using their passkey
```

## Admin: Give User Env Access

```mermaid
---
config:
  theme: redux-dark-color
---
%%{init: { "sequence": {"messageAlign": "left", "noteAlign": "left", "wrap": true} }}%%
sequenceDiagram
    autonumber
    participant Core@{"type": "boundary"} as Wasm Core
    participant Vault@{"type": "boundary"} as Secure Enclave
    actor User as Admin
    participant App@{"type": "control"} as Web App
    participant Cli@{"type": "control"} as Spass Cli
    App ->> User: Presents administration interface
    User ->> App: - Select update user<br/>- Select new environments to add<br/>- Confirm Action

    par App to Cli
        App <<->> Cli: - [GET]<br/>- /api/secrets/encrypted/key_id?env=env1,env2<br/><br/>Response:<br/>- Encrypted secrets for the admin key<br/>- Filtered based on what the new environments introduced
        App <<->> Cli: - [GET]<br/>- /api/keys/[user-id]<br/><br/>Response:<br/>- Public keys for the user
    end

    App -->> Core: Request:<br/>- Authorization with PRF<br/>- Decryption of secrets to be shared<br/>- Encryption of secrets with the user's public keys
    activate Core
    Core ->> Vault: Request authorization with PRF extension
    activate Vault
    Vault <<->> User: Request approval
    Vault ->> Core: Result:<br/>- Authorization signature<br/>- PRF results
    deactivate Vault
    Core -->> App: Action:<br/>- Verify signature<br/>- Derive admin private key from PRF<br/>- Decrypt secrets with admin private key<br/>- Encrypt decrypted secrets with the user's public keys<br/><br/>Result:<br/>- Secrets encrypted with the user's public keys
    deactivate Core
    App <<->> Cli: [POST]<br/>- /api/secrets<br/>- Encrypted secrets
    Note over User, App: Once synced, the user can access secrets in the added environment
```

## Remove User Env Access

```mermaid
---
config:
  theme: redux-dark-color
---
%%{init: { "sequence": {"messageAlign": "left", "noteAlign": "left", "wrap": true} }}%%
sequenceDiagram
    autonumber
    participant Core@{"type": "boundary"} as Wasm Core
    participant Vault@{"type": "boundary"} as Secure Enclave
    actor User
    participant App@{"type": "control"} as Web App
    participant Cli@{"type": "control"} as Spass Cli
    App ->> User: Presents administration interface
    User ->> App: - Select update user<br/>- Remove environments<br/>- Confirm Action
    App -->> Core: Request authorization
    activate Core
    Core ->> Vault: Request authorization
    activate Vault
    Vault <<->> User: Request approval
    Vault ->> Core: Result:<br/>- Authorization signature
    deactivate Vault
    Core -->> App: Action:<br/>- Verify signature
    deactivate Core
    App <<->> Cli: [PUT]<br/>- /api/users/user-id<br/> - Environemts to remove access to
    Note over User, App: Once synced, the user won't have access secrets to the environment removed
```
