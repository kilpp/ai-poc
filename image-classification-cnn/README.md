# Image Classification System using CNNs

This project implements a CNN-based image classification system with data augmentation and transfer learning.

## Features
- Convolutional Neural Networks (CNNs) for image classification
- Data augmentation techniques (rotation, flipping, zooming, etc.)
- Transfer learning using pre-trained models (ResNet, VGG, MobileNet)
- Model training and evaluation
- Prediction on new images
- Performance visualization

## Project Structure

```
image-classification-cnn/
├── data/
│   ├── train/          # Training images organized by category
│   └── test/           # Test images organized by category
├── models/             # Trained model files
├── notebooks/          # Jupyter notebooks for exploration
├── src/                # Source code
│   ├── model.py        # CNN model definitions
│   ├── data_loader.py  # Data loading and preprocessing
│   ├── augmentation.py # Data augmentation utilities
│   └── train.py        # Training script
├── requirements.txt    # Python dependencies
└── README.md          # This file
```

## Installation

1. Create a virtual environment:
```bash
python -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate
```

2. Install dependencies:
```bash
pip install -r requirements.txt
```

## Usage

### Training a Model
```bash
python src/train.py --epochs 50 --batch-size 32 --model resnet50
```

### Making Predictions
```bash
python src/predict.py --image path/to/image.jpg --model models/best_model.h5
```

## Data Preparation

Organize your data in the following structure:
```
data/
├── train/
│   ├── class1/
│   ├── class2/
│   └── class3/
└── test/
    ├── class1/
    ├── class2/
    └── class3/
```

## Transfer Learning Models Available

- ResNet50
- VGG16
- MobileNetV2
- EfficientNetB0
- InceptionV3

## Data Augmentation Techniques

- Random rotation
- Horizontal/vertical flipping
- Zoom augmentation
- Brightness adjustment
- Contrast adjustment
- Width/height shifts

## Results

Model performance metrics will be saved to `models/` directory including:
- Training/validation accuracy plots
- Confusion matrix
- Classification report

## Notes

- GPU is recommended for faster training
- Adjust batch size based on available memory
- Use transfer learning for better results with limited data
