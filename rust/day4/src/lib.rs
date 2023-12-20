use std::{error::Error, collections::HashMap};

pub fn read_file(filename: &str) -> Result<String, Box<dyn Error>> {
    let contents: String = std::fs::read_to_string(filename).unwrap();
    Ok(contents)
}

pub fn part_1(contents: &str) -> Option<usize> {
    let parse_str = |val: &str| -> usize { val.parse::<usize>().expect("Failed at parsing! NaN") };

    let cards: Vec<(Vec<usize>, Vec<usize>)> = contents.lines()
        .map(|line| {
             let (winning, numbers) = line.split_once(':').unwrap().1.split_once(" | ").unwrap();
             (winning.split_whitespace().map(|val| parse_str(val)).collect::<Vec<usize>>(),
              numbers.split_whitespace().map(|val| parse_str(val)).collect::<Vec<usize>>())
        }).collect();
    
    let overlapping: Vec<usize> = cards.iter()
        .map(|(winning, numbers)|
             numbers.iter().filter(|&x| winning.contains(x)).count()
        ).collect();

    let result: usize = overlapping.iter()
        .filter_map(|n| if *n == 0 { None } else { Some(n) })
        .map(|n| (2 as usize).pow((n-1).try_into().unwrap())).sum();

    Some(result)
}

pub fn part_2(contents: &str) -> Option<usize> {
    let parse_str = |val: &str| -> usize { val.parse::<usize>().expect("Failed at parsing! NaN") };
    let cards: Vec<(Vec<usize>, Vec<usize>)> = contents.lines()
        .map(|line| {
             let (winning, numbers) = line.split_once(':').unwrap().1.split_once(" | ").unwrap();
             (winning.split_whitespace().map(|val| parse_str(val)).collect::<Vec<usize>>(),
              numbers.split_whitespace().map(|val| parse_str(val)).collect::<Vec<usize>>())
        }).collect();
    
    let overlapping: Vec<usize> = cards.iter()
        .map(|(winning, numbers)|
             numbers.iter().filter(|&x| winning.contains(x)).count()
        ).collect();
    
    let mut result: HashMap<usize, usize> = HashMap::new();
    for i in 0..overlapping.len() {
        result.insert(i, 1);
    }
    
    for (i, overlap) in overlapping.iter().enumerate() {
        for _ in 0..*result.get(&i).expect("Failed at retrieving the number of cards! NaN") {
            for j in 1..=*overlap {
                match result.get_mut(&(i+j)) {
                    Some(value) => *value += 1,
                    None        => { result.insert(i+j, 1); () },
                };
            }
        }
    }

    Some(result.values().sum::<usize>())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn part_1_works() {
        let input: String = read_file("./src/test.dat").unwrap();

        let result = part_1(&input).unwrap();
        assert_eq!(result, 13);
    }

    #[test]
    fn part_2_works() {
        let input: String = read_file("./src/test.dat").unwrap();

        let result = part_2(&input).unwrap();
        assert_eq!(result, 30);
    }
}
