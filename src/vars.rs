use serde_json::Value;
use std::env;
use std::fs;
use tera::Context;
use tracing::{debug, info};

use crate::error::Result;
use crate::error::TemplateError;

/// Load values from a JSON file
pub fn load_values_from_file(path: &str) -> Result<Context> {
    debug!("Loading values from file: {}", path);

    let content = fs::read_to_string(path)?;
    let json: Value = serde_json::from_str(&content)?;
    let mut context = Context::new();
    let mut ctr: u32 = 0;

    match json {
        Value::Object(map) => {
            map.into_iter()
                .map(|item| {
                    context.insert(item.0, &item.1);
                    ctr = ctr.saturating_add(1);
                })
                .count();
        }
        _ => {
            return Err(TemplateError::InvalidArgument(
                "JSON file must contain an object at root level".to_string(),
            ));
        }
    }

    info!("Loaded {} value(s) from file '{}'", ctr, path);
    Ok(context)
}

/// Add environment variables to context with optional filtering
pub fn add_env_vars(context: &mut Context, prefix_filter: Option<&str>) {
    for (key, value) in env::vars() {
        // Only include env vars matching the prefix filter (for security)
        if let Some(prefix) = prefix_filter
            && !key.starts_with(prefix)
        {
            continue;
        }

        context.insert(format!("ENV_{}", key), &Value::String(value));

        #[cfg(debug_assertions)]
        debug!("Added env var: {}", key);
    }
}

/// Parse a key=value string into a tuple
pub fn parse_key_value(kv: &str) -> Result<(String, Value)> {
    let parts: Vec<&str> = kv.splitn(2, '=').collect();

    if parts.len() != 2 {
        return Err(TemplateError::InvalidArgument(format!(
            "Invalid key=value format: '{}'. Expected format: KEY=VALUE",
            kv
        )));
    }

    let key = parts[0].trim();
    let value_str = parts[1];

    if key.is_empty() {
        return Err(TemplateError::InvalidArgument(
            "Key cannot be empty in key=value pair".to_string(),
        ));
    }

    // Try to parse as JSON, fall back to string
    let value =
        serde_json::from_str(value_str).unwrap_or_else(|_| Value::String(value_str.to_string()));

    debug!("Set variable: {} = {:?}", key, value);

    Ok((key.to_string(), value))
}
