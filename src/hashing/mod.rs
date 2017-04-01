use ring::pbkdf2;

// Most of this code is taken from the ring doc comments for PBKDF2
// with my own special sauce thrown in.
static PBKDF2_PRF: &'static pbkdf2::PRF = &pbkdf2::HMAC_SHA256;
const CREDENTIAL_LEN: usize = 32;
const ITERATIONS: u32 = 100000;
pub type Credential = [u8; CREDENTIAL_LEN];

fn hash_password_with_salt(password: &str, salt: Vec<u8>)
                           -> Credential {
    let mut hash: Credential = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(PBKDF2_PRF, ITERATIONS, &salt,
                   password.as_bytes(), &mut hash);
    hash
}

fn verify_password(password_hash: Credential, attempted_password: &str, salt: Vec<u8>) -> bool {
    match pbkdf2::verify(PBKDF2_PRF, ITERATIONS, &salt,
                         attempted_password.as_bytes(), &password_hash) {
        Ok(_) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use hashing;

    #[test]
    pub fn hash_test() {
        let password = "swordfish";
        let salt: Vec<u8> = vec![
            // This value was generated from a secure PRNG.
            0xd6, 0x26, 0x98, 0xda, 0xf4, 0xdc, 0x50, 0x52,
            0x24, 0xf2, 0x27, 0xd1, 0xfe, 0x39, 0x01, 0x8a
        ];
        
        let hash: hashing::Credential = hashing::hash_password_with_salt(password, salt);
        let expected_hash: hashing::Credential = [213, 80, 254, 181, 41, 78, 38, 191, 40, 210, 228, 171, 199, 85, 191, 139, 64, 255, 246, 165, 82, 178, 147, 214, 237, 131, 138, 164, 99, 240, 134, 82];
        assert_eq!(hash, expected_hash);
    }

    #[test]
    pub fn verify_test() {
        let password_attempt = "swordfish";
        let salt: Vec<u8> = vec![
            // This value was generated from a secure PRNG.
            0xd6, 0x26, 0x98, 0xda, 0xf4, 0xdc, 0x50, 0x52,
            0x24, 0xf2, 0x27, 0xd1, 0xfe, 0x39, 0x01, 0x8a
        ];
        let actual_hash: hashing::Credential = [213, 80, 254, 181, 41, 78, 38, 191, 40, 210, 228, 171, 199, 85, 191, 139, 64, 255, 246, 165, 82, 178, 147, 214, 237, 131, 138, 164, 99, 240, 134, 82];

        let success = hashing::verify_password(actual_hash, password_attempt, salt);

        assert!(success);
    }
}
