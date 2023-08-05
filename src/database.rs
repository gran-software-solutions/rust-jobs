use std::env;

const DEFAULT_DB_NAME: &str = "rust_jobs";
const ENV_VAR_DB_HOST: &str = "SURREALDB_HOST";
const ENV_VAR_DB_PORT: &str = "SURREALDB_PORT";
const ENV_VAR_DB_USERNAME: &str = "SURREALDB_USERNAME";
const ENV_VAR_DB_PASSWORD: &str = "SURREALDB_PASSWORD";
const ENV_VAR_DB_NAMESPACE: &str = "SURREALDB_NAMESPACE";
const ENV_VAR_DB_DATABASE: &str = "SURREALDB_DATABASE";
const DEFAULT_DB_PORT: u16 = 8000;
const DEFAULT_DB_NAMESPACE: &str = "apps";

pub struct SurrealdbConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub namespace: String,
    pub database: String,
}

impl SurrealdbConfig {
    pub fn new() -> Self {
        let port = env::var(ENV_VAR_DB_PORT)
            .map_or_else(|_| DEFAULT_DB_PORT, |v| v.parse::<u16>().unwrap());
        let db_host = env::var(ENV_VAR_DB_HOST);
        assert!(db_host.is_ok(), "{} env var is mandatory", ENV_VAR_DB_HOST);
        let db_username = env::var(ENV_VAR_DB_USERNAME);
        assert!(
            db_username.is_ok(),
            "{} env var is mandatory",
            ENV_VAR_DB_USERNAME
        );
        let db_password = env::var(ENV_VAR_DB_PASSWORD);
        assert!(
            db_password.is_ok(),
            "{} env var is mandatory",
            ENV_VAR_DB_PASSWORD
        );
        Self {
            host: db_host.unwrap(),
            port,
            username: db_username.unwrap(),
            password: db_password.unwrap(),
            namespace: env::var(ENV_VAR_DB_NAMESPACE)
                .map_err(|_| DEFAULT_DB_NAMESPACE)
                .unwrap(),
            database: env::var(ENV_VAR_DB_DATABASE)
                .map_err(|_| DEFAULT_DB_NAME)
                .unwrap(),
        }
    }
}
