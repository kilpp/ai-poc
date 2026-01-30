// Example usage of the chatbot library components
// This file demonstrates how to use the chatbot programmatically

use chatbot_nlp::{
    chatbot::Chatbot,
    intent::{Intent, IntentRecognizer},
    entity::{Entity, EntityExtractor, EntityType},
};

fn main() {
    println!("=== Chatbot NLP Examples ===\n");
    
    // Example 1: Intent Recognition
    example_intent_recognition();
    
    // Example 2: Entity Extraction
    example_entity_extraction();
    
    // Example 3: Full Chatbot Conversation
    example_chatbot_conversation();
    
    // Example 4: Context Tracking
    example_context_tracking();
}

fn example_intent_recognition() {
    println!("1. Intent Recognition Examples");
    println!("─────────────────────────────────");
    
    let recognizer = IntentRecognizer::new();
    
    let test_inputs = vec![
        "Hello there!",
        "Good morning",
        "I need to book an appointment",
        "What's the weather like?",
        "I want to order pizza",
        "Can you help me?",
        "Goodbye!",
    ];
    
    for input in test_inputs {
        let intent = recognizer.recognize(input);
        println!("Input: \"{}\"", input);
        println!("Intent: {:?}\n", intent);
    }
}

fn example_entity_extraction() {
    println!("\n2. Entity Extraction Examples");
    println!("─────────────────────────────────");
    
    let extractor = EntityExtractor::new();
    
    let test_inputs = vec![
        "Book appointment for 2024-01-30 at 3pm",
        "Contact me at john.doe@example.com or call 555-123-4567",
        "Meeting in New York tomorrow at 2:30pm",
        "I need 5 tickets for the show",
    ];
    
    for input in test_inputs {
        let entities = extractor.extract(input);
        println!("Input: \"{}\"", input);
        println!("Entities found:");
        for entity in entities {
            println!("  - {:?}: \"{}\" at position {:?}",
                entity.entity_type, entity.value, entity.position);
        }
        println!();
    }
}

fn example_chatbot_conversation() {
    println!("\n3. Full Chatbot Conversation");
    println!("─────────────────────────────────");
    
    let mut chatbot = Chatbot::new();
    let session_id = "example-session";
    
    let conversation = vec![
        "Hello!",
        "I'd like to book an appointment",
        "How about tomorrow at 3pm?",
        "Thanks! What's the weather in Boston?",
        "Goodbye",
    ];
    
    for input in conversation {
        println!("User: {}", input);
        let response = chatbot.process_message(session_id, input);
        println!("Bot: {}\n", response);
    }
}

fn example_context_tracking() {
    println!("\n4. Context Tracking Example");
    println!("─────────────────────────────────");
    
    let mut chatbot = Chatbot::new();
    let session_id = "context-example";
    
    // First interaction
    chatbot.process_message(session_id, "Book appointment for tomorrow at 2pm");
    
    // Second interaction
    chatbot.process_message(session_id, "What's the weather?");
    
    // Check context
    if let Some(context) = chatbot.get_conversation_context(session_id) {
        println!("Session ID: {}", context.session_id);
        println!("Number of turns: {}", context.conversation_history.len());
        
        println!("\nContext Data:");
        for (key, value) in &context.context_data {
            println!("  {}: {}", key, value);
        }
        
        println!("\nConversation History:");
        for (i, turn) in context.conversation_history.iter().enumerate() {
            println!("  Turn {}: {}", i + 1, turn.user_input);
            println!("    Intent: {}", turn.intent);
            println!("    Entities: {:?}", turn.entities);
        }
    }
}
