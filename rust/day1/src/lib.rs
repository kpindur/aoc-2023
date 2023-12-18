use std::{error::Error, collections::{HashMap, VecDeque}};

pub fn read_file(filename: &str) -> Result<String, Box<dyn Error>> {
    let contents: String = std::fs::read_to_string(filename).unwrap();
    Ok(contents)
}

pub fn part_1(contents: &str) -> u32 {
    let result = contents.lines().map(|line| {
            let first_digit = line.chars().filter(|c| c.is_digit(10)).next()
                .unwrap().to_digit(10).unwrap();
            let last_digit  = line.chars().filter(|c| c.is_digit(10)).last()
                .unwrap().to_digit(10).unwrap();

            first_digit * 10 + last_digit
            }
        ).sum::<u32>();
    result
}

// Solve using Aho-Corasick algorithm

type NodeID = usize;
type WordID = usize;

struct Node {
    children:   HashMap<char, NodeID>,
    failure:    NodeID,
    output:     Vec<WordID>
}

pub struct AhoCorasick {
    nodes: Vec<Node>,
    words: Vec<String>
}

impl AhoCorasick {
    pub fn new() -> Self {
        Self {
            nodes: vec![],
            words: vec![],
        }
    }

    fn alloc(&mut self) -> NodeID {
        let id = self.nodes.len();
        let node = Node {
            children:   HashMap::new(),
            failure:    0,
            output:     vec![],
        };
        self.nodes.push(node);
        id
    }

    fn get_node(&self, id: NodeID) -> &Node {
        &self.nodes[id]
    }

    fn get_words(&self) -> &Vec<String> {
        &self.words
    }

    fn get_edges(&self, id: NodeID) -> Vec<(char, NodeID)> {
        self.nodes[id].children.iter()
            .map(|(&c, &q)| (c, q))
            .collect::<Vec<_>>()
    }

    fn get_node_mut(&mut self, id: NodeID) -> &mut Node {
        &mut self.nodes[id]
    }

    pub fn build<S: Into<String>>(&mut self, words: Vec<S>) {
        self.build_children(words);
        self.build_failure();
    }

    fn build_children<S: Into<String>>(&mut self, words: Vec<S>) {
        let root = self.alloc();
        let words: Vec<String> = words.into_iter()
            .map(|s| s.into()).collect();

        for (word_id, word) in words.iter().enumerate() {
            let mut q = root;
            for c in word.chars() {
                if let Some(&q_next) = self.get_node(q).children.get(&c) {
                    q = q_next;
                } else {
                    let q_new = self.alloc();
                    self.get_node_mut(q).children.insert(c, q_new);
                    q = q_new;
                }
            }
            self.get_node_mut(q).output.push(word_id);
        }
        self.words = words;
    }

    fn build_failure(&mut self) {
        let mut queue = VecDeque::new();
        
        for (_, &q) in self.get_node(0).children.iter() {
            queue.push_back(q);
        }

        while let Some(q1) = queue.pop_front() {
            for (c, q2) in self.get_edges(q1) {
                queue.push_back(q2);

                let mut q = q1;
                while q != 0 {
                    q = self.get_node(q).failure;
                    if let Some(&q_target) = self.get_node(q).children.get(&c) {
                        q = q_target;
                        break;
                    }
                }
                let out = &self.get_node(q).output.clone();
                let node = self.get_node_mut(q2);
                node.failure = q;
                node.output.extend(out);
            }
        }
    }

    pub fn find(&self, text: &str) -> Vec<Vec<usize>> {
        let word_len: Vec<usize> = self.words.iter().map(|s| s.chars().count()).collect();
        let mut q: NodeID = 0;
        let mut result: Vec<Vec<usize>> = self.words.iter().map(|_| vec![]).collect();

        for (i, c) in text.chars().enumerate() {
            loop {
                let node = self.get_node(q);
                if let Some(&q_goto) = node.children.get(&c) {
                    q = q_goto;
                    break;
                } 
                if q == 0 {
                    break;
                }
                q = node.failure;
            }
            for &word_id in &self.get_node(q).output {
                result[word_id].push(i + 1 - word_len[word_id]);
            }
        }
        result
    }
}

pub fn part_2(contents: &str) -> usize {
    let mut pma: AhoCorasick = AhoCorasick::new();
    let mut map: HashMap<String, usize> = HashMap::new();
    
    let patterns = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
                        "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    for (i, key) in patterns[..=8].iter().enumerate() {
        map.insert(key.to_string(), i+1);
    }
    for (i, key) in patterns[9..].iter().enumerate() {
        map.insert(key.to_string(), i+1);
    }

    pma.build(patterns.clone());

    let result: usize = contents.lines().map(|line| {
        let result = pma.find(line);
        
        let min_index: usize = result.iter().enumerate()
            .flat_map(|(index, inner_vec)|{
                inner_vec.iter().map(move |&value| (index, value))
            }).collect::<Vec<(usize, usize)>>()
            .iter().min_by_key(|&&(_, value)| value).map(|&(index, _)| index).unwrap();
        
        let max_index: usize = result.iter().enumerate()
            .flat_map(|(index, inner_vec)|{
                inner_vec.iter().map(move |&value| (index, value))
            }).collect::<Vec<(usize, usize)>>()
            .iter().max_by_key(|&&(_, value)| value).map(|&(index, _)| index).unwrap();

        let first_digit = map.get(patterns[min_index]).unwrap();
        let last_digit = map.get(patterns[max_index]).unwrap();

        first_digit * 10 + last_digit
    }).sum();

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn part_1_works() {
        let input: String = read_file("./src/test-1.dat").unwrap();

        let result = part_1(&input);
        assert!(result == 142);
    }

    #[test]
    fn part_2_works() {
        let input: String = read_file("./src/test-2.dat").unwrap();

        let result = part_2(&input);
        assert!(result == 281);
    }
}
