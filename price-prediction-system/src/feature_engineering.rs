use ndarray::{Array1, Array2, Axis};
use ndarray_stats::QuantileExt;

/// Feature engineering and preprocessing utilities
pub struct FeatureEngine;

impl FeatureEngine {
    /// Normalize features using min-max scaling
    pub fn normalize(data: &Array2<f64>) -> (Array2<f64>, Vec<f64>, Vec<f64>) {
        let mut mins = Vec::new();
        let mut maxs = Vec::new();
        let mut normalized = data.clone();

        for i in 0..data.ncols() {
            let col = data.column(i);
            let min = *col.min().unwrap();
            let max = *col.max().unwrap();
            
            mins.push(min);
            maxs.push(max);
            
            if (max - min).abs() > 1e-10 {
                for j in 0..data.nrows() {
                    normalized[[j, i]] = (data[[j, i]] - min) / (max - min);
                }
            }
        }

        (normalized, mins, maxs)
    }

    /// Standardize features using z-score normalization
    pub fn standardize(data: &Array2<f64>) -> (Array2<f64>, Vec<f64>, Vec<f64>) {
        let mut means = Vec::new();
        let mut stds = Vec::new();
        let mut standardized = data.clone();

        for i in 0..data.ncols() {
            let col = data.column(i);
            let mean = col.mean().unwrap();
            let std = col.std(0.0);
            
            means.push(mean);
            stds.push(std);
            
            if std > 1e-10 {
                for j in 0..data.nrows() {
                    standardized[[j, i]] = (data[[j, i]] - mean) / std;
                }
            }
        }

        (standardized, means, stds)
    }

    /// Create polynomial features (degree 2)
    pub fn polynomial_features(data: &Array2<f64>) -> Array2<f64> {
        let n_rows = data.nrows();
        let n_cols = data.ncols();
        
        // Original features + squared features + interaction terms
        let n_new_features = n_cols + n_cols + (n_cols * (n_cols - 1)) / 2;
        let mut poly_data = Array2::<f64>::zeros((n_rows, n_new_features));
        
        let mut col_idx = 0;
        
        // Copy original features
        for i in 0..n_cols {
            for j in 0..n_rows {
                poly_data[[j, col_idx]] = data[[j, i]];
            }
            col_idx += 1;
        }
        
        // Add squared features
        for i in 0..n_cols {
            for j in 0..n_rows {
                poly_data[[j, col_idx]] = data[[j, i]] * data[[j, i]];
            }
            col_idx += 1;
        }
        
        // Add interaction terms
        for i in 0..n_cols {
            for k in (i + 1)..n_cols {
                for j in 0..n_rows {
                    poly_data[[j, col_idx]] = data[[j, i]] * data[[j, k]];
                }
                col_idx += 1;
            }
        }
        
        poly_data
    }

    /// Add interaction features between selected columns
    pub fn add_interactions(data: &Array2<f64>, col_pairs: &[(usize, usize)]) -> Array2<f64> {
        let n_rows = data.nrows();
        let n_cols = data.ncols();
        let n_interactions = col_pairs.len();
        
        let mut new_data = Array2::<f64>::zeros((n_rows, n_cols + n_interactions));
        
        // Copy original data
        for i in 0..n_cols {
            for j in 0..n_rows {
                new_data[[j, i]] = data[[j, i]];
            }
        }
        
        // Add interaction terms
        for (idx, &(col1, col2)) in col_pairs.iter().enumerate() {
            for j in 0..n_rows {
                new_data[[j, n_cols + idx]] = data[[j, col1]] * data[[j, col2]];
            }
        }
        
        new_data
    }

    /// Split data into train and test sets
    pub fn train_test_split(
        x: &Array2<f64>,
        y: &Array1<f64>,
        test_ratio: f64,
        seed: u64,
    ) -> (Array2<f64>, Array1<f64>, Array2<f64>, Array1<f64>) {
        use rand::seq::SliceRandom;
        use rand::SeedableRng;
        
        let n = x.nrows();
        let n_test = (n as f64 * test_ratio) as usize;
        let n_train = n - n_test;
        
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let mut indices: Vec<usize> = (0..n).collect();
        indices.shuffle(&mut rng);
        
        let train_indices = &indices[..n_train];
        let test_indices = &indices[n_train..];
        
        let x_train = x.select(Axis(0), train_indices);
        let y_train = y.select(Axis(0), train_indices);
        let x_test = x.select(Axis(0), test_indices);
        let y_test = y.select(Axis(0), test_indices);
        
        (x_train, y_train, x_test, y_test)
    }
}
