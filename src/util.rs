use std::env;

pub fn assert_env_variable(variable_name: &str) -> String {
    env::var(variable_name)
        .expect(format!("{} environment variable is not set.", variable_name).as_str())
}