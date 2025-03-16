Assignment 5: Use Case Modeling and Test Case Development for Weather Tracking System

1. Use Case diagram

![Screenshot (21)](https://github.com/user-attachments/assets/768f308a-fab6-432b-b93d-d0fe10ebb3a7)

1.1 actors:

General User – Accesses weather updates and forecasts.
Meteorologist – Provides and analyzes weather data.
System Administrator – Manages system operations and security.
Emergency Responder – Receives alerts for disaster response.
Developer – Maintains and enhances system functionalities.
Government Agency – Uses weather insights for policy and compliance.

1.2 Use Cases

user cases:
 
 View Weather – General users can access real-time weather updates.
Receive Alerts – Users receive notifications for severe weather conditions.
Configure Notifications – Users set location-based alerts.
Submit Reports – Users report weather anomalies.
Analyze Historical Data – Meteorologists access past weather trends.
Manage API Access – Third-party systems retrieve weather data.
Monitor System Performance – Admins ensure system stability.
Authenticate Users – Secure user access with multi-factor authentication.

2 use case: receive alerts

Actor: General User, Emergency ResponderPrecondition: User has opted into notifications.Postcondition: User receives a timely alert about severe weather.

Basic Flow:
System detects extreme weather conditions.
System verifies affected locations.
System sends alerts to subscribed users.
User receives and acknowledges the alert.

Alternative Flows:

Network Failure: System retries sending the alert every 10 seconds for 1 minute.
User Unsubscribed: System does not send alerts if the user has opted out.

3 Test case Development

   Functional Test Cases

Test Case ID

Requirement ID

Description

Steps

Expected Result

Status

TC-001

FR-002

User views weather update

1. Open app 2. Select location 3. View forecast

Weather data loads within 2s

Pass/Fail

TC-002

FR-003

Severe weather alert sent

1. Trigger extreme weather 2. Verify alert received

Alert received within 5s

Pass/Fail

Non-Functional Test Cases

Test Case ID

Requirement ID

Description

Test Method

Expected Result

Status

NTC-001

NFR-001

System handles 10,000 concurrent users

Load test simulation

No performance degradation

Pass/Fail

NTC-002

NFR-002

Data encryption validation

Check AES-256 encryption

All user data encrypted

Pass/Fail



4. Reflection

   Challenges Faced:

Defining Real-Time Scenarios: Ensuring alert delivery within 5 seconds required precise event handling.

Scalability Concerns: Load testing simulations needed robust infrastructure to handle peak usage.

Security Considerations: Implementing authentication while maintaining low latency posed integration challenges.
