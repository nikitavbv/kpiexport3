use levenshtein::levenshtein;

fn name_distance(a: String, b: String) -> u32 {
    levenshtein(a, b)
}