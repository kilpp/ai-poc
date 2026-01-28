use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Movie {
    pub id: u32,
    pub title: String,
    pub genres: Vec<String>,
    pub year: u32,
    pub director: String,
    pub actors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rating {
    pub user_id: u32,
    pub movie_id: u32,
    pub rating: f64, // Explicit feedback (e.g., 1.0 to 5.0)
}

#[derive(Debug)]
pub struct Dataset {
    pub movies: HashMap<u32, Movie>,
    pub users: HashMap<u32, User>,
    pub ratings: Vec<Rating>,
}

impl Dataset {
    pub fn new() -> Self {
        Dataset {
            movies: HashMap::new(),
            users: HashMap::new(),
            ratings: Vec::new(),
        }
    }

    pub fn add_movie(&mut self, movie: Movie) {
        self.movies.insert(movie.id, movie);
    }

    pub fn add_user(&mut self, user: User) {
        self.users.insert(user.id, user);
    }

    pub fn add_rating(&mut self, rating: Rating) {
        self.ratings.push(rating);
    }

    pub fn get_user_ratings(&self, user_id: u32) -> Vec<&Rating> {
        self.ratings
            .iter()
            .filter(|r| r.user_id == user_id)
            .collect()
    }

    pub fn get_movie_ratings(&self, movie_id: u32) -> Vec<&Rating> {
        self.ratings
            .iter()
            .filter(|r| r.movie_id == movie_id)
            .collect()
    }
}
