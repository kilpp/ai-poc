use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Intent {
    Greeting,
    Farewell,
    Question,
    BookAppointment,
    CheckWeather,
    OrderFood,
    GetHelp,
    Unknown,
}

pub struct IntentRecognizer {
    patterns: HashMap<Intent, Vec<Vec<String>>>,
}

impl IntentRecognizer {
    pub fn new() -> Self {
        let mut patterns = HashMap::new();
        
        // Greeting patterns
        patterns.insert(Intent::Greeting, vec![
            vec!["hello".to_string()],
            vec!["hi".to_string()],
            vec!["hey".to_string()],
            vec!["good".to_string(), "morning".to_string()],
            vec!["good".to_string(), "afternoon".to_string()],
            vec!["good".to_string(), "evening".to_string()],
        ]);
        
        // Farewell patterns
        patterns.insert(Intent::Farewell, vec![
            vec!["bye".to_string()],
            vec!["goodbye".to_string()],
            vec!["see".to_string(), "you".to_string()],
            vec!["exit".to_string()],
            vec!["quit".to_string()],
        ]);
        
        // Question patterns
        patterns.insert(Intent::Question, vec![
            vec!["what".to_string()],
            vec!["when".to_string()],
            vec!["where".to_string()],
            vec!["how".to_string()],
            vec!["why".to_string()],
            vec!["who".to_string()],
        ]);
        
        // BookAppointment patterns
        patterns.insert(Intent::BookAppointment, vec![
            vec!["book".to_string(), "appointment".to_string()],
            vec!["schedule".to_string(), "meeting".to_string()],
            vec!["make".to_string(), "appointment".to_string()],
            vec!["reserve".to_string()],
        ]);
        
        // CheckWeather patterns
        patterns.insert(Intent::CheckWeather, vec![
            vec!["weather".to_string()],
            vec!["temperature".to_string()],
            vec!["forecast".to_string()],
            vec!["rain".to_string()],
            vec!["sunny".to_string()],
        ]);
        
        // OrderFood patterns
        patterns.insert(Intent::OrderFood, vec![
            vec!["order".to_string(), "food".to_string()],
            vec!["order".to_string(), "pizza".to_string()],
            vec!["want".to_string(), "to".to_string(), "eat".to_string()],
            vec!["hungry".to_string()],
        ]);
        
        // Help patterns
        patterns.insert(Intent::GetHelp, vec![
            vec!["help".to_string()],
            vec!["assist".to_string()],
            vec!["support".to_string()],
        ]);
        
        IntentRecognizer { patterns }
    }
    
    pub fn recognize(&self, text: &str) -> Intent {
        let text_lower = text.to_lowercase();
        let words: Vec<String> = text_lower
            .split_whitespace()
            .map(|s| s.trim_matches(|c: char| !c.is_alphanumeric()).to_string())
            .filter(|s| !s.is_empty())
            .collect();
        
        let mut best_match = Intent::Unknown;
        let mut best_score = 0;
        
        for (intent, pattern_list) in &self.patterns {
            for pattern in pattern_list {
                let score = self.calculate_match_score(&words, pattern);
                if score > best_score {
                    best_score = score;
                    best_match = intent.clone();
                }
            }
        }
        
        // If no match found, return Unknown
        if best_score == 0 {
            Intent::Unknown
        } else {
            best_match
        }
    }
    
    fn calculate_match_score(&self, words: &[String], pattern: &[String]) -> usize {
        if pattern.is_empty() {
            return 0;
        }
        
        let mut score = 0;
        let mut matched_words = 0;
        
        // Check if all pattern words are present in the input
        for pattern_word in pattern {
            if words.contains(pattern_word) {
                matched_words += 1;
            }
        }
        
        // Only score if we match ALL pattern words (more specific matching)
        if matched_words == pattern.len() {
            // Base score is number of matched words squared (favor longer patterns)
            score = matched_words * matched_words * 10;
            
            // Bonus for consecutive matches
            for i in 0..words.len() {
                let remaining = words.len() - i;
                if remaining >= pattern.len() {
                    let consecutive = words[i..i+pattern.len()]
                        .iter()
                        .zip(pattern.iter())
                        .filter(|(w, p)| w == p)
                        .count();
                    
                    if consecutive == pattern.len() {
                        score += 20; // Big bonus for exact phrase match
                        break;
                    }
                }
            }
        }
        
        score
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_greeting_intent() {
        let recognizer = IntentRecognizer::new();
        assert_eq!(recognizer.recognize("Hello there"), Intent::Greeting);
        assert_eq!(recognizer.recognize("Hi!"), Intent::Greeting);
    }
    
    #[test]
    fn test_appointment_intent() {
        let recognizer = IntentRecognizer::new();
        assert_eq!(
            recognizer.recognize("I want to book an appointment"),
            Intent::BookAppointment
        );
    }
}
