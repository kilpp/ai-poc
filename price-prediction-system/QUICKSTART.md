# Quick Start Guide

## Installation

```bash
# Make sure Rust is installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone/navigate to the project
cd price-prediction-system

# Build the project
cargo build --release
```

## Running the Demo

```bash
cargo run --release
```

This will:
- Generate 500 synthetic real estate properties
- Train 4 different regression models
- Compare their performance
- Display evaluation metrics

## Running Tests

```bash
cargo test
```

## Using Custom Data

### 1. Prepare Your CSV

Create a CSV file with these columns:
```csv
area_sqft,bedrooms,bathrooms,stories,main_road,guestroom,basement,hot_water_heating,air_conditioning,parking,prefarea,furnishing_status,price
2000,3,2,2,1,0,1,1,1,2,1,2,500000
1500,2,1,1,1,0,0,1,1,1,0,1,350000
```

### 2. Load and Train

Modify [src/main.rs](src/main.rs):

```rust
// Instead of generating synthetic data:
let properties = DataLoader::load_csv("your_data.csv")?;
```

## Command Line Arguments (Future Enhancement)

Currently, the system runs with default parameters. To customize:

1. Edit the main.rs file
2. Adjust parameters like:
   - `test_ratio` in train_test_split (default: 0.2)
   - `alpha` in Ridge/Lasso (default: 1.0/0.1)
   - `learning_rate` in GradientDescent (default: 0.01)
   - `max_iter` for iterative algorithms

## Performance Benchmarks

On a typical modern laptop:
- Training time per model: < 100ms for 500 samples
- Prediction time: < 1ms for 100 samples
- Memory usage: < 50MB

## Troubleshooting

### Build Errors

```bash
# Clean and rebuild
cargo clean
cargo build
```

### Test Failures

```bash
# Run tests with output
cargo test -- --nocapture
```

### Performance Issues

For large datasets (> 10,000 samples):
- Use Ridge/Lasso instead of coordinate descent
- Consider feature selection to reduce dimensionality
- Use standardization instead of polynomial features

## Next Steps

1. **Try different models**: Experiment with various alpha values
2. **Feature engineering**: Add domain-specific interaction terms
3. **Cross-validation**: Implement k-fold validation for better estimates
4. **Visualization**: Export predictions to CSV and plot them
5. **Real data**: Use actual real estate datasets from Kaggle

## Example Workflows

### Workflow 1: Quick Prediction

```bash
# Generate sample data
cargo run --release

# Use the generated sample_data.csv
# Modify main.rs to load it
# Re-run to see results
```

### Workflow 2: Model Comparison

```bash
# Edit main.rs to try different hyperparameters
# Ridge with alpha: 0.1, 1.0, 10.0
# Lasso with alpha: 0.01, 0.1, 1.0
cargo run --release
```

### Workflow 3: Production Use

```rust
// In your Rust application:
use price_prediction_system::*;

// Load trained model (you'll need to implement save/load)
let mut model = LinearRegression::new();
// ... load weights ...

// Make predictions
let prediction = model.predict(&features)?;
```

## Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [ndarray Documentation](https://docs.rs/ndarray/)
- [Real Estate Datasets](https://www.kaggle.com/datasets?search=real+estate)

## Support

For issues and questions:
1. Check the README.md
2. Review EXAMPLES.md
3. Look at the test suite
4. Open an issue on GitHub
