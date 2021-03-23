use std::env::var;

// http server
pub fn bind_address() -> String {
    var("BIND_ADDRESS").unwrap_or("0.0.0.0:8080".into())
}

// database
pub fn postgres_username() -> String {
    var("POSTGRES_USER").unwrap_or("api".into())
}

pub fn postgres_password() -> String {
    var("POSTGRES_PASSWORD").unwrap_or("dev".into())
}

pub fn postgres_host() -> String {
    var("POSTGRES_HOST").unwrap_or("postgres".into())
}

pub fn postgres_port() -> u16 {
    var("POSTGRES_PORT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(5432)
}

pub fn postgres_db() -> String {
    var("POSTGRES_DB").unwrap_or("api".into())
}
