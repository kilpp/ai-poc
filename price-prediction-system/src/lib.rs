pub mod models;
pub mod error;
pub mod data;
pub mod feature_engineering;
pub mod regression;
pub mod evaluation;

pub use models::{Property, ModelMetrics, ModelComparison};
pub use error::{PredictionError, Result};
pub use data::DataLoader;
pub use feature_engineering::FeatureEngine;
pub use regression::{Regressor, LinearRegression, RidgeRegression, LassoRegression, GradientDescentRegression};
pub use evaluation::Evaluator;
