Real time weather tracking system architecture C4 diagrams

1. Context Diagram
This diagram Shows the interaction between the User and the Weather Tracking System.
The user interacts with the system via the command-line interface (CLI).

![Screenshot_2-3-2025_223350_www mermaidchart com](https://github.com/user-attachments/assets/8f434eed-fe86-437a-877f-f056881375e6)

2. Container Diagram
This diagram shows the high-level structure of the system and its containers.
Breaks down the system into two containers:
     CLI: Handles user input and output.
     Weather Logic: Generates weather data based on the current season and random parameters.

![Screenshot_2-3-2025_223640_www mermaidchart com](https://github.com/user-attachments/assets/52a3c4fc-7c72-4bb9-9b1c-dd7d3f546804)

3. Component Diagram
This diagram Breaks down the Weather Logic container into three components:
       Input Handler: Reads and validates user input.
       Weather Generator: Generates weather data (temperature, humidity, wind speed, weather condition).
       Report Formatter: Formats the weather data into a readable report.

![Screenshot_2-3-2025_223748_www mermaidchart com](https://github.com/user-attachments/assets/587ebd71-0016-48df-a869-7bf5fc24210c)

4. Code Diagram
This diagram shows the code-level structure of the system, including functions and their relationships.

Shows the code-level structure, including the main() function and the generate_weather() function.
Illustrates how the main() function calls generate_weather() and receives the formatted weather report.

![Screenshot_2-3-2025_224243_www mermaidchart com](https://github.com/user-attachments/assets/d9693550-319f-41e8-af61-bdf721543137)


