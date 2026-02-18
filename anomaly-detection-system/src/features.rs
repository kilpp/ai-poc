use crate::parser::NetworkEvent;

/// Extracts a numerical feature vector from a NetworkEvent.
///
/// Features:
/// 0: src_port
/// 1: dst_port
/// 2: bytes
/// 3: duration
/// 4: protocol (encoded as numeric)
/// 5: hour_of_day
pub fn extract_features(event: &NetworkEvent) -> Vec<f64> {
    let hour = event.timestamp.time().hour() as f64;

    vec![
        event.src_port as f64,
        event.dst_port as f64,
        event.bytes as f64,
        event.duration,
        event.protocol.as_f64(),
        hour,
    ]
}

use chrono::Timelike;

pub const NUM_FEATURES: usize = 6;

/// Online min-max normalizer that tracks running min/max per feature.
pub struct Normalizer {
    min: Vec<f64>,
    max: Vec<f64>,
    initialized: bool,
}

impl Normalizer {
    pub fn new() -> Self {
        Self {
            min: vec![f64::MAX; NUM_FEATURES],
            max: vec![f64::MIN; NUM_FEATURES],
            initialized: false,
        }
    }

    /// Update min/max bounds with a batch of samples.
    pub fn fit_batch(&mut self, data: &[Vec<f64>]) {
        for sample in data {
            for (i, &val) in sample.iter().enumerate() {
                if val < self.min[i] {
                    self.min[i] = val;
                }
                if val > self.max[i] {
                    self.max[i] = val;
                }
            }
        }
        self.initialized = true;
    }

    /// Update min/max bounds with a single sample.
    pub fn update(&mut self, sample: &[f64]) {
        for (i, &val) in sample.iter().enumerate() {
            if val < self.min[i] {
                self.min[i] = val;
            }
            if val > self.max[i] {
                self.max[i] = val;
            }
        }
        self.initialized = true;
    }

    /// Normalize a feature vector to [0, 1] range.
    pub fn normalize(&self, sample: &[f64]) -> Vec<f64> {
        if !self.initialized {
            return sample.to_vec();
        }

        sample
            .iter()
            .enumerate()
            .map(|(i, &val)| {
                let range = self.max[i] - self.min[i];
                if range == 0.0 {
                    0.5
                } else {
                    (val - self.min[i]) / range
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_line;

    #[test]
    fn test_extract_features() {
        let line = "2024-01-15T10:30:00 192.168.1.10 54321 10.0.0.1 443 TCP 1500 0.05";
        let event = parse_line(line).unwrap();
        let features = extract_features(&event);
        assert_eq!(features.len(), NUM_FEATURES);
        assert_eq!(features[0], 54321.0); // src_port
        assert_eq!(features[1], 443.0); // dst_port
        assert_eq!(features[2], 1500.0); // bytes
        assert!((features[3] - 0.05).abs() < 1e-9); // duration
        assert_eq!(features[4], 0.0); // TCP
        assert_eq!(features[5], 10.0); // hour
    }

    #[test]
    fn test_normalizer() {
        let mut norm = Normalizer::new();
        let data = vec![
            vec![0.0, 10.0, 100.0, 0.0, 0.0, 0.0],
            vec![100.0, 20.0, 200.0, 1.0, 1.0, 23.0],
        ];
        norm.fit_batch(&data);

        let result = norm.normalize(&[50.0, 15.0, 150.0, 0.5, 0.5, 12.0]);
        assert!((result[0] - 0.5).abs() < 1e-9);
        assert!((result[1] - 0.5).abs() < 1e-9);
    }

    #[test]
    fn test_normalizer_zero_range() {
        let mut norm = Normalizer::new();
        let data = vec![
            vec![5.0, 5.0, 5.0, 5.0, 5.0, 5.0],
            vec![5.0, 5.0, 5.0, 5.0, 5.0, 5.0],
        ];
        norm.fit_batch(&data);
        let result = norm.normalize(&[5.0, 5.0, 5.0, 5.0, 5.0, 5.0]);
        // All features have zero range, should return 0.5
        for v in &result {
            assert!((v - 0.5).abs() < 1e-9);
        }
    }
}
