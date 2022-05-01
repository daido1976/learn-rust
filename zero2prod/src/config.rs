use std::env;

pub struct Settings {
    pub database_url: String,
    pub application_port: u16,
}

pub fn get_configuration() -> Settings {
    Settings {
        database_url: env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set environment variable."),
        application_port: 8080,
    }
}
