use clap::CommandFactory;
use colored::Colorize;
use keyring::Entry;
use log::{debug, error, info, warn};

use crate::cli::{Cli, Commands};

pub const KEYRING_SERVICE: &str = "envchain";

pub fn open_keyring(name: &str) -> Entry {
  debug!("Opening keyring {KEYRING_SERVICE}");

  Entry::new(KEYRING_SERVICE, name)
    .unwrap_or_else(|_| panic!("Unable to load keyring {:?} for value {:?}", KEYRING_SERVICE, &name))
}

#[cfg(target_os = "macos")]
fn check() -> Result<(), String> {
  use security_framework::os::macos::keychain::{SecKeychain, SecPreferencesDomain};
  use terminal_prompt::Terminal;

  let user_interaction_allowed = SecKeychain::user_interaction_allowed().unwrap();
  info!("User interaction allowed: {:?}", user_interaction_allowed);
  let mut keychain = SecKeychain::default_for_domain(SecPreferencesDomain::User).unwrap();
  match keychain.unlock(None) {
    Ok(_) => {
      info!("keychain unlocked with default value");
    },
    Err(err) => {
      warn!("unable to unlock keychain {:?}, asking user the password to unlock it", err);
      let mut terminal = match Terminal::open() {
        Ok(it) => it,
        Err(err) => return Err(err.to_string()),
      };

      let password = terminal.prompt_sensitive("Enter keychain password: ").unwrap();
      match keychain.unlock(Some(&password)) {
        Ok(_) => {
          info!("keychain unlocked");
        },
        Err(err) => {
          error!("unable to unlock keychain {:?}, exiting", err);
          return Err(err.to_string());
        },
      }
    },
  }

  Ok(())
}

#[cfg(target_os = "macos")]
const MACOS_USER_INTERACTION_NOT_ALLOWED: i32 = -25308;

/// Main handler for commands
pub fn handle_command(command: &Commands) -> Result<String, String> {
  #[cfg(target_os = "macos")]
  check()?;

  match command {
    Commands::Completions { shell } => {
      let mut value = Vec::new();
      shell.generate(&mut Cli::command(), &mut value);

      Ok(String::from_utf8(value).expect("Unable to generate completions"))
    },
    Commands::Get { name } => {
      info!("Getting credential {name}");
      let entry = open_keyring(name);
      debug!("Getting password {name}");
      match entry.get_password() {
        Ok(password) => Ok(password),
        Err(error) => {
          #[cfg(target_os = "macos")]
          if let keyring::Error::PlatformFailure(err) = error {
            if let Some(e) = err.downcast_ref::<security_framework::base::Error>() {
              if e.code() == MACOS_USER_INTERACTION_NOT_ALLOWED {
                error!("User interaction is not allowed, please unlock the keychain first");
                return Err(format!("Unable to get entry {:?}: User interaction are disabled", name));
              }
            }
            debug!("Unable to get password {:?} {:?}", name, err);
          }
          warn!("Entry {:?} doesn't exists", name);
          Err(format!("Entry {:?} doesn't exists", name))
        },
      }
    },
    Commands::Set { name, value, force } => {
      info!("Setting credential {name}");
      let entry = open_keyring(name);

      let current_value = entry.get_password();
      if *force {
        info!("Overriding current credential value");
        eprintln!("{}: overriding existing value in {}", "Warning".yellow(), name);
        if current_value.is_ok() {
          info!("Deleting existing value for {name}");
          if entry.delete_password().is_err() {
            return Err(format!("Unable to delete password {:?}", name));
          };
        }
      } else if current_value.is_ok() {
        error!("There is already a value for entry {name}");
        return Err(format!("Entry {:?} already exists. Please use force parameter to override it.", name));
      }

      if let Err(err) = entry.set_password(value) {
        error!("Unable to set password {:?} {:?}", name, err);
        Err(format!("Unable to set password {:?}", name))
      } else {
        Ok(format!("Password {:?} set", name))
      }
    },
    Commands::Delete { name } => {
      info!("Deleting credential {name}");
      let entry = open_keyring(name);

      if let Err(err) = entry.delete_password() {
        error!("Unable to delete password {:?} {:?}", name, err);
        Err(format!("Unable to delete password {:?}", name))
      } else {
        Ok(format!("Password {:?} deleted", name))
      }
    },
  }
}
