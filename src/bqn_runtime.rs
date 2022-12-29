use std::env;

use cbqn::{BQNValue, eval};

pub fn build_runtime() -> BQNValue {
    let current_directory = env::current_dir().unwrap();
    let current_directory_str = current_directory.to_str().unwrap();

    eval(
        &(
            "⟨ \"".to_string() +
            current_directory_str + "/\", ⟨⟩, ⟨⟩⟩ •BQN •FChars \"" +
            current_directory_str + "/main.bqn\""
        )
    )
}
