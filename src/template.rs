use std::fs;
use std::{path::Path, time::Instant};
use tera::{Context, Tera};
use tracing::{debug, info};

use crate::error::{Result, TemplateError};

/// Load a multiple template (.tpl) files from directory
pub fn load_templates_from_dir(tera: &mut Tera, dir: &str) -> Result<()> {
    debug!("Loading templates from directory: {}", dir);

    let paths = fs::read_dir(dir).map_err(|e| {
        TemplateError::InvalidArgument(format!("Invalid template directory '{}': {}", dir, e))
    })?;

    let mut count = 0;
    for entry in paths {
        let path = entry?.path();

        if path.extension().and_then(|s| s.to_str()) == Some("tpl") {
            let template_name = path.file_stem().and_then(|s| s.to_str()).ok_or_else(|| {
                TemplateError::InvalidArgument(format!("Invalid filename: {:?}", path))
            })?;

            let template_content = fs::read_to_string(&path)?;
            tera.add_raw_template(template_name, &template_content)?;

            debug!("Loaded template '{}' from {:?}", template_name, path);
            count += 1;
        }
    }

    info!("Loaded {} template(s) from directory '{}'", count, dir);
    Ok(())
}

/// Load a single template file
pub fn load_template_file(tera: &mut Tera, path: &str) -> Result<String> {
    debug!("Loading template file: {}", path);

    let path_obj = Path::new(path);

    if !path_obj.exists() {
        return Err(TemplateError::InvalidArgument(format!(
            "Template file not found: {}",
            path
        )));
    }

    let template_name = path_obj
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| TemplateError::InvalidArgument(format!("Invalid filename: {}", path)))?;

    let template_content = fs::read_to_string(path_obj)?;
    tera.add_raw_template(template_name, &template_content)?;

    info!("Loaded template '{}' from {}", template_name, path);

    Ok(template_name.to_string())
}

/// Parse render argument (TEMPLATE=OUTPUT or just TEMPLATE)
pub fn parse_render_arg(arg: &str) -> Result<(String, Option<String>)> {
    let parts: Vec<&str> = arg.splitn(2, '=').collect();
    let template = parts[0].to_string();
    let output = if parts.len() == 2 {
        Some(parts[1].to_string())
    } else {
        None
    };

    if template.is_empty() {
        return Err(TemplateError::InvalidArgument(
            "Template name cannot be empty".to_string(),
        ));
    }

    Ok((template, output))
}

/// Render certain loaded template and write to file or stdout
pub fn render_template(
    tera: &Tera,
    context: &Context,
    template_name: &str,
    output: Option<&str>,
    started_time: &Instant,
) -> Result<()> {
    debug!("Rendering template: {}", template_name);

    let result = tera.render(template_name, context)?;

    if let Some(output_file) = output {
        debug!("Rendering '{}' to file '{}'", template_name, output_file);
        info!(
            "Processing + rendering took: {:?}",
            Instant::now().duration_since(*started_time)
        );
        fs::write(output_file, result)?;
    } else {
        debug!("Rendering '{}' to stdout", template_name);
        info!(
            "Processing + rendering took: {:?}",
            Instant::now().duration_since(*started_time)
        );
        print!("{}", result);
    }

    Ok(())
}
