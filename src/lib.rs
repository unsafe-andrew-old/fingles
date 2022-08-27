#![feature(generic_arg_infer)]
mod hasher;
use crate::hasher::{Murmur3Hasher, SeedHasher};
use core::str::Chars;
use std::cmp;
use std::hash::{Hash, Hasher};

pub type ShinglesHash<const N: usize, const L: usize> = [u32; N];

fn shift<T: Copy, const L: usize>(arr: &mut [T; L]) {
    for i in 0..L - 1 {
        let next = arr[i + 1];
        arr[i] = next;
    }
}

/// Constructs a hash from a given char iterator
/// ```no_run
/// use fingles::*;
/// hash_from_string::<100, 5>("Hello world!".to_string().chars());
/// ```
pub fn hash_from_string<const N: usize, const L: usize>(mut input: Chars) -> ShinglesHash<N, L> {
    let mut hash = [u32::MAX; N];

    let mut buf = ['\x00'; L];
    for i in 0..L - 1 {
        if let Some(item) = input.next() {
            buf[i] = item;
        }
    }

    for item in input {
        buf[L - 1] = item;
        for i in 0..N {
            let mut hasher = Murmur3Hasher::with_seed(i as u32);
            for item in buf.iter() {
                item.hash(&mut hasher);
            }
            let shingle_hash = hasher.finish() as u32;
            hash[i] = cmp::min(hash[i], shingle_hash);
        }
        shift(&mut buf);
    }

    return hash;
}

/// Returns a distance between two hashes. The distance is a hamming distance.
/// ```no_run
/// use fingles::*;
/// let hello_world_hash = hash_from_string::<100, 5>("Hello world!".to_string().chars());
/// let bye_word_hash = hash_from_string::<100, 5>("Bye world!".to_string().chars());
/// println!("{}", compare_hashes::<100, 5>(&hello_world_hash, &bye_word_hash));
/// ```
pub fn compare_hashes<const N: usize, const L: usize>(
    first: &ShinglesHash<N, L>,
    second: &ShinglesHash<N, L>,
) -> f64 {
    first.iter()
         .zip(second.iter())
         .filter(|pair| pair.0 == pair.1)
         .count() as f64 / N as f64
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn plagiarism_check() {
        const HASH_LEN: usize = 100;
        const NGRAM_LEN: usize = 5;

        let original = "\
            “My sight is failing,” she said finally. “Even when I was young I could not have read what was written there. \
            But it appears to me that that wall looks different. Are the Seven Commandments the same as they used to be, \
            Benjamin?” For once Benjamin consented to break his rule, and he read out to her what was written on the wall. \
            There was nothing there now except a single Commandment. It ran:\
            ALL ANIMALS ARE EQUAL BUT SOME ANIMALS ARE MORE EQUAL THAN OTHERS";

        let plagiarism = "\
            “My sight is failing,” she said finally. “When I was young I could not have read what was written there. \
            But it appears to me that that wall looks different. Are the Seven Commandments the same as they used to be” \
            Benjamin read out to her what was written. There was nothing there now except a single Commandment. \
            It ran: ALL ANIMALS ARE EQUAL BUT SOME ANIMALS ARE MORE EQUAL THAN OTHERS";

        let other = "\
            Throughout the spring and summer they worked a sixty-hour week, and in August Napoleon announced that there \
            would be work on Sunday afternoons as well. This work was strictly voluntary, but any animal who absented \
            himself from it would have his rations reduced by half. Even so, it was found necessary to leave certain \
            tasks undone. The harvest was a little less successful than in the previous year, and two fields which \
            should have been sown with roots in the early summer were not sown because the ploughing had not been \
            completed early enough. It was possible to foresee that the coming winter would be a hard one.";

        let original_hash = hash_from_string::<HASH_LEN, NGRAM_LEN>(original.chars());
        let plagiarism_hash = hash_from_string::<HASH_LEN, NGRAM_LEN>(plagiarism.chars());
        let other_hash = hash_from_string::<HASH_LEN, NGRAM_LEN>(other.chars());

        assert!(
            compare_hashes::<_, NGRAM_LEN>(&original_hash, &plagiarism_hash) >
            compare_hashes::<_, NGRAM_LEN>(&original_hash, &other_hash)
        );
    }
}
