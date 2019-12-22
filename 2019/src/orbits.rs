use std::{
    collections::{HashMap, HashSet, VecDeque},
    iter::FromIterator,
};

#[derive(Debug, Clone, Default)]
struct Node {
    parent: Option<String>,
    children: HashSet<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Orbits {
    map: HashMap<String, Node>,
}

impl Orbits {
    pub fn num_orbits(&self, level: u32, root: &str) -> u32 {
        self.map[root]
            .children
            .iter()
            .map(|x| level + self.num_orbits(level + 1, x))
            .sum()
    }

    // do a breadth-first search
    pub fn dist(&self, src: &str, dest: &str) -> Option<u32> {
        let mut to_search = VecDeque::<(u32, &str)>::new();
        let mut visited = HashSet::<&str>::new();
        to_search.push_back((0, &src));

        while !to_search.is_empty() {
            let (dist, node) = to_search.pop_front().unwrap();
            visited.insert(node);

            if node == dest {
                return Some(dist - 2);
            }

            if let Some(p) = &self.map[node].parent {
                if !visited.contains(p.as_str()) {
                    to_search.push_back((dist + 1, p));
                }
            }

            for c in &self.map[node].children {
                if !visited.contains(c.as_str()) {
                    to_search.push_back((dist + 1, c));
                }
            }
        }

        None
    }
}

impl FromIterator<(String, String)> for Orbits {
    fn from_iter<I: IntoIterator<Item = (String, String)>>(iter: I) -> Self {
        let mut map = HashMap::<String, Node>::new();
        for (a, b) in iter {
            map.entry(a.clone()).or_default().children.insert(b.clone());
            let old = map.entry(b).or_default().parent.replace(a);
            if old.is_some() {
                panic!("an object is orbiting multiple objects");
            }
        }
        Orbits { map }
    }
}
