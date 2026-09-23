use rand::Rng;
use secretpass_core::utils::get_global_rng;
use secretpass_core::{EncryptionAlgorithm, PrivateKey, PrivateKeySeed};

fn create_seed_keys() -> ([u8; 32], [u8; 32]) {
    let mut rng = get_global_rng().lock().unwrap();
    let mut seed_a = [0u8; 32];
    let mut seed_b = [0u8; 32];

    rng.fill_bytes(&mut seed_a);
    rng.fill_bytes(&mut seed_b);

    (seed_a, seed_b)
}

fn create_same_seed_key() -> (PrivateKey, PrivateKey, PrivateKey) {
    let seed_keys = create_seed_keys();
    (
        PrivateKey::from_seed(PrivateKeySeed {
            algorithm: EncryptionAlgorithm::ECC,
            first: seed_keys.0,
            second: seed_keys.1,
        })
        .unwrap(),
        PrivateKey::from_seed(PrivateKeySeed {
            algorithm: EncryptionAlgorithm::KEM,
            first: seed_keys.0,
            second: seed_keys.1,
        })
        .unwrap(),
        PrivateKey::from_seed(PrivateKeySeed {
            algorithm: EncryptionAlgorithm::Hybrid,
            first: seed_keys.0,
            second: seed_keys.1,
        })
        .unwrap(),
    )
}

#[test]
fn test_ecc_encryption_decryption() {
    let (ecc_key, kem_key, hybrid_key) = create_same_seed_key();

    let original_payload = b"Basic Elliptic-curve cryptography test";

    let encrypted_payload = ecc_key
        .public_key()
        .encrypt(original_payload.to_vec())
        .unwrap();
    assert_ne!(
        original_payload,
        encrypted_payload.as_slice(),
        "encrypted payload cannot be equal to the clear payload"
    );
    assert_ne!(
        encrypted_payload,
        ecc_key
            .public_key()
            .encrypt(original_payload.to_vec())
            .unwrap(),
        "encrypted payload should change with every encryption run"
    );

    let decrypted_payload = ecc_key.decrypt(encrypted_payload.clone()).unwrap();
    assert_eq!(
        decrypted_payload, original_payload,
        "decrypted payload should be equal to the original payload"
    );

    let kem_error = kem_key.decrypt(encrypted_payload.clone()).unwrap_err();
    assert_eq!(
        kem_error.to_string(),
        "Invalid payload/private key",
        "attempting to decrypt ECC payload with KEM key should throw an error"
    );

    let hybrid_error = hybrid_key.decrypt(encrypted_payload.clone()).unwrap_err();
    assert_eq!(
        hybrid_error.to_string(),
        "Invalid payload/private key",
        "attempting to decrypt ECC payload with Hybrid key should throw an error"
    );
}

#[test]
fn test_kem_encryption_decryption() {
    let (ecc_key, kem_key, hybrid_key) = create_same_seed_key();

    let original_payload = b"Basic Key-Encapsulation Mechanism cryptography test";

    let encrypted_payload = kem_key
        .public_key()
        .encrypt(original_payload.to_vec())
        .unwrap();
    assert_ne!(
        original_payload,
        encrypted_payload.as_slice(),
        "encrypted payload cannot be equal to the clear payload"
    );
    assert_ne!(
        encrypted_payload,
        kem_key
            .public_key()
            .encrypt(original_payload.to_vec())
            .unwrap(),
        "encrypted payload should change with every encryption run"
    );

    let decrypted_payload = kem_key.decrypt(encrypted_payload.clone()).unwrap();
    assert_eq!(
        decrypted_payload, original_payload,
        "decrypted payload should be equal to the original payload"
    );

    let ecc_error = ecc_key.decrypt(encrypted_payload.clone()).unwrap_err();
    assert_eq!(
        ecc_error.to_string(),
        "Invalid payload/private key",
        "attempting to decrypt KEM payload with ECC key should throw an error"
    );

    let hybrid_error = hybrid_key.decrypt(encrypted_payload.clone()).unwrap_err();
    assert_eq!(
        hybrid_error.to_string(),
        "Invalid payload/private key",
        "attempting to decrypt KEM payload with Hybrid key should throw an error"
    );
}

#[test]
fn test_hybrid_encryption_decryption() {
    let (ecc_key, kem_key, hybrid_key) = create_same_seed_key();

    let original_payload = b"Basic Key-Encapsulation Mechanism cryptography test";

    let encrypted_payload = hybrid_key
        .public_key()
        .encrypt(original_payload.to_vec())
        .unwrap();
    assert_ne!(
        original_payload,
        encrypted_payload.as_slice(),
        "encrypted payload cannot be equal to the clear payload"
    );
    assert_ne!(
        encrypted_payload,
        hybrid_key
            .public_key()
            .encrypt(original_payload.to_vec())
            .unwrap(),
        "encrypted payload should change with every encryption run"
    );

    let decrypted_payload = hybrid_key.decrypt(encrypted_payload.clone()).unwrap();
    assert_eq!(
        decrypted_payload, original_payload,
        "decrypted payload should be equal to the original payload"
    );

    let ecc_error = ecc_key.decrypt(encrypted_payload.clone()).unwrap_err();
    assert_eq!(
        ecc_error.to_string(),
        "Invalid payload/private key",
        "attempting to decrypt Hybrid payload with ECC key should throw an error"
    );

    let kem_error = kem_key.decrypt(encrypted_payload.clone()).unwrap_err();
    assert_eq!(
        kem_error.to_string(),
        "Invalid payload/private key",
        "attempting to decrypt Hybrid payload with KEM key should throw an error"
    );
}
