use std::collections::HashMap;
use std::process;

use dialoguer::{console::Term, theme::ColorfulTheme, Input, Select};

use crate::cli::utils;
use crate::libs::satellite_search_service::{get_satellites, Satellite};

use super::info::info_ui;

pub fn search_ui() {
    let search_query: String = Input::new()
        .with_prompt("Enter the name of the satellite to search for")
        .interact_text()
        .unwrap();

    let mut current_page = 1;
    let page_size = 10; // Define how many items you want per page
    let mut cache: HashMap<i32, Vec<Satellite>> = HashMap::new();
    let mut total_pages: i32 = 1; // Assuming total_pages can be determined similarly

    loop {
        let pb = utils::display_spinner("Searching...".to_string(), None);

        let satellites;
        if let Some(cached) = cache.get(&current_page) {
            satellites = cached.clone(); // Clone the data from cache
        } else {
            // Call the API to fetch results with pagination
            let fetched_result = get_satellites(
                Some(search_query.as_str()),
                Some(current_page),
                Some(page_size),
            );
            total_pages = fetched_result.1; // Update total_pages based on fetched data
            cache.insert(current_page, fetched_result.0); // Cache the fetched data
            satellites = (*cache.get(&current_page).unwrap()).clone();
        }

        pb.finish_and_clear();
        clearscreen::clear().unwrap();

        if satellites.is_empty() {
            println!("No results found for {}", search_query);
            return;
        }

        if satellites.len() == 1 {
            info_ui(satellites.first().unwrap());
            return;
        }

        // Create selection options and display
        let mut selections: Vec<String> = satellites.iter().map(|s| s.name.clone()).collect();

        // Add "Previous" option only if not on the first page
        if current_page > 1 {
            selections.push("Previous".to_string());
        }

        // Add "Load More" option only if not on the last page
        if current_page < total_pages {
            selections.push("Load More".to_string());
        }

        selections.push("Cancel".to_string());

        let theme = ColorfulTheme {
            ..ColorfulTheme::default()
        };

        let selection = Select::with_theme(&theme)
            .with_prompt("Select a satellite to get more information or navigate pages")
            .default(0)
            .items(&selections)
            .interact_on_opt(&Term::stderr());

        match selection.unwrap_or_else(|_| process::exit(1)) {
            Some(index) => match selections[index].as_str() {
                "Previous" => {
                    if current_page > 1 {
                        current_page -= 1;
                    } else {
                        println!("Already at the first page.");
                    }
                }
                "Load More" => {
                    if current_page < total_pages {
                        current_page += 1;
                    } else {
                        println!("No more pages to load.");
                    }
                }
                "Cancel" => {
                    return;
                }
                _ => {
                    if index < satellites.len() {
                        // Call the new info view with the selected satellite
                        info_ui(&satellites[index]);
                        // After returning from info_ui, clear the screen
                        clearscreen::clear().unwrap();
                    }
                }
            },
            None => {
                println!("No selection made.");
                return;
            }
        }
    }
}
