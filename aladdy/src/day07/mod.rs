use std::io::Result;
use std::collections::HashMap;
use std::usize::MAX;

pub fn solve<L: Iterator<Item=Result<String>>>(input: &mut L) -> () {
    let mut tower = Tower::new();

    for res in input {
        let line = res.unwrap();
        let mut tokens = line.split_whitespace();

        let name = String::from(tokens.next().unwrap());
        let weight = tokens.next().unwrap();

        tower.set_weight(&name, (weight[1..weight.len()-1]).parse().unwrap());
        
        if tokens.next().is_none() {
            continue;
        }

        for tok in tokens {
            let tok = if tok.ends_with(',') {
                &tok[..tok.len()-1]
            } else {
                &tok
            };

            tower.set_parent(&String::from(tok), &name);
        }

    }
    println!("Found root {}", tower.get_root());
    println!("{}", tower.get_imbalanced());
}

#[derive(Debug)]
struct Node {
    name: String,
    weight: usize,
    summed_weight: usize,
    parent_id: usize,
}

struct Tower {
    name_to_i: HashMap<String, usize>,
    nodes: Vec<Node>,
}

impl Tower {
    fn new() -> Tower {
        Tower{
            name_to_i: HashMap::new(),
            nodes: Vec::new(),
        }
    }

    fn set_weight(&mut self, name: &String, weight: usize) {
        let index = self.get_or_create(name);
        self.nodes[index].weight = weight;
    }

    fn set_parent(&mut self, name: &String, parent: &String) {
        let nid = self.get_or_create(name);
        let pid = self.get_or_create(parent);
        self.nodes[nid].parent_id = pid;
    }

    fn get_root(&self) -> String {
        self.nodes.iter()
            .find(|ref n| n.parent_id == MAX)
            .unwrap().name.clone()
    }

    fn get_imbalanced(&mut self) -> String {
        let root_id = self.nodes.iter()
            .enumerate()
            .find(|&(_, ref n)| n.parent_id == MAX)
            .unwrap()
            .0;
        self.sum_weight(root_id);

        let lvl = self.find_imbalanced_level(root_id).unwrap();

        format!("{:?}", lvl.iter().map(|&i| &self.nodes[i]).collect::<Vec<&Node>>())
    }

    // helper functions

    fn get_or_create(&mut self, name: &String) -> usize {
        let i = self.name_to_i.entry(name.clone()).or_insert(self.nodes.len());
        if *i == self.nodes.len() {
            // make new node
            self.nodes.push(Node{
                name: name.clone(),
                weight: 0,
                summed_weight: 0,
                parent_id: MAX,
            })
        }

        *i
    }

    fn sum_weight(&mut self, id: usize) -> usize {
        if self.nodes[id].summed_weight != 0 {
            return self.nodes[id].summed_weight;
        }

        let children: Vec<usize> = self.nodes.iter()
            .enumerate()
            .filter(|&(_, ref n)| n.parent_id == id)
            .map(|(i, _)| i)
            .collect();

        let mut total_weight = self.nodes[id].weight;
        for &i in children.iter() {
            let mut w = self.nodes[i].summed_weight;
            if w == 0 {
                w = self.sum_weight(i);
            }
            
            total_weight += w;
        }

        self.nodes[id].summed_weight = total_weight;

        total_weight
    }

    fn find_imbalanced_level(&self, id: usize) -> Option<Vec<usize>> {
        let children: Vec<usize> = self.nodes.iter()
            .enumerate()
            .filter(|&(_, ref n)| n.parent_id == id)
            .map(|(i, _)| i)
            .collect();
        if children.len() == 0 {
            return None;
        }

        let w = self.nodes[children[0]].summed_weight;
        let balanced = children.iter()
            .all(|&i| self.nodes[i].summed_weight == w);
        if !balanced {
            // check deeper
            for &id in children.iter() {
                match self.find_imbalanced_level(id) {
                    // found imbalance at lower level
                    Some(lvl) => return Some(lvl),
                    None => {},
                }
            }
            // all lower levels are balanced, this level is the problem
            return Some(children);
        }
        // no problems here
        return None;
    }
}
