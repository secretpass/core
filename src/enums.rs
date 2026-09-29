use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(skip_typescript)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, EnumString, Display, Tsify)]
#[tsify(namespace)]
pub enum PasskeyResidency {
    OnDevice,      // Store the passkey on a physical device, a hardware key, laptop, phone, etc.
    HardwareKey,   // Store the passkey on a hardware key e.g., Yubikey
    SyncedAllowed, // On device or hardware key preferred, but synced keys (e.g., icloud keychain) are okay
}

#[wasm_bindgen(skip_typescript)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, EnumString, Display, Tsify)]
#[tsify(namespace)]
pub enum OperationMode {
    Local,
    Cloud,
}

#[wasm_bindgen(skip_typescript)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, EnumString, Display, Tsify)]
#[tsify(namespace)]
pub enum EncryptionAlgorithm {
    ECC,    // X25519
    KEM,    // ML-KEM-768
    Hybrid, // Encrypts with both algorithms
}

#[wasm_bindgen(skip_typescript)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, EnumString, Display, Tsify)]
#[tsify(namespace)]
pub enum PublicKeyType {
    User,
    Machine,
}

#[wasm_bindgen(skip_typescript)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, EnumString, Display, Tsify)]
#[tsify(namespace)]
pub enum UserLevel {
    Admin,
    Standard,
    Viewer,
}
