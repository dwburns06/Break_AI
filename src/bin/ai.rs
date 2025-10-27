use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::time::Instant;

#[derive(Clone, Copy)]
struct Word {
    mask: u32,
    idx: u16,
}

fn char_to_bit(c: char) -> u32 {
    1 << (c as u32 - 'a' as u32)
}

fn str_to_mask(s: &str) -> Option<u32> {
    let mut mask = 0u32;
    for c in s.chars() {
        let bit = char_to_bit(c);
        if mask & bit != 0 {
            return None; // duplicate letter
        }
        mask |= bit;
    }
    Some(mask)
}

fn find_combinations(words: &[Word], original_words: &[String]) -> Vec<[u16; 5]> {
    let mut results = Vec::new();
    let n = words.len();

    for i in 0..n {
        let w1 = words[i];
        for j in (i + 1)..n {
            let w2 = words[j];
            if w1.mask & w2.mask != 0 {
                continue;
            }
            let mask12 = w1.mask | w2.mask;

            for k in (j + 1)..n {
                let w3 = words[k];
                if mask12 & w3.mask != 0 {
                    continue;
                }
                let mask123 = mask12 | w3.mask;

                for l in (k + 1)..n {
                    let w4 = words[l];
                    if mask123 & w4.mask != 0 {
                        continue;
                    }
                    let mask1234 = mask123 | w4.mask;

                    for m in (l + 1)..n {
                        let w5 = words[m];
                        if mask1234 & w5.mask != 0 {
                            continue;
                        }

                        // Check if all 25 letters are covered
                        let final_mask = mask1234 | w5.mask;
                        if final_mask.count_ones() == 25 {
                            results.push([w1.idx, w2.idx, w3.idx, w4.idx, w5.idx]);
                        }
                    }
                }
            }
        }
    }

    results
}

fn main() -> std::io::Result<()> {
    let begin: Instant = Instant::now();

    // Read and filter words
    let file = File::open("words_alpha.txt")?;
    let reader = BufReader::new(file);

    let mut original_words = Vec::new();
    let mut words = Vec::new();

    for line in reader.lines() {
        let word = line?.trim().to_lowercase();

        // Only 5-letter words with unique letters
        if word.len() == 5 {
            if let Some(mask) = str_to_mask(&word) {
                let idx = original_words.len() as u16;
                original_words.push(word);
                words.push(Word { mask, idx });
            }
        }
    }

    let read_time: u128 = begin.elapsed().as_millis();

    // Sort by mask for better cache locality
    words.sort_unstable_by_key(|w| w.mask);

    let mid: Instant = Instant::now();

    // Find all combinations
    let combinations = find_combinations(&words, &original_words);

    let process_time: u128 = mid.elapsed().as_millis();

    // Write results
    let output = File::create("output.txt")?;
    let mut writer = BufWriter::new(output);

    for combo in &combinations {
        writeln!(
            writer,
            "{}, {}, {}, {}, {}",
            original_words[combo[0] as usize],
            original_words[combo[1] as usize],
            original_words[combo[2] as usize],
            original_words[combo[3] as usize],
            original_words[combo[4] as usize]
        )?;
    }

    writer.flush()?;

    println!("{:5}ms Reading time", read_time);
    println!("{:5}ms Processing time", process_time);
    println!("{:5}ms Total time", begin.elapsed().as_millis());
    println!("Found {} solutions", combinations.len());

    Ok(())
}