# Architecture Documentation - NLP Chatbot

## Overview
This chatbot is built using traditional NLP techniques without machine learning, relying on pattern matching, regex-based extraction, and rule-based systems.

## Core Components

### 1. Intent Recognition (`intent.rs`)

#### Intent Types
- **Greeting**: Hello, hi, good morning/afternoon/evening
- **Farewell**: Bye, goodbye, see you, exit, quit
- **Question**: What, when, where, how, why, who
- **BookAppointment**: Book/schedule appointment/meeting, reserve, make appointment
- **CheckWeather**: Weather, temperature, forecast, rain, sunny
- **OrderFood**: Order food/pizza, want to eat, hungry
- **GetHelp**: Help, assist, support
- **Unknown**: Fallback for unrecognized intents

#### Algorithm
```rust
1. Tokenize input text (lowercase, remove punctuation)
2. For each intent pattern:
   a. Check if ALL pattern words exist in input
   b. Calculate score based on:
      - Number of matched words (squared)
      - Consecutive phrase matches (bonus)
3. Return intent with highest score
4. Return Unknown if no matches
```

#### Scoring System
- Base score: `matched_words² * 10`
- Consecutive match bonus: `+20`
- Requires ALL pattern words to match

### 2. Entity Extraction (`entity.rs`)

#### Entity Types
- **Date**: Dates in various formats (YYYY-MM-DD, DD/MM/YYYY, today, tomorrow, day names)
- **Time**: Times with optional am/pm (14:30, 3pm, 3:00 pm)
- **Location**: Places prefixed by in/at/from/to
- **Email**: Email addresses
- **Phone**: Phone numbers in various formats
- **Number**: Numeric values

#### Extraction Method
Uses regex patterns for each entity type:
```rust
Date: \b(\d{1,2}/\d{1,2}/\d{4}|\d{4}-\d{2}-\d{2}|today|tomorrow|monday|...)\b
Time: \b(\d{1,2}:\d{2}(?:\s?(?:am|pm))?|\d{1,2}\s?(?:am|pm))\b
Email: \b[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}\b
Phone: \b(\+?\d{1,3}[-.\s]?)?\(?\d{3}\)?[-.\s]?\d{3}[-.\s]?\d{4}\b
...
```

Returns entities with:
- Type
- Value (matched text)
- Position (start, end indices)

### 3. Conversation Management (`conversation.rs`)

#### ConversationTurn
Represents a single interaction:
```rust
{
    user_input: String,
    bot_response: String,
    intent: String,
    entities: Vec<String>,
    timestamp: DateTime<Utc>
}
```

#### ConversationContext
Session state for each user:
```rust
{
    session_id: String,
    user_name: Option<String>,
    conversation_history: VecDeque<ConversationTurn>,  // Last 10 turns
    context_data: HashMap<String, String>,
    started_at: DateTime<Utc>,
    last_intent: Option<String>
}
```

#### Features
- **History Management**: Keeps last 10 conversation turns
- **Context Storage**: Key-value store for entities and preferences
- **Multi-session**: Supports multiple concurrent sessions
- **Temporal Tracking**: Timestamps all interactions

### 4. Chatbot Engine (`chatbot.rs`)

#### Main Flow
```
User Input
    ↓
Intent Recognition
    ↓
Entity Extraction
    ↓
Response Generation
    ↓
Context Update
    ↓
History Recording
    ↓
Return Response
```

#### Response Generation
1. **Template Selection**: Choose template based on intent
2. **Entity Enhancement**: Modify response with extracted entities
3. **Contextualization**: Use conversation context if available

#### Response Templates
Predefined templates for each intent with variations:
```rust
Intent::Greeting → [
    "Hello! How can I help you today?",
    "Hi there! What can I do for you?",
    ...
]
```

Template selected using: `input_length % templates.length`

#### Entity-Enhanced Responses
For specific intents (BookAppointment, CheckWeather), responses are enhanced:
```rust
"Book appointment" + Date + Time
  → "Perfect! I've noted your appointment for {date} at {time}."

"Book appointment" + Date only
  → "Great! I see you want an appointment on {date}. What time?"
```

## Data Flow

### Example: Booking an Appointment

```
Input: "I want to book an appointment for tomorrow at 3pm"

1. Intent Recognition:
   - Tokenize: ["i", "want", "to", "book", "an", "appointment", "for", "tomorrow", "at", "3pm"]
   - Match "book" + "appointment" → Intent::BookAppointment
   - Score: 40 (2² * 10)

2. Entity Extraction:
   - Extract "tomorrow" → EntityType::Date
   - Extract "3pm" → EntityType::Time

3. Response Generation:
   - Select template for BookAppointment
   - Enhance with entities:
     "Perfect! I've noted your appointment for tomorrow at 3pm."

4. Context Update:
   - Store "last_date": "tomorrow"
   - Store "last_time": "3pm"
   - Add turn to history

5. Return enhanced response
```

## Session Management

### Session Lifecycle
```
User connects → Create/Retrieve session → Process messages → End session
```

### Session Operations
- **Create**: `conversation_manager.get_or_create_session(id)`
- **Access**: `conversation_manager.get_session(id)`
- **Update**: `conversation_manager.record_turn(...)`
- **Terminate**: `conversation_manager.end_session(id)`

## Testing Strategy

### Unit Tests
- **Intent Recognition**: Test each intent with sample inputs
- **Entity Extraction**: Test each entity type with various formats
- **Conversation**: Test context storage and history management
- **Integration**: Test full chatbot flow with realistic conversations

### Test Coverage
- Intent recognition accuracy
- Entity extraction patterns
- Context persistence across turns
- Response generation logic

## Performance Characteristics

### Time Complexity
- Intent Recognition: O(P * W) where P = patterns, W = words
- Entity Extraction: O(R * L) where R = regex patterns, L = text length
- Response Generation: O(1)

### Space Complexity
- Context per session: O(10) turns + O(K) context data
- Memory efficient with VecDeque for history

## Extensibility Points

### Adding New Intents
```rust
1. Add enum variant to Intent
2. Add patterns to IntentRecognizer::new()
3. Add response templates
4. Optionally add entity enhancement logic
```

### Adding New Entity Types
```rust
1. Add enum variant to EntityType
2. Add regex pattern to EntityExtractor::new()
3. Optionally add extraction helper methods
```

### Customizing Responses
```rust
// In initialize_response_templates()
self.response_templates.insert(
    Intent::YourIntent,
    vec!["Template 1", "Template 2", ...]
);
```

## Limitations

### Pattern Matching
- Cannot understand semantically similar phrases not in patterns
- Example: "Set up a meeting" might not match if not in patterns

### Entity Extraction
- Regex-based, may miss complex entity formats
- Location extraction requires capitalization
- No disambiguation (e.g., "Apple" as company vs fruit)

### Context
- No cross-session learning
- Context data not persisted to disk
- Limited to 10 recent turns

## Future Improvements

### Short-term
1. Add spell checking/correction
2. Implement fuzzy string matching
3. Add more entity types (money, percentages, etc.)
4. Persistent storage (SQLite/PostgreSQL)

### Medium-term
1. Sentiment analysis
2. Multi-turn dialog management
3. User preference learning
4. Integration with external APIs

### Long-term
1. Machine learning-based intent classification
2. Named Entity Recognition (NER) models
3. Dialog state tracking
4. Multi-language support

## Configuration

### Customizable Parameters
- History size: Change `conversation_history` capacity
- Pattern weights: Modify scoring algorithm
- Response selection: Change template selection logic
- Entity patterns: Update regex patterns

## Deployment Considerations

### Production Readiness
- [ ] Add logging framework
- [ ] Implement rate limiting
- [ ] Add authentication/authorization
- [ ] Set up monitoring and metrics
- [ ] Implement graceful error handling
- [ ] Add configuration file support
- [ ] Set up CI/CD pipeline

### Scaling
- Stateless design allows horizontal scaling
- Sessions stored in-memory (consider Redis for distributed systems)
- Each request is independent

## Dependencies

- `serde`: Data serialization
- `serde_json`: JSON support
- `regex`: Pattern matching
- `chrono`: Date/time handling

## File Structure
```
src/
├── lib.rs           # Library exports
├── main.rs          # CLI application
├── intent.rs        # Intent recognition
├── entity.rs        # Entity extraction
├── conversation.rs  # Conversation management
└── chatbot.rs       # Main chatbot engine
```
