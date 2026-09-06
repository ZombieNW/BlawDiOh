/// Smooths rms samples using "Exponential Moving Average"
/// Alpha (0 -> 1) is responseiveness
pub fn smooth(values: &[f32], alpha: f32) -> Vec<f32> {
    if values.is_empty() {
        return Vec::new();
    }

    let mut smoothed_values = Vec::with_capacity(values.len());
    let mut current_smoothed = values[0];
    smoothed_values.push(current_smoothed);

    for &raw_value in values.iter().skip(1) {
        current_smoothed = (alpha * raw_value) + ((1.0 - alpha) * current_smoothed);
        smoothed_values.push(current_smoothed);
    }

    return smoothed_values;
}
