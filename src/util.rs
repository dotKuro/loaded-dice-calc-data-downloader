use std::env;

pub fn assert_env_variable(variable_name: &str) -> String {
    env::var(variable_name)
        .unwrap_or_else(|_| panic!("{} environment variable is not set.", variable_name))
}
