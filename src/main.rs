use std::{fs, time::Instant};
use std::collections::{HashMap, HashSet};


fn valid_word(text: String) -> Option<u32> {
    let mut letters: u32 = 0;
    let mut letters_len = 0;

    for c in text.chars() {
        if (letters_len  >= 5) || !c.is_ascii_lowercase() {
            return None;
        }
        if (letters & (1 << (c as u8 - b'a'))) != 0 {
            return None;
        }
        letters |= 1 << (c as u8 - b'a');
        letters_len += 1;
    }

    if letters_len != 5 {
        return None;
    }

    Some(letters)
}

fn get_vowel(letters: u32) -> usize {
    match letters & 1065233 {
        1 => 0,       // A
        16 => 1,      // E
        256 => 2,     // I
        16384 => 3,   // O
        1048576 => 4, // U
        0 => 5,       // NONE
        _ => 6,       // MULTI
    }
}

fn m_i(
    cur_solutions: u32,
    cur_iter: usize,
    mask: u32,
    total_solutions: &mut u32,
    io: &[usize; 5],
    ll: &[Vec<u32>; 7],
    wf: &HashMap<u32, u32>,
) -> Option<Vec<Vec<u32>>> {

    if cur_iter == 4 {
        let fnl: Vec<Vec<u32>> = ll[io[cur_iter]]
            .iter()
            .filter_map(|x| return if *x & mask == 0 { Some(vec![*x]) } else { None } )
            .collect();
        return if fnl.is_empty() { None } else {
            *total_solutions += cur_solutions + 1;
            Some(fnl)
        }
    }

    let words: Vec<u32> =
        if mask != 0 {
            ll[io[cur_iter]]
                .iter()
                .filter(|x| **x & mask == 0)
                .copied()
                .collect()
        } else {
            ll[io[cur_iter]].clone()
        };

    let mut ans: Vec<Vec<u32>> = Vec::new();
    for word in words {
        if let Some(mut success ) = m_i(cur_solutions + wf[&word] - 1, cur_iter + 1, mask | word, total_solutions, io, ll, wf) {
            success.iter_mut().for_each(|x| x.push(word));
            ans.extend(success);
        }
    }

    if ans.is_empty() { None } else { Some(ans) }
}

fn main() {
    let begin: Instant = Instant::now();

    let contents = fs::read_to_string("words_alpha.txt")
    .expect("Something went wrong reading the file");

    let mut wordlst: HashMap<u32, Vec<String>> = HashMap::new();
    let mut wordfreq: HashMap<u32, u32> = HashMap::new();
    let mut letterslst: [Vec<u32>; 7] = [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()];

    for cont in contents.lines() {
        match valid_word(cont.to_string()) {
            Some(word) => {
                if !wordlst.contains_key(&word) {
                    wordlst.insert(word, Vec::new());
                    letterslst[get_vowel(word)].push(word);
                }
                if let Some(val) = wordlst.get_mut(&word) { val.push(cont.to_string()); };
                *wordfreq.entry(word).or_insert(0) += 1;
            },
            None => {
                //pass
                }
        }
    }

    let read_time: u128 = begin.elapsed().as_millis();

    let sort_key = [5, 4, 1, 3, 2, 0, 6];
    let mut iter_order: Vec<[usize; 5]> = Vec::new();

    iter_order.push([4, 1, 3, 2, 0]); // a, e, i, o, u

    // 4 single vowels; 1 no vowels
    // for i in 0..5 {
    //     let mut temp = [4, 1, 3, 2, 0];
    //     temp[i] = 5;
    //     temp.sort_by_key(|&a| sort_key[a]);
    //     iter_order.push(temp);
    // }

    // 3 single vowels; 1 no vowel; 1 multi vowel
    // 3 single vowels; 2 no vowels; NONE
    for k in 6..7 {
        for i in 0..5 {
            for j in i..5 {
                let mut temp = [4, 1, 3, 2, 0];
                temp[i] = k;
                temp[j] = 5;
                temp.sort_by_key(|&a| sort_key[a]);
                iter_order.push(temp);
            }
        }
    }

    // 1 single vowels; 2 no vowels; 2 multi vowels
    for i in 0..5 {
        iter_order.push([5, 5, i, 6, 6]);
    }

    // 3 no vowels; 2 multi vowels - 5 no vowels
    // for i in 2..5 {
    //     let mut temp = [5, 5, 5, 6, 6];
    //     temp[i-1] = 5;
    //     temp[i] = 5;
    //     iter_order.push(temp);
    // }

    let mut ans: HashSet<[u32; 5]> = HashSet::new();
    let mut total_solutions: u32 = 0;

    let mid: Instant = Instant::now();

    for i in iter_order.iter() {
        if let Some(mut a) = m_i(0, 0, 0, &mut total_solutions, &i, &letterslst, &wordfreq) {
            a.iter_mut().for_each(|x| x.sort());
            let t: HashSet<[u32; 5]> = a.into_iter().map(|x| -> [u32; 5] { [x[0], x[1], x[2], x[3], x[4]] }).collect();
            ans.extend(t);
        }
    }

    let process_time: u128 = mid.elapsed().as_millis();

    #[allow(dead_code)]
    fn print_combinations_with_option<T: std::fmt::Display>(
        vecs: &[&Vec<T>],
        current: &mut Vec<String>,
        depth: usize,
    ) {
        if depth == vecs.len() {
            // Print the current combination
            println!("{}", current.join(" "));
            return;
        }

        // Iterate through all options at this depth
        for item in vecs[depth].iter() {
            current.push(item.to_string());
            print_combinations_with_option(vecs, current, depth + 1);
            current.pop();
        }
    }

    // for lst in ans.iter() {
    //     let vecs: Vec<&Vec<_>> = lst.iter()
    //         .map(|letters| &wordlst[letters])
    //         .collect();
    //
    //     print_combinations_with_option(&vecs, &mut vec![], 0);
    // }

    let c_contents = fs::read_to_string("expected.txt")
        .expect("Something went wrong reading the file");

    for cont in c_contents.lines() {
        let words_iter = cont.to_string();
        let mut cur: [u32; 5] = [0,0,0,0,0];
        let mut i = 0;
        for word in words_iter.split_whitespace() {
            if let Some(j) = valid_word(word.to_string()) {
                cur[i] = j;
                i += 1;
            }
        }

        cur.sort();

        if !ans.contains(&cur) {
            println!("MISSING: {}", words_iter);
        }
    }

    println!("{:5}ms Reading time", read_time);
    println!("{:5}ms Processing time", process_time);
    println!("{:5}ms Total time", begin.elapsed().as_millis());
    println!("Found {} unique solutions", ans.len());
    println!("Found {} total solutions", total_solutions);
}