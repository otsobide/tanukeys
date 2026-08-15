use kernel::crypto_keys::domain::value_objects::crypto_key_algorithm::CryptoKeyAlgorithm;
use kernel::crypto_keys::domain::value_objects::crypto_key_kind::CryptoKeyKind;

#[test]
fn it_parses_every_canonical_algorithm() {
    for canonical in [
        "rsa", "ed25519", "ecdsa", "x25519", "aes256", "chacha20", "hmac",
    ] {
        let algorithm = CryptoKeyAlgorithm::new(canonical).unwrap();
        assert_eq!(algorithm.as_str(), canonical);
    }
}

#[test]
fn it_matches_case_insensitively() {
    assert_eq!(
        CryptoKeyAlgorithm::new("RSA").unwrap(),
        CryptoKeyAlgorithm::Rsa
    );
    assert_eq!(
        CryptoKeyAlgorithm::new("Ed25519").unwrap(),
        CryptoKeyAlgorithm::Ed25519
    );
}

#[test]
fn it_rejects_an_unknown_algorithm() {
    assert!(CryptoKeyAlgorithm::new("des").is_err());
}

#[test]
fn it_rejects_an_empty_algorithm() {
    assert!(CryptoKeyAlgorithm::new("").is_err());
}

#[test]
fn it_derives_the_asymmetric_kind_for_public_key_algorithms() {
    for raw in ["rsa", "ed25519", "ecdsa", "x25519"] {
        let algorithm = CryptoKeyAlgorithm::new(raw).unwrap();
        assert_eq!(algorithm.kind(), CryptoKeyKind::Asymmetric);
    }
}

#[test]
fn it_derives_the_symmetric_kind_for_secret_key_algorithms() {
    for raw in ["aes256", "chacha20", "hmac"] {
        let algorithm = CryptoKeyAlgorithm::new(raw).unwrap();
        assert_eq!(algorithm.kind(), CryptoKeyKind::Symmetric);
    }
}
