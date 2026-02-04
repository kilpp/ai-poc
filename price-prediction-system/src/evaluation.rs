use ndarray::Array1;
use crate::models::ModelMetrics;

/// Evaluation metrics for regression models
pub struct Evaluator;

impl Evaluator {
    /// Calculate R² score
    pub fn r2_score(y_true: &Array1<f64>, y_pred: &Array1<f64>) -> f64 {
        let mean = y_true.mean().unwrap();
        
        let ss_tot: f64 = y_true.iter().map(|&y| (y - mean).powi(2)).sum();
        let ss_res: f64 = y_true
            .iter()
            .zip(y_pred.iter())
            .map(|(&yt, &yp)| (yt - yp).powi(2))
            .sum();
        
        if ss_tot.abs() < 1e-10 {
            return 0.0;
        }
        
        1.0 - (ss_res / ss_tot)
    }

    /// Calculate Mean Squared Error
    pub fn mse(y_true: &Array1<f64>, y_pred: &Array1<f64>) -> f64 {
        let n = y_true.len() as f64;
        y_true
            .iter()
            .zip(y_pred.iter())
            .map(|(&yt, &yp)| (yt - yp).powi(2))
            .sum::<f64>()
            / n
    }

    /// Calculate Root Mean Squared Error
    pub fn rmse(y_true: &Array1<f64>, y_pred: &Array1<f64>) -> f64 {
        Self::mse(y_true, y_pred).sqrt()
    }

    /// Calculate Mean Absolute Error
    pub fn mae(y_true: &Array1<f64>, y_pred: &Array1<f64>) -> f64 {
        let n = y_true.len() as f64;
        y_true
            .iter()
            .zip(y_pred.iter())
            .map(|(&yt, &yp)| (yt - yp).abs())
            .sum::<f64>()
            / n
    }

    /// Calculate all metrics at once
    pub fn evaluate(y_true: &Array1<f64>, y_pred: &Array1<f64>) -> ModelMetrics {
        let r2 = Self::r2_score(y_true, y_pred);
        let mse = Self::mse(y_true, y_pred);
        let rmse = Self::rmse(y_true, y_pred);
        let mae = Self::mae(y_true, y_pred);
        
        ModelMetrics::new(r2, mse, rmse, mae)
    }
}
