use crate::{cli::utils, libs::satellite_position_service};
use crossterm::style::{Color, Stylize};
// Import colored crate for coloring text
use dialoguer::console::Term; // Import Dialoguer for enhanced UI

// Function to mark a position with '∆' on the ASCII map without coloring it
fn mark_position(image_ascii: &mut String, x: u32, y: u32, width: u32) {
    let pos = (y * (width + 1) + x) as usize; // +1 accounts for newlines in the ASCII image
    let marker = "∆";
    image_ascii.replace_range(pos..pos + 1, marker);
}

fn mark_orbit_trail(image_ascii: &mut String, orbit_trail: &Vec<(u32, u32)>, width: u32) {
    for (x, y) in orbit_trail {
        let pos = (*y * (width + 1) + *x) as usize; // +1 accounts for newlines in the ASCII image
        let marker = "o";
        image_ascii.replace_range(pos..pos + 1, marker);
    }
}

// New function to colorize the map, including the marked position and orbit trail
fn colorize_map(image_ascii: &mut String) {
    let colored_map = image_ascii
        .chars()
        .map(|c| {
            if c == '∆' {
                // Assuming '∆' is the marker character
                c.to_string().with(Color::Magenta).to_string() // Apply blue color to marker
            } else if c == 'o' {
                c.to_string().with(Color::Cyan).to_string() // Apply dark cyan color to orbit trail
            } else {
                c.to_string().with(Color::DarkGrey).to_string() // Apply dark grey color to the rest of the map
            }
        })
        .collect::<Vec<_>>()
        .join("");
    *image_ascii = colored_map;
}

use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

pub fn map_ui(satellite_id: i32, lat: f64, lon: f64) {
    const IMAGE_SRC: &str = "assets/world_map.jpeg";
    const HEIGHT: u32 = 50;
    const WIDTH: u32 = 100;
    const REFRESH_INTERVAL: u64 = 30; // Refresh interval in seconds
    let mut orbit_trail: Vec<(u32, u32)> = Vec::new();

    let running = Arc::new(AtomicBool::new(true));
    let running_clone = Arc::clone(&running);

    thread::spawn(move || {
        // Initial display without waiting
        let mut image_ascii = utils::convert_image_to_ascii(IMAGE_SRC, WIDTH, HEIGHT);

        let (x, y) = lat_long_to_ascii_coords(lat, lon, WIDTH, HEIGHT);
        mark_position(&mut image_ascii, x, y, WIDTH); // Mark position first
        colorize_map(&mut image_ascii); // Then colorize the entire map
        orbit_trail.push((x, y));

        clearscreen::clear().unwrap();
        let term = Term::stdout();

        term.write_line(&image_ascii).unwrap();
        term.write_line("\nPress Enter to return...").unwrap();

        while running.load(Ordering::SeqCst) {
            thread::sleep(Duration::from_secs(REFRESH_INTERVAL)); // Wait for the initial data to become stale

            // Fetch new data here, assuming fetch_lat_lon() is implemented
            let (new_lat, new_lon) =
                satellite_position_service::get_satellite_position(satellite_id); // Replace with actual fetching logic

            let mut image_ascii = utils::convert_image_to_ascii(IMAGE_SRC, WIDTH, HEIGHT);
            let (x, y) = lat_long_to_ascii_coords(new_lat, new_lon, WIDTH, HEIGHT);

            orbit_trail.push((x, y)); // Store the new position in the orbit trail

            mark_orbit_trail(&mut image_ascii, &orbit_trail, WIDTH); // Draw the orbit trail first
            mark_position(&mut image_ascii, x, y, WIDTH); // Add the new position to the map
            colorize_map(&mut image_ascii); // Then colorize the entire map

            clearscreen::clear().unwrap();
            let term = Term::stdout();
            term.write_line(&image_ascii).unwrap(); // Print the ASCII map with the blue marker
            term.write_line("\nPress Enter to return...").unwrap(); // Prompt to continue

            // Drain the orbit trail after a certain number of iterations to prevent memory bloat
            if orbit_trail.len() > (WIDTH * 2) as usize {
                orbit_trail.drain(0..(WIDTH / 2) as usize);
            }
        }
    });

    // Separate thread for user input
    let term = Term::stdout();
    term.read_line().unwrap(); // Wait for user input to continue
    running_clone.store(false, Ordering::SeqCst);
}

pub fn lat_long_to_ascii_coords(
    lat: f64,
    lon: f64,
    ascii_width: u32,
    ascii_height: u32,
) -> (u32, u32) {
    // Adjust the normalization process for longitude and latitude
    // Longitude: Convert from [-180, 180] to [0, 1`
    let norm_lon = (lon + 180.0) / 360.0;
    // Latitude: Convert from [-90, 90] to [0, 1], with inversion for correct orientation
    let norm_lat = (90.0 - lat) / 180.0;

    // Convert to ASCII coordinates
    let ascii_x = (norm_lon * (ascii_width as f64)).round() as u32;
    let ascii_y = (norm_lat * (ascii_height as f64)).round() as u32;

    // Ensure the coordinates are within the bounds of the ASCII map
    let ascii_x = ascii_x.min(ascii_width - 1);
    let ascii_y = ascii_y.min(ascii_height - 1);

    (ascii_x, ascii_y)
}
