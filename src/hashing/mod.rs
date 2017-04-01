use ring::pbkdf2;

// Most of this code is taken from the ring doc comments for PBKDF2
// with my own special sauce thrown in.
static PBKDF2_PRF: &'static pbkdf2::PRF = &pbkdf2::HMAC_SHA256;
const CREDENTIAL_LEN: usize = 32;
const iterations: u32 = 100000;
pub type Credential = [u8; CREDENTIAL_LEN];

fn hash_password_with_salt(password: &str, salt: Vec<u8>)
                           -> Credential {
    let mut hash: Credential = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(PBKDF2_PRF, iterations, &salt,
                   password.as_bytes(), &mut hash);
    hash
}

fn verify_password(password_hash: Credential, attempted_password: &str, salt: Vec<u8>) -> bool {
    match pbkdf2::verify(PBKDF2_PRF, iterations, &salt,
                         attempted_password.as_bytes(), &password_hash) {
        Ok(_) => true,
        _ => false,
    }
}
