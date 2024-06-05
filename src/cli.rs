use anyhow::{anyhow, Result};
use clap::{Args, Parser, ValueEnum};
use std::net::SocketAddr;
use std::str::FromStr;
use tor_client_lib::control_connection::TorSocketAddr;
use voynich::config::{Config, TorAuthConfig};
use voynich::onion_service::OnionType;

static SHORT_HELP: &str = "Voynich-term - Anonymous, end-to-end encrypted chat";
static LONG_HELP: &str = "Voynich-term - Anonymous, end-to-end encrypted chat

Uses Tor Onion Services to provide anonymization and NAT traversal.

The onion services it uses come in two types, persistent and transient.

For instance, to create a persistent onion service for your chat session, you could do:

    % voynich-term --create --name my_onion_service --service-port 3000

Thereafter, you can reuse that service like so:

    % voynich-term --name my_onion_service

If you want a transient service that only lasts for the current session:

    % voynich-term --transient --service-port 3000";

#[derive(Debug, Parser)]
#[command(author, version, about = SHORT_HELP, long_about = LONG_HELP)]
pub struct Cli {
    /// Tor control address - default is 127.0.0.1:9051
    #[arg(long, value_name = "ADDRESS", default_value_t = SocketAddr::from_str("127.0.0.1:9051").unwrap())]
    pub tor_address: SocketAddr,

    /// Tor proxy address - default is 127.0.0.1:9050
    #[arg(long, value_name = "ADDRESS", default_value_t = SocketAddr::from_str("127.0.0.1:9050").unwrap())]
    pub tor_proxy_address: SocketAddr,

    /// Listen address to use for onion service
    /// Default is "127.0.0.1:<service-port>"
    #[arg(long, value_name = "LOCAL-ADDRESS")]
    pub listen_address: Option<TorSocketAddr>,

    /// Service port to use for the transient or newly created persistent onion service
    #[arg(long, required_if_eq_any([("onion_type", "transient"), ("create", "true")]))]
    pub service_port: Option<u16>,

    /// Tor Authentication Arguments
    #[command(flatten)]
    pub auth_args: AuthArgs,

    /// Don't run connection test on startup (by default, it will run the test)
    #[arg(long, default_value_t = false)]
    pub no_connection_test: bool,

    /// Use debug logging
    #[arg(short, long, default_value_t = false)]
    pub debug: bool,

    /// Type of the onion service
    #[arg(short, long, value_enum)]
    pub onion_type: OnionServiceType,

    /// Create the onion service.
    /// Ignored if --onion-type is "transient"
    #[arg(long, default_value_t = false)]
    pub create: bool,

    /// Name of the onion service. Ignored if onion type is "transient"
    ///
    /// If --create is specified, saves the created service under that name.
    ///
    /// If not, it tries to look up a saved onion service by that name
    #[arg(short, long)]
    pub name: Option<String>,
}

#[derive(Args, Clone, Debug)]
#[group(required = false, multiple = false)]
pub struct AuthArgs {
    /// Tor service authentication. Set the value of the cookie; no value means use the cookie from the cookie file
    #[arg(long = "safe-cookie")]
    safe_cookie: Option<Option<String>>,

    /// Tor service authentication. Set the value of the password; no value means prompt for the password
    #[arg(long = "hashed-password")]
    hashed_password: Option<Option<String>>,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum OnionServiceType {
    Transient,
    Persistent,
}

impl Cli {
    pub fn get_onion_type(&self) -> Result<OnionType> {
        match self.onion_type {
            OnionServiceType::Persistent => match self.name {
                Some(ref name) => {
                    if self.create {
                        Ok(OnionType::new_persistent(name))
                    } else {
                        Ok(OnionType::existing_persistent(name))
                    }
                }
                None => Err(anyhow!("Must specify --name with --persistent")),
            },
            OnionServiceType::Transient => Ok(OnionType::new_transient()),
        }
    }
}

impl From<&Cli> for Config {
    fn from(cli: &Cli) -> Config {
        let mut config = Config::default();
        config.system.debug = cli.debug;
        config.system.connection_test = !cli.no_connection_test;
        config.tor.proxy_address = cli.tor_proxy_address;
        config.tor.control_address = cli.tor_address;
        match &cli.auth_args {
            AuthArgs {
                safe_cookie: None,
                hashed_password: None,
            } => {}
            AuthArgs {
                safe_cookie: Some(None),
                hashed_password: None,
            } => {
                config.tor.authentication = Some(TorAuthConfig::SafeCookie);
            }
            AuthArgs {
                safe_cookie: Some(Some(cookie)),
                hashed_password: None,
            } => {
                config.tor.authentication = Some(TorAuthConfig::SafeCookie);
                config.tor.cookie = Some(cookie.as_bytes().to_vec());
            }
            AuthArgs {
                safe_cookie: None,
                hashed_password: Some(None),
            } => {
                config.tor.authentication = Some(TorAuthConfig::HashedPassword);
            }
            AuthArgs {
                safe_cookie: None,
                hashed_password: Some(Some(password)),
            } => {
                config.tor.authentication = Some(TorAuthConfig::HashedPassword);
                config.tor.hashed_password = Some(password.clone());
            }
            _ => {
                unreachable!()
            }
        }

        config
    }
}
