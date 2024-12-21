pub mod env {
    use dotenv::dotenv;
    use lazy_static::lazy_static;
    use serde::Deserialize;

    lazy_static! {
        pub static ref ENV: Env = Env::new();
    }

    #[derive(Clone, Deserialize, Debug)]
    pub struct Env {
        pub port: u16,
        pub host: String,
        pub jwt_secret: String,
        pub database_url: String,
    }

    impl Env {
        fn new() -> Env {
            dotenv().ok();

            match envy::from_env::<Env>() {
                Ok(env) => env,
                Err(e) => panic!("Unable to parse environment variables {:?}", e),
            }
        }
    }
}