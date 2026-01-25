"""
Data loading and preprocessing utilities for image classification
"""

import os
import numpy as np
from tensorflow.keras.preprocessing import image
from augmentation import get_train_augmentation, get_val_augmentation, get_test_augmentation


def load_images_from_directory(directory, target_size=(224, 224)):
    """
    Load all images from a directory
    
    Args:
        directory: Path to directory containing images
        target_size: Target image size (height, width)
        
    Returns:
        List of image arrays and filenames
    """
    images = []
    filenames = []
    
    for filename in os.listdir(directory):
        if filename.lower().endswith(('.jpg', '.jpeg', '.png', '.bmp', '.gif')):
            img_path = os.path.join(directory, filename)
            img = image.load_img(img_path, target_size=target_size)
            img_array = image.img_to_array(img)
            images.append(img_array)
            filenames.append(filename)
    
    return np.array(images), filenames


def create_data_generators(train_dir, val_dir=None, batch_size=32, target_size=(224, 224)):
    """
    Create data generators for training and validation
    
    Args:
        train_dir: Path to training data directory
        val_dir: Path to validation data directory
        batch_size: Batch size for generators
        target_size: Target image size
        
    Returns:
        Tuple of (train_generator, val_generator)
    """
    train_datagen = get_train_augmentation()
    val_datagen = get_val_augmentation()
    
    train_generator = train_datagen.flow_from_directory(
        train_dir,
        target_size=target_size,
        batch_size=batch_size,
        class_mode='categorical'
    )
    
    if val_dir:
        val_generator = val_datagen.flow_from_directory(
            val_dir,
            target_size=target_size,
            batch_size=batch_size,
            class_mode='categorical'
        )
    else:
        val_generator = None
    
    return train_generator, val_generator


def get_class_names(directory):
    """
    Get class names from directory structure
    
    Args:
        directory: Path to data directory
        
    Returns:
        List of class names
    """
    return sorted([d for d in os.listdir(directory) if os.path.isdir(os.path.join(directory, d))])


def prepare_single_image(image_path, target_size=(224, 224)):
    """
    Prepare a single image for prediction
    
    Args:
        image_path: Path to image file
        target_size: Target image size
        
    Returns:
        Image array ready for prediction
    """
    img = image.load_img(image_path, target_size=target_size)
    img_array = image.img_to_array(img)
    img_array = img_array / 255.0  # Normalize
    img_array = np.expand_dims(img_array, axis=0)  # Add batch dimension
    
    return img_array
