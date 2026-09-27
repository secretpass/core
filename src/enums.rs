use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(skip_typescript)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, EnumString, Display, Tsify)]
pub enum PasskeyResidency {
    OnDevice,      // Store the passkey on a physical device, a hardware key, laptop, phone, etc.
    HardwareKey,   // Store the passkey on a hardware key e.g., Yubikey
    SyncedAllowed, // On device or hardware key preferred, but synced keys (e.g., icloud keychain) are okay
}

#[wasm_bindgen(skip_typescript)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, EnumString, Display, Tsify)]
pub enum OperationMode {
    Local,
    Cloud,
}

#[wasm_bindgen(skip_typescript)]
#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Debug, EnumString, Display, Tsify)]
pub enum EncryptionAlgorithm {
    ECC,    // X25519
    KEM,    // ML-KEM-768
    Hybrid, // Encrypts with both algorithms
}

// [163, 99, 102, 109, 116, 100, 110, 111, 110, 101, 103, 97, 116, 116, 83, 116, 109, 116, 160, 104, 97, 117, 116, 104, 68, 97, 116, 97, 88, 152, 239, 16, 235, 144, 98, 167, 8, 118, 253, 87, 158, 149, 207, 110, 14, 226, 148, 49, 113, 205, 211, 73, 109, 221, 164, 145, 89, 88, 217, 147, 32, 128, 93, 0, 0, 0, 0, 251, 252, 48, 7, 21, 78, 78, 204, 140, 11, 110, 2, 5, 87, 215, 189, 0, 20, 54, 166, 6, 248, 156, 119, 205, 220, 207, 225, 50, 32, 186, 28, 127, 198, 104, 37, 51, 168, 165, 1, 2, 3, 38, 32, 1, 33, 88, 32, 105, 218, 139, 242, 209, 66, 39, 180, 132, 128, 48, 79, 27, 238, 81, 78, 253, 60, 103, 58, 152, 253, 157, 101, 18, 104, 228, 157, 251, 187, 206, 233, 34, 88, 32, 121, 114, 114, 209, 57, 9, 148, 255, 88, 133, 17, 245, 198, 92, 171, 243, 124, 97, 104, 125, 191, 107, 46, 156, 255, 165, 212, 58, 36, 204, 11, 232]
