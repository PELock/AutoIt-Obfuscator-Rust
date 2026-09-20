/******************************************************************************
 * AutoIt Obfuscator WebApi interface
 *
 * Version        : v1.5.0
 * Language       : Rust
 * Author         : Bartosz Wójcik
 * Web page       : https://www.pelock.com
 *
 *****************************************************************************/

use base64::engine::general_purpose::STANDARD as B64_ENGINE;
use base64::Engine;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{Read, Write};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AutoItObfuscatorResult {
    pub error: i64,
    #[serde(default)]
    pub output: Option<String>,
    #[serde(default)]
    pub demo: Option<bool>,
    #[serde(default, rename = "credits_left")]
    pub credits_left: Option<i64>,
    #[serde(default, rename = "credits_total")]
    pub credits_total: Option<i64>,
    #[serde(default)]
    pub expired: Option<bool>,
    #[serde(default, rename = "string_limit")]
    pub string_limit: Option<i64>,
}

#[derive(Debug, Clone)]
pub enum AutoItObfuscatorResponse {
    Object(AutoItObfuscatorResult),
    Json(String),
}

#[derive(Debug, Clone)]
pub struct AutoItObfuscator {
    api_key: Option<String>,
    client: reqwest::Client,
    pub enable_compression: bool,
    pub anti_debug: bool,
    pub anti_vm: bool,
    pub anti_sandbox: bool,
    pub anti_emulator: bool,
    pub random_integers: bool,
    pub random_characters: bool,
    pub random_anti_regex: bool,
    pub random_arrays: bool,
    pub random_arrays_multidimensional: bool,
    pub random_functions: bool,
    pub random_autostarted: bool,
    pub mix_code_flow: bool,
    pub rename_variables: bool,
    pub rename_functions: bool,
    pub rename_function_calls: bool,
    pub shuffle_functions: bool,
    pub resolve_constants: bool,
    pub crypt_numbers: bool,
    pub split_strings: bool,
    pub modify_strings: bool,
    pub crypt_strings: bool,
    pub insert_ternary_operators: bool,
}

impl AutoItObfuscator {
    pub const API_URL: &'static str = "https://www.pelock.com/api/autoit-obfuscator/v1";
    pub const ERROR_SUCCESS: i64 = 0;
    pub const ERROR_INPUT_SIZE: i64 = 1;
    pub const ERROR_INPUT: i64 = 2;
    pub const ERROR_PARSING: i64 = 3;
    pub const ERROR_OBFUSCATION: i64 = 4;
    pub const ERROR_OUTPUT: i64 = 5;

    pub fn new(api_key: Option<String>) -> Self {
        Self {
            api_key,
            client: reqwest::Client::new(),
            enable_compression: false,
            anti_debug: false,
            anti_vm: false,
            anti_sandbox: false,
            anti_emulator: false,
            random_integers: false,
            random_characters: false,
            random_anti_regex: false,
            random_arrays: false,
            random_arrays_multidimensional: false,
            random_functions: false,
            random_autostarted: false,
            mix_code_flow: false,
            rename_variables: false,
            rename_functions: false,
            rename_function_calls: false,
            shuffle_functions: false,
            resolve_constants: false,
            crypt_numbers: false,
            split_strings: false,
            modify_strings: false,
            crypt_strings: false,
            insert_ternary_operators: false,
        }
    }

    pub async fn login(&self, return_as_object: bool) -> Option<AutoItObfuscatorResponse> {
        let mut params = HashMap::new();
        params.insert("command".to_string(), "login".to_string());
        self.post_request(params, return_as_object).await
    }

    pub async fn obfuscate_script_file(
        &self,
        script_file_path: &std::path::Path,
        return_as_object: bool,
    ) -> Option<AutoItObfuscatorResponse> {
        let source = tokio::fs::read_to_string(script_file_path).await.ok()?;
        if source.is_empty() {
            return None;
        }
        self.obfuscate_script_source(&source, return_as_object).await
    }

    pub async fn obfuscate_script_source(
        &self,
        script_source: &str,
        return_as_object: bool,
    ) -> Option<AutoItObfuscatorResponse> {
        let mut params = HashMap::new();
        params.insert("command".to_string(), "obfuscate".to_string());
        params.insert("source".to_string(), script_source.to_string());
        self.post_request(params, return_as_object).await
    }

    async fn post_request(
        &self,
        mut params: HashMap<String, String>,
        return_as_object: bool,
    ) -> Option<AutoItObfuscatorResponse> {
        if let Some(ref key) = self.api_key {
            if !key.is_empty() {
                params.insert("key".to_string(), key.clone());
            }
        }

        if self.anti_debug {
            params.insert("anti_debug".to_string(), "1".to_string());
        }
        if self.anti_vm {
            params.insert("anti_vm".to_string(), "1".to_string());
        }
        if self.anti_sandbox {
            params.insert("anti_sandbox".to_string(), "1".to_string());
        }
        if self.anti_emulator {
            params.insert("anti_emulator".to_string(), "1".to_string());
        }
        if self.random_integers {
            params.insert("random_bucket_integers".to_string(), "1".to_string());
        }
        if self.random_characters {
            params.insert("random_bucket_characters".to_string(), "1".to_string());
        }
        if self.random_anti_regex {
            params.insert("random_bucket_anti_regex".to_string(), "1".to_string());
        }
        if self.random_arrays {
            params.insert("random_bucket_arrays".to_string(), "1".to_string());
        }
        if self.random_arrays_multidimensional {
            params.insert("random_bucket_arrays_multidimensional".to_string(), "1".to_string());
        }
        if self.random_functions {
            params.insert("random_bucket_functions".to_string(), "1".to_string());
        }
        if self.random_autostarted {
            params.insert("random_bucket_autostart".to_string(), "1".to_string());
        }
        if self.mix_code_flow {
            params.insert("mix_code_flow".to_string(), "1".to_string());
        }
        if self.rename_variables {
            params.insert("rename_variables".to_string(), "1".to_string());
        }
        if self.rename_functions {
            params.insert("rename_functions".to_string(), "1".to_string());
        }
        if self.rename_function_calls {
            params.insert("rename_function_calls".to_string(), "1".to_string());
        }
        if self.shuffle_functions {
            params.insert("shuffle_functions".to_string(), "1".to_string());
        }
        if self.resolve_constants {
            params.insert("resolve_const".to_string(), "1".to_string());
        }
        if self.crypt_numbers {
            params.insert("crypt_numbers".to_string(), "1".to_string());
        }
        if self.split_strings {
            params.insert("split_strings".to_string(), "1".to_string());
        }
        if self.modify_strings {
            params.insert("modify_strings".to_string(), "1".to_string());
        }
        if self.crypt_strings {
            params.insert("crypt_strings".to_string(), "1".to_string());
        }
        if self.insert_ternary_operators {
            params.insert("insert_ternary_operators".to_string(), "1".to_string());
        }

        if self.enable_compression {
            if let Some(source) = params.get("source").cloned() {
                let mut enc = ZlibEncoder::new(Vec::new(), Compression::best());
                enc.write_all(source.as_bytes()).ok()?;
                let compressed = enc.finish().ok()?;
                params.insert("source".to_string(), B64_ENGINE.encode(compressed));
                params.insert("compression".to_string(), "1".to_string());
            }
        }

        let mut form = reqwest::multipart::Form::new();
        for (k, v) in params {
            form = form.text(k, v);
        }

        let response_text = self
            .client
            .post(Self::API_URL)
            .header("User-Agent", "PELock AutoIt Obfuscator")
            .multipart(form)
            .send()
            .await
            .ok()?
            .text()
            .await
            .ok()?;

        if response_text.is_empty() {
            return None;
        }

        let mut result: AutoItObfuscatorResult = serde_json::from_str(&response_text).ok()?;
        let mut depacked = false;

        if self.enable_compression
            && result.error == Self::ERROR_SUCCESS
            && result.output.as_ref().is_some_and(|s| !s.is_empty())
        {
            let b64 = result.output.as_ref()?;
            let buf = B64_ENGINE.decode(b64.as_bytes()).ok()?;
            let mut decoder = ZlibDecoder::new(&buf[..]);
            let mut out = String::new();
            decoder.read_to_string(&mut out).ok()?;
            result.output = Some(out);
            depacked = true;
        }

        if return_as_object {
            return Some(AutoItObfuscatorResponse::Object(result));
        }

        if depacked {
            let s = serde_json::to_string(&result).ok()?;
            return Some(AutoItObfuscatorResponse::Json(s));
        }

        Some(AutoItObfuscatorResponse::Json(response_text))
    }
}
