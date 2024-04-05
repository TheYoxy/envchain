use colored::Colorize;
use keyring::Entry;
use log::{debug, error, info, warn};

use crate::cli::Commands;

pub const KEYRING_SERVICE: &str = "envchain";

pub fn open_keyring(name: &str) -> Entry {
  debug!("Opening keyring {KEYRING_SERVICE}");

  Entry::new(KEYRING_SERVICE, name)
    .unwrap_or_else(|_| panic!("Unable to load keyring {:?} for value {:?}", KEYRING_SERVICE, &name))
}

/// Main handler for commands
pub fn handle_command(command: &Commands) -> Result<String, String> {
  match command {
    Commands::Get { name } => {
      info!("Getting credential {name}");
      let entry = open_keyring(name);
      debug!("Getting password {name}");
      if let Ok(password) = entry.get_password() {
        Ok(password)
      } else {
        warn!("Entry {:?} doesn't exists", name);
        Err(format!("Entry {:?} doesn't exists", name))
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
