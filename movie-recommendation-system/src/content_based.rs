use crate::models::{Dataset, Movie};
use std::collections::{HashMap, HashSet};

pub struct ContentBasedFilter<'a> {
    dataset: &'a Dataset,
}

impl<'a> ContentBasedFilter<'a> {
    pub fn new(dataset: &'a Dataset) -> Self {
        ContentBasedFilter { dataset }
    }

    /// Build a feature vector for a movie based on genres, director, and actors
    fn build_movie_features(&self, movie: &Movie) -> HashSet<String> {
        let mut features = HashSet::new();

        // Add genres
        for genre in &movie.genres {
            features.insert(format!("genre:{}", genre.to_lowercase()));
        }

        // Add director
        features.insert(format!("director:{}", movie.director.to_lowercase()));

        // Add actors
        for actor in &movie.actors {
            features.insert(format!("actor:{}", actor.to_lowercase()));
        }

        // Add year decade
        let decade = (movie.year / 10) * 10;
        features.insert(format!("decade:{}", decade));

        features
    }

    /// Calculate Jaccard similarity between two feature sets
    fn jaccard_similarity(&self, set1: &HashSet<String>, set2: &HashSet<String>) -> f64 {
        if set1.is_empty() && set2.is_empty() {
            return 0.0;
        }

        let intersection: HashSet<_> = set1.intersection(set2).collect();
        let union: HashSet<_> = set1.union(set2).collect();

        if union.is_empty() {
            0.0
        } else {
            intersection.len() as f64 / union.len() as f64
        }
    }

    /// Build user profile based on their rated movies
    fn build_user_profile(&self, user_id: u32) -> HashMap<String, f64> {
        let user_ratings = self.dataset.get_user_ratings(user_id);
        let mut feature_weights: HashMap<String, (f64, u32)> = HashMap::new(); // (total_weight, count)

        for rating in user_ratings {
            if let Some(movie) = self.dataset.movies.get(&rating.movie_id) {
                let features = self.build_movie_features(movie);
                for feature in features {
                    let entry = feature_weights.entry(feature).or_insert((0.0, 0));
                    entry.0 += rating.rating;
                    entry.1 += 1;
                }
            }
        }

        // Calculate average weight for each feature
        feature_weights
            .into_iter()
            .map(|(feature, (total, count))| (feature, total / count as f64))
            .collect()
    }

    /// Calculate content similarity between user profile and a movie
    fn calculate_movie_score(&self, user_profile: &HashMap<String, f64>, movie: &Movie) -> f64 {
        let movie_features = self.build_movie_features(movie);
        let mut score = 0.0;
        let mut total_weight = 0.0;

        for feature in movie_features {
            if let Some(&weight) = user_profile.get(&feature) {
                score += weight;
                total_weight += 1.0;
            }
        }

        if total_weight > 0.0 {
            score / total_weight
        } else {
            0.0
        }
    }

    /// Recommend movies based on content similarity to user's preferences
    pub fn recommend(&self, user_id: u32, n: usize) -> Vec<(u32, f64)> {
        let user_profile = self.build_user_profile(user_id);

        if user_profile.is_empty() {
            return Vec::new();
        }

        let user_rated_movies: HashSet<u32> = self
            .dataset
            .get_user_ratings(user_id)
            .iter()
            .map(|r| r.movie_id)
            .collect();

        let mut recommendations: Vec<(u32, f64)> = self
            .dataset
            .movies
            .iter()
            .filter(|(&id, _)| !user_rated_movies.contains(&id))
            .map(|(&id, movie)| (id, self.calculate_movie_score(&user_profile, movie)))
            .filter(|(_, score)| *score > 0.0)
            .collect();

        recommendations.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        recommendations.truncate(n);
        recommendations
    }

    /// Find similar movies based on content features
    pub fn find_similar_movies(&self, movie_id: u32, n: usize) -> Vec<(u32, f64)> {
        let movie = match self.dataset.movies.get(&movie_id) {
            Some(m) => m,
            None => return Vec::new(),
        };

        let movie_features = self.build_movie_features(movie);

        let mut similarities: Vec<(u32, f64)> = self
            .dataset
            .movies
            .iter()
            .filter(|(&id, _)| id != movie_id)
            .map(|(&id, other_movie)| {
                let other_features = self.build_movie_features(other_movie);
                let similarity = self.jaccard_similarity(&movie_features, &other_features);
                (id, similarity)
            })
            .filter(|(_, sim)| *sim > 0.0)
            .collect();

        similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        similarities.truncate(n);
        similarities
    }
}
