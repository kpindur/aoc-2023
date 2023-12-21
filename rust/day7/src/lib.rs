use std::{error::Error, collections::HashMap};

pub fn read_file(filename: &str) -> Result<String, Box<dyn Error>> {
    let contents: String = std::fs::read_to_string(filename).unwrap();
    Ok(contents)
}

pub fn part_1(contents: &str) -> Option<usize> {
    let tokens: [char; 13] = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];
    let token_map: HashMap<char, usize> = tokens.iter().enumerate().map(|(i, &token)| (token, i)).collect();
    
    let hands: Vec<_> = contents.lines()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .map(|chunk| (chunk[0].chars().map(|s| *token_map.get(&s).unwrap()).collect::<Vec<usize>>(),
                      chunk[1].parse::<usize>().expect("Parse failed! NaN")))
        .collect();

    let mut ranked: Vec<(usize, Vec<usize>, usize)> = Vec::new();
    for (hand, bid) in &hands {
        let mut counts: HashMap<usize, usize> = HashMap::new();
        for &c in hand {
            *counts.entry(c).or_insert(0) += 1;
        }
        let mut counts_vec = counts.values().cloned().collect::<Vec<usize>>();
        counts_vec.sort_by(|a, b| b.cmp(a));
        let rank: usize = match counts_vec.as_slice() {
            [5]             => 7,
            [4, 1]          => 6,
            [3, 2]          => 5,
            [3, 1, 1]       => 4,
            [2, 2, 1]       => 3,
            [2, 1, 1, 1]    => 2,
            [1, 1, 1, 1, 1] => 1,
            _               => panic!("Must have been the wind...")
        };
        ranked.push((rank, hand.clone(), bid.clone()));
    }
    
    ranked.sort_by_key(|&(first, ref second, _)| (first, second.clone()));

    let result: usize = (1..=ranked.len()).collect::<Vec<usize>>().iter().zip(ranked.iter())
        .map(|(i, (_, _, bid))| i*bid).sum();

    Some(result)
}

pub fn part_2(contents: &str) -> Option<usize> {
    let tokens: [char; 13] = ['J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A'];
    let token_map: HashMap<char, usize> = tokens.iter().enumerate().map(|(i, &token)| (token, i)).collect();

    let hands: Vec<_> = contents.lines()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .map(|chunk| (chunk[0].chars().map(|s| *token_map.get(&s).unwrap()).collect::<Vec<usize>>(),
                      chunk[1].parse::<usize>().expect("Parse failed! NaN")))
        .collect();

    let mut ranked: Vec<(usize, Vec<usize>, usize)> = Vec::new();
    for (hand, bid) in &hands {
        let mut counts: HashMap<usize, usize> = HashMap::new();
        for &c in hand {
            *counts.entry(c).or_insert(0) += 1;
        }

        match &counts.get(&0) {
             None           => (),
             Some(5)        => (),
             Some(jokers)  => {
                 let best = counts.iter().filter(|&(key, _)| *key != 0).max_by_key(|&(_, value)| value).unwrap().0.clone();
                 *counts.get_mut(&best).unwrap() += **jokers;
                 counts.remove(&0);
             },
        };

        let mut counts_vec = counts.values().cloned().collect::<Vec<usize>>();
        counts_vec.sort_by(|a, b| b.cmp(a));
        let rank: usize = match counts_vec.as_slice() {
            [5]             => 7,
            [4, 1]          => 6,
            [3, 2]          => 5,
            [3, 1, 1]       => 4,
            [2, 2, 1]       => 3,
            [2, 1, 1, 1]    => 2,
            [1, 1, 1, 1, 1] => 1,
            _               => panic!("Must have been the wind...")
        };

        ranked.push((rank, hand.clone(), bid.clone()));
    }

    ranked.sort_by_key(|&(first, ref second, _)| (first, second.clone()));

    let result: usize = (1..=ranked.len()).collect::<Vec<usize>>().iter().zip(ranked.iter())
        .map(|(i, (_, _, bid))| i*bid).sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn part_1_works() {
        let input: String = read_file("./src/test.dat").unwrap();

        let result = part_1(&input).unwrap();
        assert_eq!(result, 6440);
    }

    #[test]
    fn part_2_works() {
        let input: String = read_file("./src/test.dat").unwrap();

        let result = part_2(&input).unwrap();
        assert_eq!(result, 5905);
    }
}
