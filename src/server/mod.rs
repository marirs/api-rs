#![allow(unused_must_use)]
use std::path::Path;
use clap::{crate_authors, crate_version, Clap};
use rocket::{Build, Config, Rocket, data::Limits};

use self::config::Settings;

/// Self Signed certificate generation
mod cert;
/// All required Guards
mod guards;
/// All the Routes/endpoints
mod controllers;
/// Catchers like 500, 501, 404, etc
mod catchers;
/// Server & App Configurations
pub(crate) mod config;

type Error = String;

#[derive(Clap, Debug)]
#[clap(version = crate_version!(), author = crate_authors!())]
struct CliOpts {
    #[clap(short = 'c', long, about = "loads the server configurations")]
    config: Option<String>,
}

/// Parse the settings from the command line arguments
fn parse_settings_from_cli() -> Result<Settings, Error> {
    // parse the cli options
    let cli_opts = CliOpts::parse();
    let cfg_file = &cli_opts.config.unwrap_or_default();
    if cfg_file.is_empty() {
        // No config file, so start
        // with default settings
        Ok(Settings::default())
    } else {
        // Config file passed in cli, check
        // to see if config file exists
        if Path::new(cfg_file).exists() {
            // load settings from the config file or return error
            // if error in loading the given config file
            Settings::from_file(&cfg_file)
        } else {
            // config file does not exist, quit app
            Err(format!("`{}` config file not found!", cfg_file))
        }
    }
}

pub async fn init_server() -> Result<Rocket<Build>, Error> {
    let settings = parse_settings_from_cli()?;

    let limits = Limits::new()
        .limit("forms", settings.server.forms_limit.into())
        .limit("json", settings.server.json_limit.into());

    let rocket_cfg = Config::figment()
        .merge(("address", settings.server.host.to_string()))
        .merge(("port", settings.server.port as u16))
        .merge(("limits", limits))
        .merge(("secret_key", (settings.server.secret_key.as_str())))
        .merge(("keep_alive", settings.server.keep_alive as u32));

    // Configure SSL status for the api server
    let rocket_cfg = if let Some(ssl_cfg) = settings.ssl {
        if ssl_cfg.enabled {
            // ssl is enabled
            if ssl_cfg.pem_certificate.is_some() && ssl_cfg.pem_private_key.is_some() {
                // merge the certs & key into rocket config
                rocket_cfg
                    .merge(("tls.certs", ssl_cfg.pem_certificate))
                    .merge(("tls.key", ssl_cfg.pem_private_key))
            } else {
                // ssl certificate info not available
                return Err("Error getting ssl certificates".to_string())
            }
        } else {
            // ssl not enabled
            rocket_cfg
        }
    } else {
        // no ssl configuration
        rocket_cfg
    };

    // Configure the Rocket server with configured settings
    let app = rocket::custom(rocket_cfg);

    // Catchers
    let app = app.register(
        "/",
        rocket::catchers![
            catchers::bad_request,
            catchers::forbidden,
            catchers::not_authorized,
            catchers::not_found,
            catchers::unprocessed_entity,
            catchers::internal_server_error
        ],
    );

    // Add the routes
    let app = app.mount(
        "/",
        routes![
            controllers::index::hello_world,
            controllers::index::hello_name,
        ],
    );

    // Add Server Settings to state, if in case a need to
    // access the settings from some endpoint
    let app = app.manage(settings.server);

    // Return the configured Rocket App
    Ok(app)
}