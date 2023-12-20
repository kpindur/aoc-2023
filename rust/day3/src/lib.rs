use std::{error::Error, collections::HashSet};

pub fn read_file(filename: &str) -> Result<String, Box<dyn Error>> {
    let contents: String = std::fs::read_to_string(filename).unwrap();
    Ok(contents)
}

fn get_neighbors(pos: (isize, isize)) -> Vec<(Option<usize>, Option<usize>)> {
    let radius: isize = 1;
    let mut result = Vec::new();

    for y in (pos.0 - radius)..=(pos.0 + radius) {
        for x in (pos.1 - radius)..=(pos.1 + radius) {
            if (y, x) != pos {
                result.push((y.try_into().ok(), x.try_into().ok()));
            }
        }
    }
    result
}

pub fn part_1(contents: &str) -> Option<usize> {
    let grid: Vec<Vec<char>> = contents.lines().map(|line| line.chars().collect::<Vec<char>>()).collect();
    let mut locs: HashSet<(usize, usize)> = HashSet::new();
    
    for (y, row) in grid.iter().enumerate() {
        for (x, sym) in row.iter().enumerate() {
            if sym.is_digit(10) || *sym == '.' {
                continue;
            }
            
            let neighbors = get_neighbors((y as isize, x as isize));
            for neighbor in neighbors {
                if neighbor.0.is_some() && neighbor.1.is_some()
                    && grid[neighbor.0.unwrap()][neighbor.1.unwrap()].is_digit(10){
                    let (ny, mut nx) = (neighbor.0.unwrap(), neighbor.1.unwrap());
                    while nx > 0 && grid[ny][nx-1].is_digit(10) {
                        nx = nx - 1;
                    }
                    locs.insert((ny, nx));
                }
            }
        }
    }

    let result = locs.iter().map(|loc| {
        let (y, mut x): (usize, usize) = (loc.0, loc.1);
        let mut value: usize = 0;
        while x < grid[y].len() && grid[y][x].is_digit(10) {
            value = value * 10 + grid[y][x].to_digit(10).unwrap() as usize;
            x += 1;
        }
        value
    });

    Some(result.sum())
}

pub fn part_2(contents: &str) -> Option<usize> {
    let grid: Vec<Vec<char>> = contents.lines().map(|line| line.chars().collect::<Vec<char>>()).collect();
    let mut locs: Vec<HashSet<(usize, usize)>> = Vec::new();
    
    for (y, row) in grid.iter().enumerate() {
        for (x, sym) in row.iter().enumerate() {
            if sym.is_digit(10) || *sym == '.' {
                continue;
            }
            
            let neighbors = get_neighbors((y as isize, x as isize));
            let mut test: HashSet<(usize, usize)> = HashSet::new();
            for neighbor in neighbors {
                if neighbor.0.is_some() && neighbor.1.is_some()
                    && grid[neighbor.0.unwrap()][neighbor.1.unwrap()].is_digit(10){
                    let (ny, mut nx) = (neighbor.0.unwrap(), neighbor.1.unwrap());
                    while nx > 0 && grid[ny][nx-1].is_digit(10) {
                        nx = nx - 1;
                    }
                    test.insert((ny, nx));
                }
            }
            locs.push(test);
        }
    }

    let locs = locs.iter().filter_map(|loc| if loc.len()==2 { Some(loc) } else { None });

    let result = locs.map(|locs| {
        let mut result: Vec<usize> = Vec::new();
        for loc in locs {
            let (y, mut x): (usize, usize) = (loc.0, loc.1);
            let mut value: usize = 0;
            while x < grid[y].len() && grid[y][x].is_digit(10) {
                value = value * 10 + grid[y][x].to_digit(10).unwrap() as usize;
                x += 1;
            }
            result.push(value);
        }
        result[0] * result[1]
    });

    Some(result.sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn part_1_works() {
        let input: String = read_file("./src/test.dat").unwrap();

        let result = part_1(&input).unwrap();
        assert_eq!(result, 4361);
    }

    #[test]
    fn part_2_works() {
        let input: String = read_file("./src/test.dat").unwrap();

        let result = part_2(&input).unwrap();
        assert_eq!(result, 467835);
    }
}
