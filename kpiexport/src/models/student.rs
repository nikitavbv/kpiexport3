use levenshtein::levenshtein;

fn name_distance(a: &str, b: &str) -> usize {
    levenshtein(a, b)
}