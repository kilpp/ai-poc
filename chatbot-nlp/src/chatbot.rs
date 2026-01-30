use crate::intent::{Intent, IntentRecognizer};
use crate::entity::{Entity, EntityExtractor};
use crate::conversation::{ConversationManager, ConversationContext};
use std::collections::HashMap;

pub struct ResponseTemplate {
    intent: Intent,
    templates: Vec<String>,
}

pub struct Chatbot {
    intent_recognizer: IntentRecognizer,
    entity_extractor: EntityExtractor,
    conversation_manager: ConversationManager,
    response_templates: HashMap<Intent, Vec<String>>,
}

impl Chatbot {
    pub fn new() -> Self {
        let mut chatbot = Chatbot {
            intent_recognizer: IntentRecognizer::new(),
            entity_extractor: EntityExtractor::new(),
            conversation_manager: ConversationManager::new(),
            response_templates: HashMap::new(),
        };
        
        chatbot.initialize_response_templates();
        chatbot
    }
    
    fn initialize_response_templates(&mut self) {
        self.response_templates.insert(
            Intent::Greeting,
            vec![
                "Hello! How can I help you today?".to_string(),
                "Hi there! What can I do for you?".to_string(),
                "Hey! Nice to meet you. How may I assist?".to_string(),
            ],
        );
        
        self.response_templates.insert(
            Intent::Farewell,
            vec![
                "Goodbye! Have a great day!".to_string(),
                "See you later! Take care!".to_string(),
                "Bye! Feel free to come back anytime!".to_string(),
            ],
        );
        
        self.response_templates.insert(
            Intent::BookAppointment,
            vec![
                "I'd be happy to help you book an appointment. What date and time works for you?".to_string(),
                "Sure! Let's schedule that. When would you like to come in?".to_string(),
            ],
        );
        
        self.response_templates.insert(
            Intent::CheckWeather,
            vec![
                "Let me check the weather for you. Which location are you interested in?".to_string(),
                "I can help with that! What city's weather would you like to know?".to_string(),
            ],
        );
        
        self.response_templates.insert(
            Intent::OrderFood,
            vec![
                "Great! What would you like to order?".to_string(),
                "I can help you with that. What are you in the mood for?".to_string(),
            ],
        );
        
        self.response_templates.insert(
            Intent::GetHelp,
            vec![
                "I'm here to help! I can assist with:\n- Booking appointments\n- Checking weather\n- Ordering food\n- Answering questions\nWhat do you need?".to_string(),
            ],
        );
        
        self.response_templates.insert(
            Intent::Question,
            vec![
                "That's a good question. Let me help you with that.".to_string(),
                "I'll do my best to answer that for you.".to_string(),
            ],
        );
        
        self.response_templates.insert(
            Intent::Unknown,
            vec![
                "I'm not sure I understood that. Could you rephrase?".to_string(),
                "I didn't quite catch that. Can you try asking in a different way?".to_string(),
            ],
        );
    }
    
    pub fn process_message(&mut self, session_id: &str, user_input: &str) -> String {
        // Get or create conversation context
        let _context = self.conversation_manager.get_or_create_session(session_id.to_string());
        
        // Recognize intent
        let intent = self.intent_recognizer.recognize(user_input);
        
        // Extract entities
        let entities = self.entity_extractor.extract(user_input);
        
        // Generate response based on intent and entities
        let response = self.generate_response(&intent, &entities, user_input);
        
        // Record this turn in conversation history
        self.conversation_manager.record_turn(
            session_id,
            user_input.to_string(),
            response.clone(),
            intent.clone(),
            entities.clone(),
        );
        
        // Update context based on entities
        if let Some(context) = self.conversation_manager.get_session_mut(session_id) {
            for entity in &entities {
                let key = format!("last_{:?}", entity.entity_type).to_lowercase();
                context.set_context(key, entity.value.clone());
            }
        }
        
        response
    }
    
    fn generate_response(&self, intent: &Intent, entities: &[Entity], user_input: &str) -> String {
        let templates = self.response_templates
            .get(intent)
            .map(|v| v.as_slice())
            .unwrap_or(&[]);
        
        if templates.is_empty() {
            return "I understand, but I'm not sure how to respond to that yet.".to_string();
        }
        
        // Select a template (using simple rotation based on input length)
        let template_index = user_input.len() % templates.len();
        let mut response = templates[template_index].clone();
        
        // Enhance response with entity information
        if !entities.is_empty() {
            response = self.enhance_response_with_entities(response, entities, intent);
        }
        
        response
    }
    
    fn enhance_response_with_entities(&self, mut response: String, entities: &[Entity], intent: &Intent) -> String {
        match intent {
            Intent::BookAppointment => {
                let dates: Vec<_> = entities.iter()
                    .filter(|e| matches!(e.entity_type, crate::entity::EntityType::Date))
                    .collect();
                let times: Vec<_> = entities.iter()
                    .filter(|e| matches!(e.entity_type, crate::entity::EntityType::Time))
                    .collect();
                
                if !dates.is_empty() && !times.is_empty() {
                    response = format!(
                        "Perfect! I've noted your appointment for {} at {}. I'll confirm this for you.",
                        dates[0].value, times[0].value
                    );
                } else if !dates.is_empty() {
                    response = format!(
                        "Great! I see you want an appointment on {}. What time would work for you?",
                        dates[0].value
                    );
                } else if !times.is_empty() {
                    response = format!(
                        "Noted the time as {}. What date would you prefer?",
                        times[0].value
                    );
                }
            }
            Intent::CheckWeather => {
                let locations: Vec<_> = entities.iter()
                    .filter(|e| matches!(e.entity_type, crate::entity::EntityType::Location))
                    .collect();
                
                if !locations.is_empty() {
                    response = format!(
                        "Checking the weather for {}... (This would connect to a weather API in a real implementation)",
                        locations[0].value
                    );
                }
            }
            _ => {}
        }
        
        response
    }
    
    pub fn get_conversation_context(&self, session_id: &str) -> Option<&ConversationContext> {
        self.conversation_manager.get_session(session_id)
    }
    
    pub fn end_conversation(&mut self, session_id: &str) -> Option<ConversationContext> {
        self.conversation_manager.end_session(session_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_greeting_conversation() {
        let mut chatbot = Chatbot::new();
        let response = chatbot.process_message("test-session", "Hello!");
        
        assert!(response.contains("Hello") || response.contains("Hi"));
    }
    
    #[test]
    fn test_appointment_booking() {
        let mut chatbot = Chatbot::new();
        let response = chatbot.process_message(
            "test-session",
            "I want to book an appointment for tomorrow at 3pm"
        );
        
        assert!(response.to_lowercase().contains("appointment") || 
                response.to_lowercase().contains("tomorrow"));
    }
    
    #[test]
    fn test_context_persistence() {
        let mut chatbot = Chatbot::new();
        
        chatbot.process_message("session-1", "Hello");
        chatbot.process_message("session-1", "Book appointment");
        
        let context = chatbot.get_conversation_context("session-1");
        assert!(context.is_some());
        assert_eq!(context.unwrap().conversation_history.len(), 2);
    }
}
