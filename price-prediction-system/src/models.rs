use serde::{Deserialize, Serialize};

/// Represents a real estate property with all relevant features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Property {
    pub area_sqft: f64,
    pub bedrooms: f64,
    pub bathrooms: f64,
    pub stories: f64,
    pub main_road: f64,        // 0 or 1
    pub guestroom: f64,        // 0 or 1
    pub basement: f64,         // 0 or 1
    pub hot_water_heating: f64, // 0 or 1
    pub air_conditioning: f64, // 0 or 1
    pub parking: f64,          // number of parking spots
    pub prefarea: f64,         // 0 or 1 (preferred area)
    pub furnishing_status: f64, // 0=unfurnished, 1=semi-furnished, 2=furnished
    pub price: f64,            // Target variable
}

impl Property {
    /// Convert property to feature vector (without price)
    pub fn to_features(&self) -> Vec<f64> {
        vec![
            self.area_sqft,
            self.bedrooms,
            self.bathrooms,
            self.stories,
            self.main_road,
            self.guestroom,
            self.basement,
            self.hot_water_heating,
            self.air_conditioning,
            self.parking,
            self.prefarea,
            self.furnishing_status,
        ]
    }

    /// Get the number of features
    pub fn feature_count() -> usize {
        12
    }

    /// Get feature names
    pub fn feature_names() -> Vec<&'static str> {
        vec![
            "area_sqft",
            "bedrooms",
            "bathrooms",
            "stories",
            "main_road",
            "guestroom",
            "basement",
            "hot_water_heating",
            "air_conditioning",
            "parking",
            "prefarea",
            "furnishing_status",
        ]
    }
}

/// Metrics for model evaluation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMetrics {
    pub r2_score: f64,
    pub mse: f64,
    pub rmse: f64,
    pub mae: f64,
}

impl ModelMetrics {
    pub fn new(r2_score: f64, mse: f64, rmse: f64, mae: f64) -> Self {
        Self {
            r2_score,
            mse,
            rmse,
            mae,
        }
    }
}

/// Result of model comparison
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelComparison {
    pub model_name: String,
    pub metrics: ModelMetrics,
}
