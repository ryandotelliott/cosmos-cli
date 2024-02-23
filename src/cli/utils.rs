use std::time::Duration;

use image::GenericImageView;
use indicatif::{ProgressBar, ProgressStyle};

use chrono::{Timelike, Utc};

pub fn print_satellite_info(satellite: &satellite_search_service::Satellite, lat: f64, lon: f64) {
    let tle = parse_tle(
        satellite.name.as_str(),
        satellite.line_one.as_str(),
        satellite.line_two.as_str(),
    );

    let datetime = Utc::now();
    let current_utc_time_in_minutes = datetime.hour() * 60 + datetime.minute();

    let distance = compute_satellite_distance_from_earth(
        &satellite.name,
        &satellite.line_one,
        &satellite.line_two,
        current_utc_time_in_minutes as f64,
    );

    println!("Name: {}", satellite.name);
    println!(
        "International Designator: {}",
        tle.international_designator.unwrap_or("N/A".to_string())
    );
    println!("Epoch: {}", tle.datetime);
    println!("Inclination: {} degrees", tle.inclination);
    println!("RAAN: {:.2} degrees", tle.right_ascension);
    println!("Eccentricity: {}", tle.eccentricity);
    println!("Perigee: {:.2} degrees", tle.argument_of_perigee);
    println!("Mean Motion: {:.2} revs per day", tle.mean_motion);
    println!("Orbit Number: {}", tle.revolution_number);
    println!("Distance from Earth: {:.2} km", distance.unwrap_or(0.0));
    println!("Latitude: {:.2} degrees", lat);
    println!("Longitude: {:.2} degrees", lon);
}

pub fn parse_tle(satellite_name: &str, line1: &str, line2: &str) -> sgp4::Elements {
    let element = sgp4::Elements::from_tle(
        Some(satellite_name.to_owned()),
        line1.as_bytes(),
        line2.as_bytes(),
    )
    .unwrap();

    element
}

use sgp4::{Constants, Elements};

use crate::libs::satellite_search_service;

pub fn compute_satellite_distance_from_earth(
    satellite_name: &str,
    line1: &str,
    line2: &str,
    time_in_minutes: f64,
) -> Result<f64, String> {
    // TODO: Update to take any UTC time & convert to minutes
    let earth_radius_km: f64 = 6371.0;

    let element = Elements::from_tle(
        Some(satellite_name.to_owned()),
        line1.as_bytes(),
        line2.as_bytes(),
    )
    .map_err(|e| e.to_string())?;

    let constants = Constants::from_elements(&element).map_err(|e| e.to_string())?;

    let prediction = constants
        .propagate(sgp4::MinutesSinceEpoch(time_in_minutes))
        .unwrap();

    let [x, y, z] = prediction.position;

    let distance_from_center = (x.powi(2) + y.powi(2) + z.powi(2)).sqrt();
    let distance_from_surface = distance_from_center - earth_radius_km;

    Ok(distance_from_surface)
}

pub fn display_spinner(message: String, duration: Option<u64>) -> ProgressBar {
    let duration = duration.unwrap_or(100);

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("{spinner:.magenta} {msg:.magenta}")
            .unwrap(),
    );
    pb.enable_steady_tick(Duration::from_millis(duration));
    pb.set_message(message);

    pb
}

pub fn convert_image_to_ascii(image_src: &str, width: u32, height: u32) -> String {
    let image = image::open(image_src)
        .unwrap()
        .resize(width, height, image::imageops::FilterType::Nearest)
        .grayscale(); // Convert the image to grayscale
    let pixels = image.pixels();
    let mut ascii_image = String::new();

    for pixel in pixels {
        let brightness = pixel.2[0]; // In grayscale, all channels have the same value

        let ascii_char = match brightness {
            0..=63 => "@",
            64..=127 => "%",
            128..=191 => ".",
            192..=255 => " ",
        };
        ascii_image.push_str(ascii_char);
    }

    // Add new lines after every width number of characters
    let ascii_image_with_newlines = ascii_image
        .chars()
        .enumerate()
        .map(|(i, c)| {
            if i % width as usize == 0 && i != 0 {
                format!("\n{}", c)
            } else {
                c.to_string()
            }
        })
        .collect();

    ascii_image_with_newlines
}
