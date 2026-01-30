# NLP Chatbot - Traditional Techniques

A chatbot implementation using traditional NLP techniques built with Rust, featuring intent recognition, entity extraction, conversation management, and context tracking.

## Features

### 🎯 Intent Recognition
- Pattern-based intent classification
- Supports multiple intent types:
  - Greetings
  - Farewells
  - Questions
  - Appointment booking
  - Weather checks
  - Food ordering
  - Help requests

### 🏷️ Entity Extraction
- Regex-based entity extraction
- Extracts various entity types:
  - Dates (e.g., "2024-01-15", "tomorrow", "Monday")
  - Times (e.g., "3pm", "14:30")
  - Locations (e.g., "in New York")
  - Email addresses
  - Phone numbers
  - Numbers

### 💬 Conversation Management
- Session-based conversation tracking
- Maintains conversation history (last 10 turns)
- Context data storage for personalization
- Timestamp tracking for all interactions

### 🧠 Context Tracking
- Persistent context across conversation turns
- Automatic entity storage in context
- Last intent tracking
- Session management with unique IDs

## Project Structure

```
src/
├── main.rs           # CLI interface and main application
├── chatbot.rs        # Main chatbot engine and response generation
├── intent.rs         # Intent recognition logic
├── entity.rs         # Entity extraction logic
└── conversation.rs   # Conversation and context management
```

## Installation

Make sure you have Rust installed. Then:

```bash
cargo build --release
```

## Usage

Run the chatbot:

```bash
cargo run
```

### Example Interactions

```
You: Hello!
Bot: Hello! How can I help you today?

You: I want to book an appointment for tomorrow at 3pm
Bot: Perfect! I've noted your appointment for tomorrow at 3pm. I'll confirm this for you.

You: What's the weather in San Francisco?
Bot: Checking the weather for San Francisco... (This would connect to a weather API in a real implementation)

You: show context
[Displays conversation history and context data]

You: Bye
Bot: Goodbye! Have a great day!
```

### Special Commands

- `show context` - Display conversation history and context data
- `clear` or `reset` - Clear conversation context
- `bye`, `exit`, or `quit` - Exit the chatbot

## Technical Details

### Intent Recognition Algorithm

The intent recognizer uses a pattern-matching approach:
1. Tokenizes input text
2. Matches tokens against predefined patterns
3. Calculates match scores
4. Returns the intent with the highest score

### Entity Extraction

Uses regex patterns to identify and extract:
- Temporal entities (dates, times)
- Contact information (emails, phones)
- Locations
- Numerical values

### Context Management

- **Session-based**: Each conversation has a unique session ID
- **History**: Maintains last 10 conversation turns
- **Context Data**: Key-value store for extracted entities
- **Temporal Tracking**: All turns are timestamped

## Dependencies

- `serde` - Serialization/deserialization
- `serde_json` - JSON support
- `regex` - Regular expression support
- `chrono` - Date and time handling

## Testing

Run the test suite:

```bash
cargo test
```

Tests cover:
- Intent recognition accuracy
- Entity extraction patterns
- Conversation context management
- Response generation

## Architecture

```
┌─────────────┐
│   User      │
└──────┬──────┘
       │
       ▼
┌─────────────────────────────────────┐
│         Chatbot Engine              │
├─────────────────────────────────────┤
│  ┌──────────────────────────────┐  │
│  │  Intent Recognizer           │  │
│  └──────────────────────────────┘  │
│  ┌──────────────────────────────┐  │
│  │  Entity Extractor            │  │
│  └──────────────────────────────┘  │
│  ┌──────────────────────────────┐  │
│  │  Conversation Manager        │  │
│  │  - Session Management        │  │
│  │  - Context Tracking          │  │
│  │  - History Storage           │  │
│  └──────────────────────────────┘  │
│  ┌──────────────────────────────┐  │
│  │  Response Generator          │  │
│  └──────────────────────────────┘  │
└─────────────────────────────────────┘
```

## Limitations

This is a traditional NLP implementation and has some limitations:

- Pattern-based intent recognition (not ML-based)
- Fixed regex patterns for entity extraction
- No spell correction
- Limited to predefined intents
- No semantic understanding

For production use, consider:
- Machine learning-based intent classification
- Named Entity Recognition (NER) models
- Integration with real APIs (weather, booking, etc.)
- Persistent storage (database)
- Multi-language support

## Future Enhancements

- [ ] Add spell checking and correction
- [ ] Implement fuzzy matching for intents
- [ ] Add sentiment analysis
- [ ] Persistent storage with SQLite/PostgreSQL
- [ ] REST API interface
- [ ] Web interface
- [ ] Multi-language support
- [ ] Integration with external APIs
- [ ] Machine learning-based improvements

## License

MIT

## Contributing

Feel free to open issues or submit pull requests!
