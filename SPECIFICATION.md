Real-Time Weather Simulation System - Specifications
This document outlines the specifications for the Real-Time Weather Simulation System, a Rust-based application that generates simulated weather data for a given city based on the current season and randomized parameters.

#Overview

The system simulates real-time weather conditions for a user-provided city. It dynamically generates weather data such as temperature, humidity, wind speed, and weather conditions (e.g., sunny, rainy, etc.) based on the current month and a random number generator.

#Functional Requirements

1. User Input
The system shall prompt the user to enter a city name.
The system shall validate the input to ensure the city name is not empty.
If the input is empty, the system shall display an error message: Error: City name cannot be empty.

2. Weather Data Generation
   
The system shall determine the current season based on the month:
Winter: December, January, February
Spring: March, April, May
Summer: June, July, August
Autumn: September, October, November

#The system shall generate the following weather data:

Temperature: Randomized within a range specific to the season:
  Winter: -5°C to 10°C
  Spring: 10°C to 25°C
  Summer: 20°C to 35°C
  Autumn: 10°C to 20°C
  
Humidity: Randomized between 30% and 90%.
Wind Speed: Randomized between 1 m/s and 15 m/s.
Weather Condition: Randomly selected from the following options:
Sunny, Cloudy, Rainy, Stormy, Foggy, Snowy, Windy.

3. Weather Report
   
The system shall generate a formatted weather report containing:
City name
Season
Temperature
Humidity
Weather condition
Wind speed

#The report shall be displayed to the user in the following format:

Real-Time Weather Simulation:
City: [City Name]
Season: [Season]
Temperature: [Temperature]°C
Humidity: [Humidity]%
Weather: [Weather Condition]
Wind Speed: [Wind Speed] m/s

#Non-Functional Requirements

 1. Performance
The system shall generate and display the weather report within 1 second of receiving valid user input.
 2. Usability
The system shall provide clear instructions and error messages to guide the user.
3. Reliability
The system shall handle invalid input gracefully and provide appropriate feedback.

#Technical Specifications

1. Programming Language
Rust
2. Dependencies
rand: For generating random numbers.
chrono: For retrieving the current date and time.

4. Code Structure
generate_weather(city: &str) -> String:
Input: City name (string).
Output: Formatted weather report (string).
Functionality: Determines the season, generates random weather data, and formats the report.
main():
Entry point of the program.
Handles user input, validates it, and calls generate_weather to display the report.

5 Input/Output
Input:
City name (provided by the user via the command line).
Output:
Weather report (displayed in the command line).

#Example Workflow

The user runs the program.
The program prompts: Enter city name:.
The user enters a city name (e.g., New York).

The program generates and displays the weather report:
Real-Time Weather Simulation:
City: New York
Season: Summer
Temperature: 28°C
Humidity: 65%
Weather: Sunny
Wind Speed: 7 m/s

#Error Handling
If the user enters an empty city name, the program shall display:
Error: City name cannot be empty.

#Assumptions and Constraints
The system assumes the user provides a valid city name (non-empty string).
The system does not validate the existence of the city or its geographical location.
The weather data is simulated and does not reflect real-world conditions.

