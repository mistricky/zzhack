const WORDS_PER_MINUTE: f64 = 200.0;

/// Estimate reading time in whole minutes for a given word count.
/// Returns 0 for empty content and otherwise rounds up to at least 1 minute.
#[allow(dead_code)]
pub fn estimate_reading_minutes(word_count: usize) -> u32 {
    if word_count == 0 {
        return 0;
    }
    ((word_count as f64 / WORDS_PER_MINUTE).ceil() as u32).max(1)
}
