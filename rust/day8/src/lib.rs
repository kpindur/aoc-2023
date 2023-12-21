use std::{error::Error, collections::HashMap};

pub fn read_file(filename: &str) -> Result<String, Box<dyn Error>> {
    let contents: String = std::fs::read_to_string(filename).unwrap();
    Ok(contents)
}

pub fn part_1(contents: &str) -> Option<usize> {
    let orders = contents.lines().next().unwrap().chars();
    let mut directives: HashMap<String, Vec<String>> = HashMap::new();

    for line in contents.lines().skip(2) {
        let split: Vec<&str> = line.split('=')
                 .map(|word| word.trim())
                 .collect::<Vec<&str>>();

        let key: String = split[0].to_string();
        let val: Vec<String> = split[1]
            .trim_matches(|c| c == '(' || c == ')')
            .split(", ").map(|s| s.to_string())
            .collect();

        directives.insert(key, val);
    }

    let mut location: String = "AAA".to_string();
    let mut steps: usize = 0;
    let mut order = orders.clone();
    
    while location != "ZZZ" {
        location = match order.next() {
            Some('L')   => {
                steps += 1;
                directives.get(&location).unwrap()[0].clone()
            },
            Some('R')   => {
                steps += 1;
                directives.get(&location).unwrap()[1].clone()
            },
            _        => {
                order = orders.clone();
                location
            }
        }
    }

    Some(steps)
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

pub fn part_2(contents: &str) -> Option<usize> {
    let orders = contents.lines().next().unwrap().chars();
    let mut directives: HashMap<String, Vec<String>> = HashMap::new();
    let mut locs: Vec<String> = Vec::new();

    for line in contents.lines().skip(2) {
        let split: Vec<&str> = line.split('=')
                 .map(|word| word.trim())
                 .collect::<Vec<&str>>();

        let key: String = split[0].to_string();
        if key.chars().last().expect("Something went terribly wrong!") == 'A' {
            locs.push(key.clone());
        }

        let val: Vec<String> = split[1]
            .trim_matches(|c| c == '(' || c == ')')
            .split(", ").map(|s| s.to_string())
            .collect();

        directives.insert(key, val);
    }

    let mut all_steps: Vec<usize> = vec![0; locs.len()];

    for i in 0..locs.len() {
        let mut order = orders.clone();
        while locs[i].chars().last().unwrap() != 'Z' {
            locs[i] = match order.next() {
                Some('L')   => {
                    all_steps[i] += 1;
                    directives.get(&locs[i]).unwrap()[0].clone()
                },
                Some('R')   => {
                    all_steps[i] += 1;
                    directives.get(&locs[i]).unwrap()[1].clone()
                },
                _        => {
                    order = orders.clone();
                    locs[i].clone()
                }
            }
        }
    }

    Some(all_steps.iter().cloned().fold(1, |a, b| lcm(a, b)))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn part_1_works() {
        let input: String = read_file("./src/test-1.dat").unwrap();

        let result = part_1(&input);
        assert_eq!(result.unwrap(), 2);

        let input: String = read_file("./src/test-2.dat").unwrap();

        let result = part_1(&input);
        assert_eq!(result.unwrap(), 6);
    }

    #[test]
    fn part_2_works() {
        let input: String = read_file("./src/test-3.dat").unwrap();

        let result = part_2(&input);
        assert_eq!(result.unwrap(), 6);
    }
}
