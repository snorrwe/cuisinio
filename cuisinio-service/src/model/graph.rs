use std::collections::BTreeMap;
use std::iter::FromIterator;
use std::slice::Iter;

macro_rules! graph {
    [$($x: expr), *] => {
        Graph::new(vec![$($x)*])
    }
}

type Connection = (usize, usize);
type EdgeMap = BTreeMap<usize, Vec<usize>>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Graph {
    #[serde(skip)]
    edges: EdgeMap,
    #[serde(rename = "edges")]
    raw_edges: Vec<Connection>, // TODO: remove this
}

impl Graph {
    pub fn new(connections: Vec<Connection>) -> Graph {
        let mut result = Graph {
            edges: EdgeMap::new(),
            raw_edges: connections,
        };
        result.init();
        result
    }

    pub fn init(&mut self) {
        if self.edges.len() != self.raw_edges.len() {
            let mut edges = EdgeMap::new();
            for (ref k, ref v) in self.raw_edges.iter() {
                if edges.contains_key(k) {
                    edges.get_mut(k).unwrap().push(*v);
                } else {
                    edges.insert(*k, vec![*v]);
                }
            }
            self.edges = edges;
        }
    }

    pub fn iter(&self) -> Iter<Connection> {
        self.raw_edges.iter()
    }

    pub fn has_circle(&self) -> bool {
        self.edges.iter().any(|(vertex, _v)| {
            let result = self.visits(vertex, vertex);
            println!("{}, {}, {:?}", result, vertex, self);
            result
        })
    }

    pub fn visits(&self, target: &usize, start: &usize) -> bool {
        self.edges.contains_key(start)
            && self.edges[start]
                .iter()
                .any(|vertex| *vertex == *target || self.visits(target, vertex))
    }
}

impl FromIterator<Connection> for Graph {
    fn from_iter<T: IntoIterator<Item = Connection>>(it: T) -> Self {
        let mut result = vec![];
        for conn in it {
            result.push(conn);
        }
        Graph::new(result)
    }
}

impl From<Vec<Connection>> for Graph {
    fn from(vec: Vec<Connection>) -> Self {
        Graph::new(vec)
    }
}

#[cfg(test)]
mod graph_tests {
    use super::*;

    #[test]
    fn test_has_circle_positive() {
        let graph = Graph::new(vec![(0, 1), (1, 2), (2, 0)]);
        let result = graph.has_circle();
        assert_eq!(result, true);
    }

    #[test]
    fn test_has_circle_positive_2() {
        let graph = Graph::new(vec![(0, 1), (1, 2), (2, 0), (1, 0)]);
        let result = graph.has_circle();
        assert_eq!(result, true);
    }

    #[test]
    fn test_has_circle_negative() {
        let graph = Graph::new(vec![(0, 1), (1, 2), (2, 3)]);
        let result = graph.has_circle();
        assert_eq!(result, false);
    }
}
