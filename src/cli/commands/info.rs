use crate::libs::satellite_search_service::Satellite;
use crate::{cli::utils, libs::satellite_position_service};
use dialoguer::{theme::ColorfulTheme, Select};
use std::process;

use super::map::map_ui;

pub fn info_ui(satellite: &Satellite) {
    let pb = utils::display_spinner("Fetching satellite position".to_string(), None);
    let (lat, lon) = satellite_position_service::get_satellite_position(satellite.id);

    pb.finish_and_clear();
    clearscreen::clear().unwrap();

    utils::print_satellite_info(&satellite, lat, lon);

    let items = &["View Map", "Return"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .default(0)
        .items(&items[..])
        .interact()
        .unwrap_or_else(|_| {
            eprintln!("Failed to interact with user");
            process::exit(1);
        });

    match selection {
        0 => map_ui(satellite.id, lat, lon), // Assuming map_ui is a function that takes a satellite and displays its map
        1 => return,
        _ => unreachable!(),
    }
}
