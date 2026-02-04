use ndarray::{Array1, Array2};
use crate::error::{PredictionError, Result};

/// Trait for regression models
pub trait Regressor {
    fn fit(&mut self, x: &Array2<f64>, y: &Array1<f64>) -> Result<()>;
    fn predict(&self, x: &Array2<f64>) -> Result<Array1<f64>>;
    fn name(&self) -> &str;
}

/// Linear Regression using Normal Equation
pub struct LinearRegression {
    weights: Option<Array1<f64>>,
}

impl LinearRegression {
    pub fn new() -> Self {
        Self { weights: None }
    }

    fn add_bias(x: &Array2<f64>) -> Array2<f64> {
        let n_rows = x.nrows();
        let n_cols = x.ncols();
        let mut x_with_bias = Array2::<f64>::ones((n_rows, n_cols + 1));
        
        for i in 0..n_rows {
            for j in 0..n_cols {
                x_with_bias[[i, j + 1]] = x[[i, j]];
            }
        }
        
        x_with_bias
    }
}

impl Regressor for LinearRegression {
    fn fit(&mut self, x: &Array2<f64>, y: &Array1<f64>) -> Result<()> {
        let x_with_bias = Self::add_bias(x);
        
        // Normal equation: w = (X^T X)^-1 X^T y
        let xt = x_with_bias.t();
        let xtx = xt.dot(&x_with_bias);
        
        // Simple matrix inversion for small matrices
        let xtx_inv = Self::pseudo_inverse(&xtx)?;
        let xty = xt.dot(y);
        
        self.weights = Some(xtx_inv.dot(&xty));
        Ok(())
    }

    fn predict(&self, x: &Array2<f64>) -> Result<Array1<f64>> {
        match &self.weights {
            Some(w) => {
                let x_with_bias = Self::add_bias(x);
                Ok(x_with_bias.dot(w))
            }
            None => Err(PredictionError::ModelError(
                "Model not trained yet".to_string(),
            )),
        }
    }

    fn name(&self) -> &str {
        "Linear Regression"
    }
}

impl LinearRegression {
    fn pseudo_inverse(matrix: &Array2<f64>) -> Result<Array2<f64>> {
        let n = matrix.nrows();
        if n != matrix.ncols() {
            return Err(PredictionError::DimensionMismatch(
                "Matrix must be square".to_string(),
            ));
        }

        // Simple Gaussian elimination for small matrices
        let mut a = matrix.clone();
        let mut inv = Array2::<f64>::eye(n);

        for i in 0..n {
            // Find pivot
            let mut max_row = i;
            for k in (i + 1)..n {
                if a[[k, i]].abs() > a[[max_row, i]].abs() {
                    max_row = k;
                }
            }

            // Swap rows
            if max_row != i {
                for j in 0..n {
                    a.swap([i, j], [max_row, j]);
                    inv.swap([i, j], [max_row, j]);
                }
            }

            let pivot = a[[i, i]];
            if pivot.abs() < 1e-10 {
                return Err(PredictionError::ModelError(
                    "Matrix is singular".to_string(),
                ));
            }

            // Scale pivot row
            for j in 0..n {
                a[[i, j]] /= pivot;
                inv[[i, j]] /= pivot;
            }

            // Eliminate column
            for k in 0..n {
                if k != i {
                    let factor = a[[k, i]];
                    for j in 0..n {
                        a[[k, j]] -= factor * a[[i, j]];
                        inv[[k, j]] -= factor * inv[[i, j]];
                    }
                }
            }
        }

        Ok(inv)
    }
}

/// Ridge Regression (L2 regularization)
pub struct RidgeRegression {
    weights: Option<Array1<f64>>,
    alpha: f64,
}

impl RidgeRegression {
    pub fn new(alpha: f64) -> Self {
        Self {
            weights: None,
            alpha,
        }
    }

    fn add_bias(x: &Array2<f64>) -> Array2<f64> {
        LinearRegression::add_bias(x)
    }
}

impl Regressor for RidgeRegression {
    fn fit(&mut self, x: &Array2<f64>, y: &Array1<f64>) -> Result<()> {
        let x_with_bias = Self::add_bias(x);
        let n_features = x_with_bias.ncols();
        
        // Ridge: w = (X^T X + α I)^-1 X^T y
        let xt = x_with_bias.t();
        let xtx = xt.dot(&x_with_bias);
        
        // Add regularization term
        let mut xtx_reg = xtx;
        for i in 0..n_features {
            xtx_reg[[i, i]] += self.alpha;
        }
        
        let xtx_inv = LinearRegression::pseudo_inverse(&xtx_reg)?;
        let xty = xt.dot(y);
        
        self.weights = Some(xtx_inv.dot(&xty));
        Ok(())
    }

    fn predict(&self, x: &Array2<f64>) -> Result<Array1<f64>> {
        match &self.weights {
            Some(w) => {
                let x_with_bias = Self::add_bias(x);
                Ok(x_with_bias.dot(w))
            }
            None => Err(PredictionError::ModelError(
                "Model not trained yet".to_string(),
            )),
        }
    }

    fn name(&self) -> &str {
        "Ridge Regression"
    }
}

/// Lasso Regression (L1 regularization) using Coordinate Descent
pub struct LassoRegression {
    weights: Option<Array1<f64>>,
    alpha: f64,
    max_iter: usize,
}

impl LassoRegression {
    pub fn new(alpha: f64) -> Self {
        Self {
            weights: None,
            alpha,
            max_iter: 1000,
        }
    }

    pub fn with_max_iter(mut self, max_iter: usize) -> Self {
        self.max_iter = max_iter;
        self
    }

    fn add_bias(x: &Array2<f64>) -> Array2<f64> {
        LinearRegression::add_bias(x)
    }

    fn soft_threshold(x: f64, lambda: f64) -> f64 {
        if x > lambda {
            x - lambda
        } else if x < -lambda {
            x + lambda
        } else {
            0.0
        }
    }
}

impl Regressor for LassoRegression {
    fn fit(&mut self, x: &Array2<f64>, y: &Array1<f64>) -> Result<()> {
        let x_with_bias = Self::add_bias(x);
        let n_samples = x_with_bias.nrows();
        let n_features = x_with_bias.ncols();
        
        let mut w = Array1::<f64>::zeros(n_features);
        
        // Coordinate descent
        for _ in 0..self.max_iter {
            for j in 0..n_features {
                let mut residual = y.clone();
                
                // Compute residual without feature j
                for k in 0..n_features {
                    if k != j {
                        for i in 0..n_samples {
                            residual[i] -= w[k] * x_with_bias[[i, k]];
                        }
                    }
                }
                
                // Compute correlation
                let mut rho = 0.0;
                for i in 0..n_samples {
                    rho += x_with_bias[[i, j]] * residual[i];
                }
                
                // Update weight with soft thresholding
                let x_j_norm = x_with_bias.column(j).dot(&x_with_bias.column(j));
                
                if j == 0 {
                    // Don't regularize bias term
                    w[j] = rho / x_j_norm;
                } else {
                    w[j] = Self::soft_threshold(rho, self.alpha * n_samples as f64) / x_j_norm;
                }
            }
        }
        
        self.weights = Some(w);
        Ok(())
    }

    fn predict(&self, x: &Array2<f64>) -> Result<Array1<f64>> {
        match &self.weights {
            Some(w) => {
                let x_with_bias = Self::add_bias(x);
                Ok(x_with_bias.dot(w))
            }
            None => Err(PredictionError::ModelError(
                "Model not trained yet".to_string(),
            )),
        }
    }

    fn name(&self) -> &str {
        "Lasso Regression"
    }
}

/// Gradient Descent Regression
pub struct GradientDescentRegression {
    weights: Option<Array1<f64>>,
    learning_rate: f64,
    max_iter: usize,
}

impl GradientDescentRegression {
    pub fn new(learning_rate: f64, max_iter: usize) -> Self {
        Self {
            weights: None,
            learning_rate,
            max_iter,
        }
    }

    fn add_bias(x: &Array2<f64>) -> Array2<f64> {
        LinearRegression::add_bias(x)
    }
}

impl Regressor for GradientDescentRegression {
    fn fit(&mut self, x: &Array2<f64>, y: &Array1<f64>) -> Result<()> {
        let x_with_bias = Self::add_bias(x);
        let n_samples = x_with_bias.nrows() as f64;
        let n_features = x_with_bias.ncols();
        
        let mut w = Array1::<f64>::zeros(n_features);
        
        for _ in 0..self.max_iter {
            let predictions = x_with_bias.dot(&w);
            let errors = &predictions - y;
            
            // Gradient: (2/n) * X^T * (predictions - y)
            let gradient = x_with_bias.t().dot(&errors) * (2.0 / n_samples);
            
            // Update weights
            w = w - &gradient * self.learning_rate;
        }
        
        self.weights = Some(w);
        Ok(())
    }

    fn predict(&self, x: &Array2<f64>) -> Result<Array1<f64>> {
        match &self.weights {
            Some(w) => {
                let x_with_bias = Self::add_bias(x);
                Ok(x_with_bias.dot(w))
            }
            None => Err(PredictionError::ModelError(
                "Model not trained yet".to_string(),
            )),
        }
    }

    fn name(&self) -> &str {
        "Gradient Descent Regression"
    }
}
