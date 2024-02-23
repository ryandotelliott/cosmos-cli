use dialoguer::{theme::ColorfulTheme, Select};

use super::{list::list_ui, search::search_ui};

pub fn main_menu() {
    clearscreen::clear().unwrap(); // Clear the screen before displaying the menu

    loop {
        let items = &["Search Satellites", "View List", "Exit"];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Please select an option")
            .default(0)
            .items(&items[..])
            .interact()
            .unwrap();

        match selection {
            0 => {
                search_ui();
                clearscreen::clear().unwrap();
            }
            1 => {
                list_ui();
                clearscreen::clear().unwrap();
            }
            2 => break, // Exit the loop, thus exiting the program
            _ => unreachable!(),
        }
    }
}
