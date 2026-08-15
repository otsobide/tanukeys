use kernel::crypto_keys::domain::value_objects::crypto_key_protocol::CryptoKeyProtocol;

#[test]
fn it_parses_every_canonical_protocol() {
    for canonical in ["openpgp", "ssh", "x509", "raw"] {
        let protocol = CryptoKeyProtocol::new(canonical).unwrap();
        assert_eq!(protocol.as_str(), canonical);
    }
}

#[test]
fn it_accepts_pgp_as_an_alias_for_openpgp() {
    let protocol = CryptoKeyProtocol::new("pgp").unwrap();
    assert_eq!(protocol, CryptoKeyProtocol::OpenPgp);
    assert_eq!(protocol.as_str(), "openpgp");
}

#[test]
fn it_accepts_x_dot_509_as_an_alias_for_x509() {
    let protocol = CryptoKeyProtocol::new("x.509").unwrap();
    assert_eq!(protocol, CryptoKeyProtocol::X509);
    assert_eq!(protocol.as_str(), "x509");
}

#[test]
fn it_matches_case_insensitively() {
    assert_eq!(
        CryptoKeyProtocol::new("OpenPGP").unwrap(),
        CryptoKeyProtocol::OpenPgp
    );
    assert_eq!(
        CryptoKeyProtocol::new("SSH").unwrap(),
        CryptoKeyProtocol::Ssh
    );
}

#[test]
fn it_rejects_an_unknown_protocol() {
    assert!(CryptoKeyProtocol::new("jwk").is_err());
}

#[test]
fn it_rejects_an_empty_protocol() {
    assert!(CryptoKeyProtocol::new("").is_err());
}
