use anyhow::Result;
use serde_json::{from_reader, to_string_pretty, Error};
use std::fs::File;
use std::io::Write;

use crate::structs::AppState;

pub fn get_stored_app() -> AppState {
    let file = File::open("app_state.json");
    match file {
        Ok(file) => {
            let app_state_result: Result<AppState, Error> = from_reader(file);
            match app_state_result {
                Ok(app_state) => app_state,
                Err(_) => {
                    let default_state = AppState::default();
                    if let Err(e) = store_app(&default_state) {
                        eprintln!("Could not store app state: {:?}", e);
                    }
                    default_state
                }
            }
        }
        Err(_) => {
            let default_state = AppState::default();
            if let Err(e) = store_app(&default_state) {
                eprintln!("Could not store app state: {:?}", e);
            }
            default_state
        }
    }
}

pub fn store_app(app_state: &AppState) -> Result<()> {
    let app_state_json = to_string_pretty(app_state)?;
    let mut file = File::create("app_state.json")?;
    file.write_all(app_state_json.as_bytes())?;
    Ok(())
}
