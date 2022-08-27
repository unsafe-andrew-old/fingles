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

pub fn hash_from_string<const N: usize, const L: usize>(input: Chars) -> ShinglesHash<N, L> {
    let mut hash = [u32::MAX; N];

    let mut input_iter = input.into_iter();
    let mut buf = ['\x00'; L];
    for i in 0..L - 1 {
        if let Some(item) = input_iter.next() {
            buf[i] = item;
        }
    }

    for item in input_iter {
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
pub fn compare_hashes<const N: usize, const L: usize>(first: &ShinglesHash<N, L>, second: &ShinglesHash<N, L>) -> f64 {
    first.iter()
         .zip(second.iter())
         .filter(|pair| pair.0 == pair.1)
         .count() as f64 / N as f64
}
