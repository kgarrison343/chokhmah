use ring::pbkdf2;

// Most of this code is taken from the ring doc comments for PBKDF2
// with my own special sauce thrown in.
static PBKDF2_PRF: &'static pbkdf2::PRF = &pbkdf2::HMAC_SHA256;
const CREDENTIAL_LEN: usize = 32;
const ITERATIONS: u32 = 100000;
pub type Credential = [u8; CREDENTIAL_LEN];

pub fn hash_password(username: &str, password: &str) -> Credential {
    let salt = generate_salt(username);
    hash_password_with_salt(password, salt)
}

pub fn verify_password(password_hash:Credential, username: &str, attempted_password: &str) -> bool {
    let salt = generate_salt(username);
    verify_password_with_salt(password_hash, attempted_password, salt)
}

fn generate_salt(username: &str) -> &[u8] {
    // TODO: Improve salt generation.
    username.as_bytes()
}

fn hash_password_with_salt(password: &str, salt: &[u8])
                           -> Credential {
    let mut hash: Credential = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(PBKDF2_PRF, ITERATIONS, salt,
                   password.as_bytes(), &mut hash);
    hash
}

fn verify_password_with_salt(password_hash: Credential, attempted_password: &str, salt: &[u8]) -> bool {
    match pbkdf2::verify(PBKDF2_PRF, ITERATIONS, salt,
                         attempted_password.as_bytes(), &password_hash) {
        Ok(_) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use hashing;

    #[test]
    pub fn hash_with_salt_test() {
        let password = "swordfish";
        let salt: Vec<u8> = vec![
            // This value was generated from a secure PRNG.
            0xd6, 0x26, 0x98, 0xda, 0xf4, 0xdc, 0x50, 0x52,
            0x24, 0xf2, 0x27, 0xd1, 0xfe, 0x39, 0x01, 0x8a
        ];
        
        let hash: hashing::Credential = hashing::hash_password_with_salt(password, salt.as_slice());
        let expected_hash: hashing::Credential = [213, 80, 254, 181, 41, 78, 38, 191, 40, 210, 228, 171, 199, 85, 191, 139, 64, 255, 246, 165, 82, 178, 147, 214, 237, 131, 138, 164, 99, 240, 134, 82];
        assert_eq!(hash, expected_hash);
    }

    #[test]
    pub fn verify_with_salt_test() {
        let password_attempt = "swordfish";
        let salt: Vec<u8> = vec![
            // This value was generated from a secure PRNG,
            // and then I stole it from the ring docs.
            0xd6, 0x26, 0x98, 0xda, 0xf4, 0xdc, 0x50, 0x52,
            0x24, 0xf2, 0x27, 0xd1, 0xfe, 0x39, 0x01, 0x8a
        ];
        let actual_hash: hashing::Credential = [213, 80, 254, 181, 41, 78, 38, 191, 40, 210, 228, 171, 199, 85, 191, 139, 64, 255, 246, 165, 82, 178, 147, 214, 237, 131, 138, 164, 99, 240, 134, 82];

        let success = hashing::verify_password_with_salt(actual_hash, password_attempt, salt.as_slice());

        assert!(success);
    }

    #[test]
    pub fn generate_salt_test() {
        let username = "IamGroot";
        let salt = username.as_bytes();

        let actual_salt = hashing::generate_salt(username);

        assert_eq!(actual_salt, salt);
    }

    #[test]
    pub fn hash_test() {
        let username = "IamGroot";
        let password = "swordfish";

        let hash: hashing::Credential = [52, 80, 203, 151, 46, 207, 204, 13, 57, 63, 244, 102, 59, 111, 4, 119, 213, 24, 100, 100, 14, 22, 95, 56, 8, 139, 216, 66, 24, 48, 254, 22];
        let actual_hash = hashing::hash_password(username, password);

        assert_eq!(hash, actual_hash);
    }

    #[test]
    pub fn verify_test() {
        let username = "IamGroot";
        let password = "swordfish";

        let actual_hash: hashing::Credential = [52, 80, 203, 151, 46, 207, 204, 13, 57, 63, 244, 102, 59, 111, 4, 119, 213, 24, 100, 100, 14, 22, 95, 56, 8, 139, 216, 66, 24, 48, 254, 22];

        let success = hashing::verify_password(actual_hash, username, password);
        assert!(success);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    pub fn hash_test_salts_change_password_hash() {
        let username1 = "IamGroot";
        let username2 = "IamHroot";
        let password ="swordfish";

        let hash1 = hashing::hash_password(username1, password);
        let hash2 = hashing::hash_password(username2, password);

        assert_eq!(hash1, hash2);
    }
}
