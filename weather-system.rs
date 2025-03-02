use rand::Rng;
use chrono::prelude::*;
use std::io;

fn generate_weather(city: &str) -> String {
    let weather_conditions = ["Sunny", "Cloudy", "Rainy", "Stormy", "Foggy", "Snowy", "Windy"];

    // Determine the season based on the month
    let month = Local::now().month();
    let (season, temp_range) = match month {
        12 | 1 | 2 => ("Winter", (-5, 10)),  // Cold season
        3 | 4 | 5 => ("Spring", (10, 25)),
        6 | 7 | 8 => ("Summer", (20, 35)),  // Hot season
        _ => ("Autumn", (10, 20)),
    };

    let mut rng = rand::thread_rng();
    let temperature = rng.gen_range(temp_range.0..=temp_range.1);
    let humidity = rng.gen_range(30..90);
    let wind_speed = rng.gen_range(1..15);
    let weather = weather_conditions[rng.gen_range(0..weather_conditions.len())];

    format!(
        "\nReal-Time Weather Simulation:\nCity: {}\nSeason: {}\nTemperature: {}°C\nHumidity: {}%\nWeather: {}\nWind Speed: {} m/s\n",
        city, season, temperature, humidity, weather, wind_speed
    )
}

fn main() {
    println!("Enter city name:");
    let mut city = String::new();
    io::stdin().read_line(&mut city).expect("Failed to read input");
    let city = city.trim();  // Remove any extra spaces/newlines

    if city.is_empty() {
        println!("Error: City name cannot be empty.");
    } else {
        let weather_report = generate_weather(city);
        println!("{}", weather_report);
    }
}

