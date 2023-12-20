use std::error::Error;

pub fn read_file(filename: &str) -> Result<String, Box<dyn Error>> {
    let contents: String = std::fs::read_to_string(filename).unwrap();
    Ok(contents)
}

pub fn part_1(contents: &str) -> Option<usize> {
    let seeds: Vec<usize> = contents.split("\n\n").nth(0).unwrap()
        .split_whitespace().skip(1).map(|v| v.parse().unwrap()).collect();
    let directives: Vec<&str> = contents.split("\n\n").collect::<Vec<&str>>();

    let mut almanac: Vec<Vec<(usize, usize, usize)>> = Vec::new();
    for directive in directives.iter().skip(1) {
        let set: Vec<(usize, usize, usize)> = directive.lines().skip(1)
            .map(|line| line.split_whitespace()
                 .map(|s| s.parse::<usize>().expect("Parsing directive failed! NaN"))
                 .collect::<Vec<usize>>()
                 .chunks(3)
                 .map(|chunk| (chunk[0], chunk[1], chunk[2]))
                 .collect::<Vec<(usize, usize, usize)>>()
                ).flatten().collect();

        almanac.push(set);
    }

    let mut final_dest: Vec<usize> = seeds.clone();
    for i in 0..final_dest.len() {
        for map in &almanac {
            for (dest, start, range) in map {
                let seed_range = start..&(start+range);
                if !seed_range.contains(&&final_dest[i]) {
                    continue;
                }
                let dist = seed_range.end - final_dest[i];
                let res = (dest+range) - dist;

                final_dest[i] = res;
                break;
            }
        }
    }

    Some(*final_dest.iter().min().unwrap())
}

pub fn part_2(contents: &str) -> Option<usize> {
    let seeds: Vec<_> = contents.split("\n\n").nth(0).unwrap()
        .split_whitespace().skip(1).map(|v| v.parse().unwrap())
        .collect::<Vec<usize>>()
        .chunks(2).map(|chunk| (chunk[0], chunk[1]))
        .collect::<Vec<(usize, usize)>>();
    
    let directives: Vec<&str> = contents.split("\n\n").collect::<Vec<&str>>();

    let mut almanac: Vec<Vec<(usize, usize, usize)>> = Vec::new();
    for directive in directives.iter().skip(1) {
        let set: Vec<(usize, usize, usize)> = directive.lines().skip(1)
            .map(|line| line.split_whitespace()
                 .map(|s| s.parse::<usize>().expect("Parsing directive failed! NaN"))
                 .collect::<Vec<usize>>()
                 .chunks(3)
                 .map(|chunk| (chunk[0], chunk[1], chunk[2]))
                 .collect::<Vec<(usize, usize, usize)>>()
                ).flatten().collect();

        almanac.push(set);
    }
    let result: Vec<usize> = seeds.iter().map(|seeds| {
        let mut final_dest: Vec<usize> = (seeds.0..(seeds.0+seeds.1)).collect();

        for i in 0..final_dest.len() {
            for map in &almanac {
                for (dest, start, range) in map {
                    let seed_range = start..&(start+range);
                    if !seed_range.contains(&&final_dest[i]) {
                        continue;
                    }
                    let dist = seed_range.end - final_dest[i];
                    let res = (dest+range) - dist;

                    final_dest[i] = res;
                    break;
                }
            }
        }
        *final_dest.iter().min().unwrap()
    }).collect();
    
    Some(*result.iter().min().expect("Minimum value not found"))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn part_1_works() {
        let input: String = read_file("./src/test.dat").unwrap();

        let result = part_1(&input).unwrap();
        assert_eq!(result, 35);
    }

    #[test]
    fn part_2_works() {
        let input: String = read_file("./src/test.dat").unwrap();

        let result = part_2(&input).unwrap();
        assert_eq!(result, 46);
    }
}
