use levenshtein::levenshtein;
use translit::{Gost779B, ToLatin, Language};


fn name_distance(a: &str, b: &str) -> usize {
    let trasliterator = Gost779B::new(Language::Ru);
    levenshtein(transliterator.to_latin(a), transliterator.to_latin(b))
}
