use std::error::Error;

pub fn read_file(filename: &str) -> Result<String, Box<dyn Error>> {
    let contents: String = std::fs::read_to_string(filename).unwrap();
    Ok(contents)
}

pub fn part_1(contents: &str) -> Option<usize> {
    let mut lines = contents.lines();

    let times: Vec<usize> = lines.next().unwrap()
        .split_once(':').unwrap().1.split_whitespace()
        .map(|s| s.parse().expect("Parse failed! NaN")).collect();
    
    let distances: Vec<usize> = lines.next().unwrap()
        .split_once(':').unwrap().1.split_whitespace()
        .map(|s| s.parse().expect("Parse failed! NaN")).collect();

    let results = times.iter().zip(distances.iter()).map(|(time, distance)| {
        let mut score: usize = 0;
        for i in 1..*time {
            if (i * (*time-i)) > *distance {
                score += 1;
            }
        }
        score
    });
    
    Some(results.product::<usize>())
}

pub fn part_2(contents: &str) -> Option<usize> {
    let mut lines = contents.lines();

    let time: usize = lines.next().unwrap()
        .split_once(':').unwrap().1.replace(" ", "").parse().expect("Parse failed! NaN");
    
    let distance: usize = lines.next().unwrap()
        .split_once(':').unwrap().1.replace(" ", "").parse().expect("Parse failed! NaN");

    let mut score: usize = 0;
    for i in 1..time {
        if (i * (time - i)) > distance {
            score += 1;
        }
    }

    Some(score)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn part_1_works() {
        let input: String = read_file("./src/test.dat").unwrap();

        let result = part_1(&input).unwrap();
        assert_eq!(result, 288);
    }

    #[test]
    fn part_2_works() {
        let input: String = read_file("./src/test.dat").unwrap();

        let result = part_2(&input).unwrap();
        assert_eq!(result, 71503);
    }
}
