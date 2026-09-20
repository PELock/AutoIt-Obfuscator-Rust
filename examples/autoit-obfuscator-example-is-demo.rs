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
    let result = my_autoit_obfuscator.login(true).await;

    match result {
        Some(AutoItObfuscatorResponse::Object(obj)) => {
            println!("Demo version status - {}", obj.demo.unwrap_or(false));
            println!("Usage credits left - {:?}", obj.credits_left);
            println!("Total usage credits - {:?}", obj.credits_total);
            println!("Max. script size - {:?}", obj.string_limit);
        }
        Some(AutoItObfuscatorResponse::Json(s)) => println!("{s}"),
        None => {
            return Err("Something unexpected happen while trying to login to the service.".into());
        }
    }

    Ok(())
}
