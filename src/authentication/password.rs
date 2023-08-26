use argon2::{password_hash::SaltString, Algorithm, Argon2, Params, PasswordHasher, Version};

pub fn compute_password_hash(password: String) -> Result<String, anyhow::Error> {
    let salt = SaltString::generate(&mut rand::thread_rng());
    Ok(Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(15000, 2, 1, None).unwrap(),
    )
    .hash_password(password.as_bytes(), &salt)?
    .to_string())
}
