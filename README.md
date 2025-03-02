# WeatherTrackingSystem
A real time weather tracking system in Rust 

Real-Time Weather Simulation/tracking System
This is a simple Rust program that simulates real-time weather conditions for a given city. The system generates weather data such as temperature, humidity, wind speed, and weather conditions (e.g., sunny, rainy, etc.) based on the current season and a random number generator.

Features

Dynamic Weather Generation: The weather conditions are generated dynamically based on the current month, simulating seasonal changes.
Randomized Data: Temperature, humidity, wind speed, and weather conditions are randomized within realistic ranges for the given season.
User Input: The user can input a city name, and the system will generate a weather report for that city.
Error Handling: The system checks if the city name is empty and provides an appropriate error message.

How It Works

Season Determination: The program determines the current season based on the month:
Winter: December, January, February
Spring: March, April, May
Summer: June, July, August
Autumn: September, October, November

Weather Data Generation:

Temperature: Randomly generated within a range specific to the season.
Humidity: Randomly generated between 30% and 90%.
Wind Speed: Randomly generated between 1 m/s and 15 m/s.
Weather Condition: Randomly selected from a list of possible conditions (e.g., Sunny, Cloudy, Rainy, etc.).

User Interaction:

The user is prompted to enter a city name.
The program generates and displays a weather report for the entered city.

Code Structure

generate_weather(city: &str) -> String: This function generates the weather report for the given city. It determines the season, generates random weather data, and formats the report as a string.
main(): The entry point of the program. It handles user input, checks for empty input, and calls the generate_weather function to display the weather report.

Dependencies

rand: Used for generating random numbers.
chrono: Used for getting the current date and time to determine the season.

Usage

Clone the repository or copy the code into a Rust project.
Build the project using cargo build.
Run the program using cargo run.
Enter a city name when prompted.
View the generated weather report.

#Example Output

Copy
Enter city name: New York

Real-Time Weather Simulation:
City: New York
Season: Summer
Temperature: 28°C
Humidity: 65%
Weather: Sunny
Wind Speed: 7 m/s

Error Handling
If the user enters an empty city name, the program will display an error message: Error: City name cannot be empty.
