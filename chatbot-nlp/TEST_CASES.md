# Example Test Cases for Chatbot NLP

## Intent Recognition Test Cases

### Greeting Intent
- "Hello"
- "Hi there"
- "Good morning"
- "Hey"
- "Good evening"

### Farewell Intent
- "Goodbye"
- "Bye"
- "See you later"
- "Exit"
- "Quit"

### Question Intent
- "What is the weather?"
- "When does the store open?"
- "Where can I find help?"
- "How do I book an appointment?"
- "Why is it so expensive?"

### BookAppointment Intent
- "I need to book an appointment"
- "Can I schedule a meeting?"
- "I want to make an appointment"
- "Reserve a slot for me"
- "Book me in for tomorrow"

### CheckWeather Intent
- "What's the weather like?"
- "Check the temperature"
- "Is it going to rain?"
- "Weather forecast for tomorrow"
- "Will it be sunny?"

### OrderFood Intent
- "I want to order food"
- "Order pizza for me"
- "I'm hungry"
- "Can I get something to eat?"

### GetHelp Intent
- "Help me"
- "I need assistance"
- "Can you support me?"

## Entity Extraction Test Cases

### Date Entities
- "2024-01-30" → Date
- "01/30/2024" → Date
- "today" → Date
- "tomorrow" → Date
- "Monday" → Date
- "next Friday" → Date

### Time Entities
- "3pm" → Time
- "15:30" → Time
- "3:00 pm" → Time
- "2 am" → Time

### Location Entities
- "in New York" → Location: "New York"
- "at Boston" → Location: "Boston"
- "from San Francisco" → Location: "San Francisco"

### Email Entities
- "contact@example.com" → Email
- "john.doe@company.org" → Email

### Phone Entities
- "555-123-4567" → Phone
- "(555) 123-4567" → Phone
- "+1-555-123-4567" → Phone

### Number Entities
- "I need 5 tickets" → Number: "5"
- "Book for 3 people" → Number: "3"

## Full Conversation Test Scenarios

### Scenario 1: Appointment Booking
```
User: Hello
Bot: [Greeting response]

User: I want to book an appointment
Bot: [Appointment booking prompt]

User: Tomorrow at 3pm
Bot: [Confirmation with extracted date and time]

User: Thanks, bye
Bot: [Farewell response]
```

### Scenario 2: Weather Check
```
User: Hi
Bot: [Greeting response]

User: What's the weather in Boston?
Bot: [Weather check response with location]

User: How about New York?
Bot: [Weather check response for new location]
```

### Scenario 3: Context Persistence
```
User: Book appointment for 2024-02-15
Bot: [Response requesting time]

User: At 2pm please
Bot: [Should remember the date from context]

User: show context
Bot: [Should display both date and time in context]
```

## Edge Cases

### Multiple Entities in One Sentence
- "Book appointment for tomorrow at 3pm in Boston" 
  - Should extract: Date (tomorrow), Time (3pm), Location (Boston)

### Ambiguous Intent
- "Can you help me book something?"
  - Could be Question or BookAppointment

### No Matching Intent
- "asdfghjkl"
  - Should return Unknown intent

### Multiple Intents in One Sentence
- "Hello, I want to book an appointment"
  - Primary intent should be BookAppointment

## Performance Expectations

- Intent recognition should complete in < 1ms
- Entity extraction should complete in < 5ms
- Full message processing should complete in < 10ms
- Context storage should support 100+ concurrent sessions
