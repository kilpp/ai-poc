use crate::collaborative_filtering::CollaborativeFilter;
use crate::content_based::ContentBasedFilter;
use crate::models::Dataset;

pub struct HybridRecommender<'a> {
    dataset: &'a Dataset,
    collaborative: CollaborativeFilter<'a>,
    content_based: ContentBasedFilter<'a>,
}

#[derive(Debug, Clone, Copy)]
pub enum HybridStrategy {
    /// Weighted average of collaborative and content-based scores
    Weighted { collaborative_weight: f64, content_weight: f64 },
    /// Switch between methods based on data availability
    Switching,
    /// Combine both methods and re-rank
    Mixed,
}

impl<'a> HybridRecommender<'a> {
    pub fn new(dataset: &'a Dataset) -> Self {
        HybridRecommender {
            dataset,
            collaborative: CollaborativeFilter::new(dataset),
            content_based: ContentBasedFilter::new(dataset),
        }
    }

    /// Recommend using weighted hybrid approach
    pub fn recommend_weighted(
        &self,
        user_id: u32,
        n: usize,
        collaborative_weight: f64,
        content_weight: f64,
    ) -> Vec<(u32, f64)> {
        let collab_recs = self.collaborative.recommend_item_based(user_id, n * 2);
        let content_recs = self.content_based.recommend(user_id, n * 2);

        // Normalize weights
        let total_weight = collaborative_weight + content_weight;
        let norm_collab_weight = collaborative_weight / total_weight;
        let norm_content_weight = content_weight / total_weight;

        // Combine scores
        let mut combined_scores = std::collections::HashMap::new();

        // Normalize collaborative scores to 0-1 range
        let max_collab_score = collab_recs.first().map(|(_, s)| *s).unwrap_or(1.0);
        for (movie_id, score) in collab_recs {
            let normalized_score = if max_collab_score > 0.0 {
                score / max_collab_score
            } else {
                0.0
            };
            *combined_scores.entry(movie_id).or_insert(0.0) += normalized_score * norm_collab_weight;
        }

        // Normalize content scores to 0-1 range
        let max_content_score = content_recs.first().map(|(_, s)| *s).unwrap_or(1.0);
        for (movie_id, score) in content_recs {
            let normalized_score = if max_content_score > 0.0 {
                score / max_content_score
            } else {
                0.0
            };
            *combined_scores.entry(movie_id).or_insert(0.0) += normalized_score * norm_content_weight;
        }

        let mut recommendations: Vec<(u32, f64)> = combined_scores.into_iter().collect();
        recommendations.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        recommendations.truncate(n);
        recommendations
    }

    /// Recommend using switching hybrid approach
    /// Uses collaborative filtering if enough rating data exists, otherwise falls back to content-based
    pub fn recommend_switching(&self, user_id: u32, n: usize) -> Vec<(u32, f64)> {
        let user_ratings = self.dataset.get_user_ratings(user_id);

        // If user has enough ratings (>= 5), use collaborative filtering
        // Otherwise use content-based filtering
        if user_ratings.len() >= 5 {
            self.collaborative.recommend_item_based(user_id, n)
        } else {
            self.content_based.recommend(user_id, n)
        }
    }

    /// Recommend using mixed hybrid approach
    /// Combines results from both methods and re-ranks based on frequency and scores
    pub fn recommend_mixed(&self, user_id: u32, n: usize) -> Vec<(u32, f64)> {
        let collab_recs = self.collaborative.recommend_item_based(user_id, n);
        let content_recs = self.content_based.recommend(user_id, n);

        let mut movie_scores = std::collections::HashMap::new();

        // Add collaborative filtering results
        for (i, (movie_id, score)) in collab_recs.iter().enumerate() {
            let rank_score = 1.0 / (i + 1) as f64;
            *movie_scores.entry(*movie_id).or_insert(0.0) += rank_score + score * 0.5;
        }

        // Add content-based filtering results
        for (i, (movie_id, score)) in content_recs.iter().enumerate() {
            let rank_score = 1.0 / (i + 1) as f64;
            *movie_scores.entry(*movie_id).or_insert(0.0) += rank_score + score * 0.5;
        }

        let mut recommendations: Vec<(u32, f64)> = movie_scores.into_iter().collect();
        recommendations.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        recommendations.truncate(n);
        recommendations
    }

    /// General recommend method that uses the specified strategy
    pub fn recommend(&self, user_id: u32, n: usize, strategy: HybridStrategy) -> Vec<(u32, f64)> {
        match strategy {
            HybridStrategy::Weighted {
                collaborative_weight,
                content_weight,
            } => self.recommend_weighted(user_id, n, collaborative_weight, content_weight),
            HybridStrategy::Switching => self.recommend_switching(user_id, n),
            HybridStrategy::Mixed => self.recommend_mixed(user_id, n),
        }
    }
}
