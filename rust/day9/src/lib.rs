use std::error::Error;

pub fn read_file(filename: &str) -> Result<String, Box<dyn Error>> {
    let contents: String = std::fs::read_to_string(filename).unwrap();
    Ok(contents)
}

pub fn part_1(contents: &str) -> Option<isize> {
    let inputs: Vec<Vec<isize>> = contents.lines()
        .map(|line| line.split_whitespace().map(|word| word.parse().expect("Parsing failed! NaN"))
             .collect::<Vec<isize>>())
        .collect();

    let mut triangles: Vec<Vec<Vec<isize>>> = inputs.iter().map(|seq| vec![seq.clone()]).collect();

    for i in 0..inputs.len() {
        let mut done: bool = false;

        while !done {
            let next_seq: Vec<isize> = triangles[i].last().expect("Something went terribly wrong!")
                .windows(2)
                .map(|pair| pair[1] - pair[0])
                .collect::<Vec<isize>>();
            
            triangles[i].push(next_seq.clone());

            done = next_seq.iter().all(|&x| x == 0);
        }
    }

    Some(triangles.iter()
        .map(|set| set.iter()
             .map(|seq| *seq.last().unwrap())
             .fold(0, |acc, v| v + acc))
        .sum())
}

pub fn part_2(contents: &str) -> Option<isize> {
    let inputs: Vec<Vec<isize>> = contents.lines()
        .map(|line| line.split_whitespace().map(|word| word.parse().expect("Parsing failed! NaN"))
             .collect::<Vec<isize>>())
        .collect();

    let mut triangles: Vec<Vec<Vec<isize>>> = inputs.iter().map(|seq| vec![seq.clone()]).collect();

    for i in 0..inputs.len() {
        let mut done: bool = false;

        while !done {
            let next_seq: Vec<isize> = triangles[i].last().expect("Something went terribly wrong!")
                .windows(2)
                .map(|pair| pair[1] - pair[0])
                .collect::<Vec<isize>>();
            
            triangles[i].push(next_seq.clone());

            done = next_seq.iter().all(|&x| x == 0);
        }
    }

    Some(triangles.iter()
        .map(|set| set.iter()
             .map(|seq| *seq.first().unwrap())
             .rev()
             .fold(0, |acc, v| v - acc))
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn part_1_works() {
        let input: String = read_file("./src/test.dat").unwrap();

        let result = part_1(&input);
        assert_eq!(result.unwrap(), 114);
    }

    #[test]
    fn part_2_works() {
        let input: String = read_file("./src/test.dat").unwrap();

        let result = part_2(&input);
        assert_eq!(result.unwrap(), 2);
    }
}
