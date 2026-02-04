use price_prediction_system::{
    DataLoader, Evaluator, FeatureEngine, GradientDescentRegression, LassoRegression,
    LinearRegression, ModelComparison, Regressor, RidgeRegression,
};

fn main() -> anyhow::Result<()> {
    println!("=== Real Estate Price Prediction System ===\n");

    // Generate synthetic data
    println!("Generating synthetic real estate data...");
    let properties = DataLoader::generate_synthetic_data(500, 42);
    println!("Generated {} properties\n", properties.len());

    // Save sample data to CSV
    DataLoader::save_csv(&properties, "sample_data.csv")?;
    println!("Saved sample data to sample_data.csv\n");

    // Convert to arrays
    let (x, y) = DataLoader::to_arrays(&properties);
    println!("Feature matrix shape: {} x {}", x.nrows(), x.ncols());
    println!("Target vector length: {}\n", y.len());

    // Feature engineering - normalize the data
    println!("Applying feature engineering...");
    let (x_normalized, _, _) = FeatureEngine::standardize(&x);
    println!("Features standardized\n");

    // Split data
    println!("Splitting data into train/test sets (80/20)...");
    let (x_train, y_train, x_test, y_test) =
        FeatureEngine::train_test_split(&x_normalized, &y, 0.2, 42);
    println!("Train set: {} samples", x_train.nrows());
    println!("Test set: {} samples\n", x_test.nrows());

    // Train and evaluate multiple models
    println!("=== Training Models ===\n");

    let mut models: Vec<Box<dyn Regressor>> = vec![
        Box::new(LinearRegression::new()),
        Box::new(RidgeRegression::new(1.0)),
        Box::new(LassoRegression::new(0.1).with_max_iter(500)),
        Box::new(GradientDescentRegression::new(0.01, 1000)),
    ];

    let mut results: Vec<ModelComparison> = Vec::new();

    for model in models.iter_mut() {
        println!("Training {}...", model.name());

        // Train the model
        model.fit(&x_train, &y_train)?;

        // Make predictions on test set
        let y_pred = model.predict(&x_test)?;

        // Evaluate
        let metrics = Evaluator::evaluate(&y_test, &y_pred);

        println!("  R² Score: {:.4}", metrics.r2_score);
        println!("  MSE: {:.2}", metrics.mse);
        println!("  RMSE: {:.2}", metrics.rmse);
        println!("  MAE: {:.2}\n", metrics.mae);

        results.push(ModelComparison {
            model_name: model.name().to_string(),
            metrics,
        });
    }

    // Find best model
    println!("=== Model Comparison ===\n");
    let best_model = results
        .iter()
        .max_by(|a, b| {
            a.metrics
                .r2_score
                .partial_cmp(&b.metrics.r2_score)
                .unwrap()
        })
        .unwrap();

    println!("Best Model: {}", best_model.model_name);
    println!("  R² Score: {:.4}", best_model.metrics.r2_score);
    println!("  RMSE: {:.2}", best_model.metrics.rmse);
    println!("  MAE: {:.2}\n", best_model.metrics.mae);

    // Display all results in a table
    println!("{:<30} {:<12} {:<12} {:<12} {:<12}", "Model", "R²", "MSE", "RMSE", "MAE");
    println!("{}", "-".repeat(78));
    for result in &results {
        println!(
            "{:<30} {:<12.4} {:<12.2} {:<12.2} {:<12.2}",
            result.model_name,
            result.metrics.r2_score,
            result.metrics.mse,
            result.metrics.rmse,
            result.metrics.mae
        );
    }

    // Demonstrate feature engineering with polynomial features
    println!("\n=== Advanced Feature Engineering ===\n");
    println!("Creating polynomial features (degree 2)...");
    let x_poly = FeatureEngine::polynomial_features(&x_train.slice(ndarray::s![0..10, ..]).to_owned());
    println!("Original features: {}", x_train.ncols());
    println!("Polynomial features: {}", x_poly.ncols());
    println!("(Note: Polynomial features can improve model performance but increase complexity)\n");

    // Demonstrate interaction features
    println!("Creating interaction features...");
    let interactions = vec![(0, 1), (0, 2), (1, 2)]; // area x bedrooms, area x bathrooms, bedrooms x bathrooms
    let x_interact = FeatureEngine::add_interactions(&x_train.slice(ndarray::s![0..10, ..]).to_owned(), &interactions);
    println!("Features with interactions: {}\n", x_interact.ncols());

    println!("=== Prediction Example ===\n");
    
    // Train the best model on a small sample for demonstration
    let mut best_regressor = LinearRegression::new();
    best_regressor.fit(&x_train, &y_train)?;
    
    // Make a prediction on the first test sample
    let sample = x_test.slice(ndarray::s![0..1, ..]).to_owned();
    let prediction = best_regressor.predict(&sample)?;
    
    println!("Sample prediction:");
    println!("  Predicted price: ${:.2}", prediction[0]);
    println!("  Actual price: ${:.2}", y_test[0]);
    println!("  Difference: ${:.2}", (prediction[0] - y_test[0]).abs());

    println!("\n=== Summary ===");
    println!("Successfully trained and compared {} regression models", results.len());
    println!("Data preprocessing and feature engineering demonstrated");
    println!("All models evaluated on unseen test data");
    
    Ok(())
}
