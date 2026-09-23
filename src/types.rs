use serde::{Deserialize, Serialize};
use std::fmt;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PasskeyResidency {
    OnDevice,      // Store the passkey on a physical device, a hardware key, laptop, phone, etc.
    HardwareKey,   // Store the passkey on a hardware key e.g., Yubikey
    SyncedAllowed, // On device or hardware key preferred, but synced keys (e.g., icloud keychain) are okay
}

impl fmt::Display for PasskeyResidency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PasskeyResidency::OnDevice => write!(f, "OnDevice"),
            PasskeyResidency::HardwareKey => write!(f, "HardwareKey"),
            PasskeyResidency::SyncedAllowed => write!(f, "SyncedAllowed"),
        }
    }
}
