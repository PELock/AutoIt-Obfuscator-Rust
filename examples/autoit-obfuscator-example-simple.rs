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
    let my_autoit_obfuscator = AutoItObfuscator::new(Some("ABCD-ABCD-ABCD-ABCD".to_string()));
    let result = my_autoit_obfuscator
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
