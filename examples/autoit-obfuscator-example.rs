/******************************************************************************
 * AutoIt Obfuscator WebApi interface usage example.
 *
 * Version        : v1.5.0
 * Language       : Rust
 * Author         : Bartosz Wójcik
 * Web page       : https://www.pelock.com
 *
 *****************************************************************************/

use autoit_obfuscator::{AutoItObfuscator, AutoItObfuscatorResponse};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = AutoItObfuscator::new(Some("ABCD-ABCD-ABCD-ABCD".to_string()));
    client.enable_compression = true;
    client.anti_debug = true;
    client.anti_vm = true;
    client.anti_sandbox = true;
    client.anti_emulator = true;
    client.random_integers = true;
    client.random_characters = true;
    client.random_anti_regex = true;
    client.random_arrays = true;
    client.random_arrays_multidimensional = true;
    client.random_functions = true;
    client.random_autostarted = true;
    client.mix_code_flow = true;
    client.rename_variables = true;
    client.rename_functions = true;
    client.rename_function_calls = true;
    client.shuffle_functions = true;
    client.resolve_constants = true;
    client.crypt_numbers = true;
    client.split_strings = true;
    client.modify_strings = true;
    client.crypt_strings = true;
    client.insert_ternary_operators = true;

    let result = client
        .obfuscate_script_source(r#"ConsoleWrite("Hello World")"#, true)
        .await;

    match result {
        Some(AutoItObfuscatorResponse::Object(obj)) => {
            if obj.error == AutoItObfuscator::ERROR_SUCCESS {
                println!("{}", obj.output.unwrap_or_default());
            } else {
                return Err(format!("An error occurred, error code: {}", obj.error).into());
            }
        }
        Some(AutoItObfuscatorResponse::Json(_)) => {}
        None => {
            return Err("Something unexpected happen while trying to obfuscate the code.".into());
        }
    }

    Ok(())
}
