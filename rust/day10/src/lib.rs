use std::{error::Error, collections::VecDeque};

pub fn read_file(filename: &str) -> Result<String, Box<dyn Error>> {
    let contents: String = std::fs::read_to_string(filename).unwrap();
    Ok(contents)
}
pub fn part_1(contents: &str) -> Option<usize> {
    let map: Vec<Vec<char>> = contents.lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    
    let mut start_pos: Option<(usize, usize)> = None;
    if let Some(row) = map.iter().position(|line| line.contains(&'S')) {
        if let Some(col) = map[row].iter().position(|&c| c == 'S') {
            start_pos = Some((row, col));
        }
    }
    let start_pos = match start_pos {
        Some(pos)   => pos,
        None        => panic!("Must have been the wind...")
    };

    let mut stack: VecDeque<(usize, usize)> = VecDeque::new();
    let mut path: Vec<(usize, usize)> = Vec::new();
    let mut visited: Vec<Vec<bool>> = vec![vec![false; map[0].len()]; map.len()];
    visited[start_pos.0][start_pos.1] = true;

    stack.push_back(start_pos);

    while let Some(current) = stack.pop_front() {
        path.push(start_pos);
        let (y, x) = (current.0 as isize, current.1 as isize);
        let edges = match map[current.0][current.1] {
            'S' => vec![(y-1,x), (y+1,x), (y,x-1), (y,x+1)],
            '-' => vec![(y,x-1), (y,x+1)], 
            '|' => vec![(y-1,x), (y+1,x)], 
            'L' => vec![(y-1,x), (y,x+1)], 
            'J' => vec![(y-1,x), (y,x-1)], 
            '7' => vec![(y+1,x), (y,x-1)], 
            'F' => vec![(y+1,x), (y,x+1)], 
            '.' => vec![], 
            _ => panic!("Must have been the wind..."),
        };
        let edges: Vec<(usize, usize)> = edges.iter()
            .filter_map(|&(y, x)| if y>= 0 && x>=0 { Some((y as usize, x as usize))} else {None})
            .collect();

        let dir_vecs: Vec<(isize, isize)> = edges.iter()
            .map(|&(iy, ix)| ((iy as isize)-y, (ix as isize)-x))
            .collect();

        let dir_vecs: Vec<bool> = dir_vecs.iter()
            .map(|&(iy, ix)| {
                let (y, x): (isize, isize) = (current.0 as isize, current.1 as isize);
                let tile: char = map[(y+iy) as usize][(x+ix) as usize];
                match (iy, ix) {
                    (1, 0) => ['|','L','J'].contains(&tile),
                    (-1,0) => ['|','7','F'].contains(&tile),
                    (0, 1) => ['-','J','7'].contains(&tile),
                    (0,-1) => ['-','L','F'].contains(&tile),
                    _ => panic!("Must have been the wind..."),
                }
            }).collect();

        let valid_edges: Vec<(usize, usize)> = edges.iter().zip(dir_vecs.iter())
            .filter_map(|(&c, &m)| if m { Some(c) } else { None })
            .collect();
        
        for next in valid_edges {
            if !visited[next.0][next.1] {
                visited[next.0][next.1] = true;
                stack.push_back(next);
            }
        }
    }

    Some(path.len() / 2)
}

pub fn part_2(contents: &str) -> Option<usize> {
    let map: Vec<Vec<char>> = contents.lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    
    let mut start_pos: Option<(usize, usize)> = None;
    if let Some(row) = map.iter().position(|line| line.contains(&'S')) {
        if let Some(col) = map[row].iter().position(|&c| c == 'S') {
            start_pos = Some((row, col));
        }
    }
    let start_pos = match start_pos {
        Some(pos)   => pos,
        None        => panic!("Must have been the wind...")
    };

    let mut stack: VecDeque<(usize, usize)> = VecDeque::new();
    let mut path: Vec<(usize, usize)> = Vec::new();
    let mut visited: Vec<Vec<bool>> = vec![vec![false; map[0].len()]; map.len()];
    visited[start_pos.0][start_pos.1] = true;

    stack.push_back(start_pos);

    while let Some(current) = stack.pop_back() {
        path.push(current);
        let (y, x) = (current.0 as isize, current.1 as isize);
        let edges = match map[current.0][current.1] {
            'S' => vec![(y-1,x), (y+1,x), (y,x-1), (y,x+1)],
            '-' => vec![(y,x-1), (y,x+1)], 
            '|' => vec![(y-1,x), (y+1,x)], 
            'L' => vec![(y-1,x), (y,x+1)], 
            'J' => vec![(y-1,x), (y,x-1)], 
            '7' => vec![(y+1,x), (y,x-1)], 
            'F' => vec![(y+1,x), (y,x+1)], 
            '.' => vec![], 
            _ => panic!("Must have been the wind..."),
        };
        let edges: Vec<(usize, usize)> = edges.iter()
            .filter_map(|&(y, x)| if y>= 0 && x>=0 { Some((y as usize, x as usize))} else {None})
            .collect();

        // Validate edges/positions
        let dir_vecs: Vec<(isize, isize)> = edges.iter()
            .map(|&(iy, ix)| ((iy as isize)-y, (ix as isize)-x))
            .collect();

        let dir_vecs: Vec<bool> = dir_vecs.iter()
            .map(|&(iy, ix)| {
                let (y, x): (isize, isize) = (current.0 as isize, current.1 as isize);
                let tile: char = map[(y+iy) as usize][(x+ix) as usize];
                match (iy, ix) {
                    (1, 0) => ['|','L','J'].contains(&tile),
                    (-1,0) => ['|','7','F'].contains(&tile),
                    (0, 1) => ['-','J','7'].contains(&tile),
                    (0,-1) => ['-','L','F'].contains(&tile),
                    _ => panic!("Must have been the wind..."),
                }
            }).collect();

        let valid_edges: Vec<(usize, usize)> = edges.iter().zip(dir_vecs.iter())
            .filter_map(|(&c, &m)| if m { Some(c) } else { None })
            .collect();
        
        for next in valid_edges {
            if !visited[next.0][next.1] {
                visited[next.0][next.1] = true;
                stack.push_back(next);
            }
        }
    }
    fn shoelace_formula(vertices: &Vec<(usize, usize)>) -> f32 {
        let mut sum: isize = 0;

        for i in 0..vertices.len()-1 {
            sum += (vertices[i].1 * vertices[i+1].0) as isize
                 - (vertices[i+1].1 * vertices[i].0) as isize;
        }
        sum += (vertices[vertices.len()-1].1 * vertices[0].0) as isize
             - (vertices[0].1 * vertices[vertices.len()-1].0) as isize;

        (sum as f32).abs() / 2.0
    }
    
    // Pick's Theorem
    let area = shoelace_formula(&path);
    Some((area - path.len() as f32 / 2.0  + 1.0) as usize)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn part_1_works() {
        let input: String = read_file("./src/test-1.dat").unwrap();

        let result = part_1(&input).unwrap();
        assert_eq!(result, 8);
    }

    #[test]
    fn part_2_works() {
        let input: String = read_file("./src/test-2.dat").unwrap();

        let result = part_2(&input).unwrap();
        assert_eq!(result, 10);
    }
}
