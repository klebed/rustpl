//! # rustpl
//!
//! Template compiler written in 🦀🦀🦀 Rust, so it's blazingly🔥🔥🔥 fast 🚀🚀🚀 and memory safe!  It may use args, env and json files as input sets of keys/values.
//!
//! It may be used as generator for:
//! 1) Declarative/conditional config files
//! 2) Static pages
//! 3) Data reports MD/TXT/etc.
//!
//! It uses Tera template engine, which allows to use nesting/inheritance of templates for better complex workloads.
//!
//! Example usage:
//! ```bash
//!   rustpl --template samples/main.tpl --template samples/header.tpl\
//!    --set key=value --render main=/path/filename.txt
//! ```

//! ```jinja2
//! {# main.tpl #}
//! {% include "header" %}
//! You should see value of **key** here: {{ key }}
//! ```

//! ```jinja2
//! {# header.tpl #}
//! Hello, rustacean!
//! This is rustpl output!
//!
//! ```
//!
//! This would produce following output in /path/filename.txt :
//! ```text
//! Hello, rustacean!
//! This is rustpl output!
//!
//! You should see value of **key** here: value    
//! ```
//!
//!
//! For more advanced guidance on template format, read Tera documentation here: https://keats.github.io/tera/docs/
//!
//! Full CLI arguments description:
//! ```bash
//! #rustpl --help
//! Usage: rustpl [OPTIONS] --render <TEMPLATE=OUTPUT> --template <TEMPLATE_FILE>
//!
//! Options:
//!   -r, --render <TEMPLATE=OUTPUT>  Sets the name of the template to render
//!   -t, --template <TEMPLATE_FILE>  Specifies the template file to use
//!   -d, --template-dir <DIR>        Specifies the directory containing templates
//!   -s, --set <KEY=VALUE>           Sets a variable for the document. You should escape quotes and doublequotes unless you're passing just a string
//!   -j, --values <FILE>             Path to a JSON file containing values
//!   -E, --import-env [<PREFIX>]     Import environment variables (optionally filtered by PREFIX, e.g., 'APP_'). WARNING: May expose secrets!
//!   -v, --verbose                   Enable verbose logging (debug level)
//!   -h, --help                      Print help
//!   -V, --version                   Print version
//! ```
use clap::{Arg, Command};
use std::time::Instant;
use tera::Context;
use tera::Tera;
use tracing::{Level, debug, info, warn};
use tracing_subscriber::FmtSubscriber;

use crate::error::Result;
use crate::error::TemplateError;
use crate::template::load_template_file;
use crate::template::load_templates_from_dir;
use crate::template::parse_render_arg;
use crate::template::render_template;
use crate::vars::add_env_vars;
use crate::vars::load_values_from_file;
use crate::vars::parse_key_value;

mod error;
mod template;
mod vars;

fn main() -> Result<()> {
    let started_time = Instant::now();
    let matches = Command::new("rustpl")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION").to_string())
        .arg(
            Arg::new("render")
                .short('r')
                .long("render")
                .value_name("TEMPLATE=OUTPUT")
                .help("Sets the name of the template to render")
                .required(true),
        )
        .arg(
            Arg::new("template")
                .short('t')
                .long("template")
                .value_name("TEMPLATE_FILE")
                .help("Specifies the template file to use")
                .required(true)
                .action(clap::ArgAction::Append),
        )
        .arg(
            Arg::new("template_dir")
                .short('d')
                .long("template-dir")
                .value_name("DIR")
                .help("Specifies the directory containing templates")
                .required(false),
        )
        .arg(
            Arg::new("set")
                .short('s')
                .long("set")
                .value_name("KEY=VALUE")
                .help("Sets a variable for the document. You should escape quotes and doublequotes unless you're passing just a string")
                .required(false)
                .action(clap::ArgAction::Append),
        )
        .arg(
            Arg::new("values")
                .short('j')
                .long("values")
                .value_name("FILE")
                .help("Path to a JSON file containing values")
                .required(false),
        )
        .arg(
            Arg::new("import_env")
                .short('E')
                .long("import-env")
                .value_name("PREFIX")
                .help("Import environment variables (optionally filtered by PREFIX, e.g., 'APP_'). WARNING: May expose secrets!")
                .required(false)
                .num_args(0..=1)
                .default_missing_value(""),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enable verbose logging (debug level)")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    // Initialize tracing
    let level = if matches.get_flag("verbose") {
        Level::DEBUG
    } else {
        Level::ERROR
    };

    let subscriber = FmtSubscriber::builder()
        .with_max_level(level)
        .with_target(false)
        .without_time()
        .with_writer(std::io::stderr)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .map_err(|e| TemplateError::InvalidArgument(format!("Failed to set logger: {}", e)))?;

    debug!("Starting rustpl v{}", env!("CARGO_PKG_VERSION"));

    debug!("Import-env: {:?}", matches.get_one::<String>("import_env"));

    let mut tera = Tera::default();
    let mut data = Context::new();

    // Load templates from directory
    if let Some(template_dir) = matches.get_one::<String>("template_dir") {
        load_templates_from_dir(&mut tera, template_dir)?;
    }

    // Load individual template files
    if let Some(template_files) = matches.get_many::<String>("template") {
        for template_file in template_files {
            load_template_file(&mut tera, template_file)?;
        }
    }

    // Load values from JSON file
    if let Some(values_file) = matches.get_one::<String>("values") {
        let file_values = load_values_from_file(values_file)?;
        data.extend(file_values);
    }

    // Parse --set arguments
    if let Some(sets) = matches.get_many::<String>("set") {
        for set in sets {
            let (key, value) = parse_key_value(set)?;
            data.insert(key, &value);
        }
    }

    // Add environment variables (with optional filtering)
    // Import environment variables ONLY if explicitly requested (OPT-IN)
    if let Some(env_prefix) = matches.get_one::<String>("import_env") {
        let prefix = if env_prefix.is_empty() {
            warn!("⚠️  Importing ALL environment variables - this may expose secrets!");
            None
        } else {
            info!(
                "Importing environment variables with prefix: {}",
                env_prefix
            );
            Some(env_prefix.as_str())
        };
        add_env_vars(&mut data, prefix);
    } else {
        debug!("Environment variables not imported (use --import-env to enable)");
    }

    // Render templates
    if let Some(renderings) = matches.get_many::<String>("render") {
        for rendering in renderings {
            let (template_name, output) = parse_render_arg(rendering)?;
            render_template(
                &tera,
                &data,
                &template_name,
                output.as_deref(),
                &started_time,
            )?;
        }
    }

    Ok(())
}
