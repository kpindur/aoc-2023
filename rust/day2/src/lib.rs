use std::error::Error;

pub fn read_file(filename: &str) -> Result<String, Box<dyn Error>> {
    let contents: String = std::fs::read_to_string(filename).unwrap();
    Ok(contents)
}

#[derive(Debug)]
enum Cube {
    Red(usize),
    Green(usize),
    Blue(usize)
}

pub fn part_1(contents: &str) -> Option<usize> {
    let mut games: Vec<(usize, Vec<Vec<Cube>>)> = Vec::new();

    for line in contents.lines() {
        let (id, round) = line.split_once(": ").unwrap();
        let id: usize = id.split_once(" ").unwrap().1.parse().expect("Failed to parse Game ID");
        let turns: Vec<Vec<Cube>> = round.split("; ")
            .map(|turn| turn.split(", ")
                 .map(|phase| {
                     let action: Vec<&str> = phase.split(" ").collect();
                     match action[1] {
                         "red"      => Cube::Red(action[0].parse().unwrap()),
                         "green"    => Cube::Green(action[0].parse().unwrap()),
                         "blue"     => Cube::Blue(action[0].parse().unwrap()),
                         _ => panic!("Must have been the wind..."),
                     }
                 }).collect::<Vec<Cube>>()
            ).collect();

        games.push((id, turns));
    }   

    let results = games.iter().map(|(id, turns)| {
        let valid: bool = turns.iter()
            .map(|turn| turn.iter().map(|phase| {
                match phase {
                    Cube::Red(v)      => if v <= &12 { true } else { false },
                    Cube::Green(v)    => if v <= &13 { true } else { false },
                    Cube::Blue(v)     => if v <= &14 { true } else { false }
                }
            }).collect::<Vec<bool>>().iter().all(|&v| v)
        ).all(|v| v);
        if valid { *id } else { 0 }
    });

    Some(results.sum())
}

pub fn part_2(contents: &str) -> Option<usize> {
    let mut games: Vec<(usize, Vec<Vec<Cube>>)> = Vec::new();

    for line in contents.lines() {
        let (id, round) = line.split_once(": ").unwrap();
        let id: usize = id.split_once(" ").unwrap().1.parse().expect("Failed to parse Game ID");
        let turns: Vec<Vec<Cube>> = round.split("; ")
            .map(|turn| turn.split(", ")
                 .map(|phase| {
                     let action: Vec<&str> = phase.split(" ").collect();
                     match action[1] {
                         "red"      => Cube::Red(action[0].parse().unwrap()),
                         "green"    => Cube::Green(action[0].parse().unwrap()),
                         "blue"     => Cube::Blue(action[0].parse().unwrap()),
                         _ => panic!("Must have been the wind..."),
                     }
                 }).collect::<Vec<Cube>>()
            ).collect();

        games.push((id, turns));
    }   
    
    let mut results: Vec<Vec<usize>> = Vec::new();

    for i in 0..games.len() {
        let (_, turns) = &games[i];
        let mut max: Vec<usize> = vec![0, 0, 0];
        for turn in turns {
            for phase in turn {
                match phase {
                    Cube::Red(v)    => if v > &max[0] { max[0] = *v },
                    Cube::Green(v)  => if v > &max[1] { max[1] = *v },
                    Cube::Blue(v)   => if v > &max[2] { max[2] = *v }
                }
            }
        }
        results.push(max.clone());
    }
    Some(results.iter().map(|game| game.iter().product::<usize>()).sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn part_1_works() {
        let input: String = read_file("./src/test.dat").unwrap();

        let result = part_1(&input);
        assert_eq!(result.unwrap(), 8);
    }

    #[test]
    fn part_2_works() {
        let input: String = read_file("./src/test.dat").unwrap();

        let result = part_2(&input);
        assert_eq!(result.unwrap(), 2286);
    }
}
