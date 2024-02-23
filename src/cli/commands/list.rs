use std::process;

use dialoguer::{console::Term, theme::ColorfulTheme, Select};

use crate::{
    cli::utils,
    libs::satellite_search_service::{self, Satellite},
};

use super::info::info_ui;

pub fn list_ui() {
    let mut current_page: i32 = 1;
    let mut num_pages: i32 = 1;
    let mut cache: std::collections::HashMap<i32, Vec<Satellite>> =
        std::collections::HashMap::new();

    loop {
        let pb = utils::display_spinner("Searching...".to_string(), None);

        let satellites;
        if let Some(cached) = cache.get(&(current_page)) {
            satellites = cached;
        } else {
            // Call the API to fetch results and total pages
            let fetched_result =
                satellite_search_service::get_satellites(None, Some(current_page), None);
            num_pages = fetched_result.1;
            cache.insert(current_page, fetched_result.0);
            satellites = cache.get(&current_page).unwrap();
        }

        pb.finish_and_clear();
        clearscreen::clear().unwrap();

        if satellites.is_empty() {
            eprintln!("No satellites available.");
            return;
        }

        let mut selections: Vec<String> = satellites.iter().map(|s| s.name.clone()).collect();

        if current_page > 1 {
            selections.push("Previous".to_string());
        }
        if current_page < num_pages {
            selections.push("Load More".to_string());
        }
        selections.push("Cancel".to_string());

        let theme = ColorfulTheme {
            ..ColorfulTheme::default()
        };

        let selection = Select::with_theme(&theme)
            .with_prompt("Select a satellite to get more information")
            .default(0)
            .items(&selections)
            .interact_on_opt(&Term::stderr());

        match selection {
            Ok(Some(index)) => {
                let is_previous = index == selections.len() - 3 && current_page > 1;
                let is_load_more = index == selections.len() - 2 && current_page < num_pages;
                let is_cancel = index == selections.len() - 1;

                if is_load_more {
                    current_page += 1;
                } else if is_previous {
                    current_page -= 1;
                } else {
                    if is_cancel {
                        return;
                    }

                    if current_page > 1 && !is_previous && !is_load_more && !is_cancel {
                        let satellite = &satellites[index - 1];

                        info_ui(satellite)
                    } else {
                        let satellite = &satellites[index];

                        info_ui(satellite)
                    };
                }
            }
            Ok(None) => {
                return;
            }
            Err(_) => {
                eprintln!("An error occurred during the selection.");
                process::exit(1);
            }
        };
    }
}
