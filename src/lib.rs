use std::collections::HashSet;
use std::io;
use memmap::Mmap;

pub fn get_words<'a>(
    file: &'a Mmap,
    letter_to_words: &mut [Vec<u32>; 26],
    words: &mut HashSet<u32>,
) -> io::Result<()> {

    const SORTED_LETTERS: [usize; 26] =
        [0,4,18,8,14,17,20,13,11,19,24,2,3,7,12,15,6,10,1,22,5,21,25,9,23,16];

    let mut word_begin: usize = 0;
    let mut bits: u32 = 0;
    for (i, char) in file.iter().enumerate() {
        let char = *char;

        if char > 13 {
            bits |= 1 << (char as u32 - 'a' as u32);
            continue;
        }

        let len: usize = i - word_begin;
        let cur_bits = bits;
        word_begin = i + 1;
        bits = 0;

        if len != 5 {
            continue;
        }
        if cur_bits.count_ones() as usize != 5 {
            continue;
        }
        if words.contains(&cur_bits) {
            continue;
        }

        let mut cur_min: usize = 26;
        let mut low_bits = cur_bits;

        while low_bits != 0 {
            let letter = low_bits.trailing_zeros() as usize;
            if cur_min > SORTED_LETTERS[letter] {
                cur_min = SORTED_LETTERS[letter];
            }
            low_bits ^= 1 << letter;
        }

        letter_to_words[cur_min].push(cur_bits);
        words.insert(cur_bits);
    }

    Ok(())
}