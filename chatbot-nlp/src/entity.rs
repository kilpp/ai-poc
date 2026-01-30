use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EntityType {
    Date,
    Time,
    Location,
    Person,
    Number,
    Email,
    Phone,
}

#[derive(Debug, Clone)]
pub struct Entity {
    pub entity_type: EntityType,
    pub value: String,
    pub position: (usize, usize),
}

pub struct EntityExtractor {
    patterns: HashMap<EntityType, Regex>,
}

impl EntityExtractor {
    pub fn new() -> Self {
        let mut patterns = HashMap::new();
        
        // Date patterns (simple formats)
        patterns.insert(
            EntityType::Date,
            Regex::new(r"\b(\d{1,2}/\d{1,2}/\d{4}|\d{4}-\d{2}-\d{2}|today|tomorrow|monday|tuesday|wednesday|thursday|friday|saturday|sunday)\b").unwrap()
        );
        
        // Time patterns
        patterns.insert(
            EntityType::Time,
            Regex::new(r"\b(\d{1,2}:\d{2}(?:\s?(?:am|pm))?|\d{1,2}\s?(?:am|pm))\b").unwrap()
        );
        
        // Location patterns (simple city/place names)
        patterns.insert(
            EntityType::Location,
            Regex::new(r"\b(?:in|at|from|to)\s+([A-Z][a-z]+(?:\s+[A-Z][a-z]+)*)\b").unwrap()
        );
        
        // Email patterns
        patterns.insert(
            EntityType::Email,
            Regex::new(r"\b[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}\b").unwrap()
        );
        
        // Phone patterns
        patterns.insert(
            EntityType::Phone,
            Regex::new(r"\b(\+?\d{1,3}[-.\s]?)?\(?\d{3}\)?[-.\s]?\d{3}[-.\s]?\d{4}\b").unwrap()
        );
        
        // Number patterns
        patterns.insert(
            EntityType::Number,
            Regex::new(r"\b\d+\b").unwrap()
        );
        
        EntityExtractor { patterns }
    }
    
    pub fn extract(&self, text: &str) -> Vec<Entity> {
        let mut entities = Vec::new();
        
        for (entity_type, regex) in &self.patterns {
            for cap in regex.captures_iter(text) {
                if let Some(matched) = cap.get(0) {
                    let value = if entity_type == &EntityType::Location {
                        // For location, use capture group 1 if available
                        cap.get(1).map(|m| m.as_str()).unwrap_or(matched.as_str()).to_string()
                    } else {
                        matched.as_str().to_string()
                    };
                    
                    entities.push(Entity {
                        entity_type: entity_type.clone(),
                        value,
                        position: (matched.start(), matched.end()),
                    });
                }
            }
        }
        
        // Sort by position
        entities.sort_by_key(|e| e.position.0);
        
        entities
    }
    
    pub fn extract_by_type(&self, text: &str, entity_type: EntityType) -> Vec<Entity> {
        self.extract(text)
            .into_iter()
            .filter(|e| e.entity_type == entity_type)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_date_extraction() {
        let extractor = EntityExtractor::new();
        let entities = extractor.extract("Meeting on 2024-01-15 at 3pm");
        
        let dates: Vec<_> = entities.iter()
            .filter(|e| e.entity_type == EntityType::Date)
            .collect();
        
        assert!(!dates.is_empty());
    }
    
    #[test]
    fn test_email_extraction() {
        let extractor = EntityExtractor::new();
        let entities = extractor.extract("Contact me at john.doe@example.com");
        
        let emails: Vec<_> = entities.iter()
            .filter(|e| e.entity_type == EntityType::Email)
            .collect();
        
        assert_eq!(emails.len(), 1);
        assert_eq!(emails[0].value, "john.doe@example.com");
    }
}
