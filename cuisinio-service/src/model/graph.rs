use std::iter::FromIterator;
use std::slice::Iter;

macro_rules! graph{
    [$($x: expr), *] => {
        Graph::new(vec![$($x)*])
    }
}

type Connection = (usize, usize);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Graph {
    graph: Vec<Connection>,
}

impl Graph {
    pub fn new(graph: Vec<Connection>) -> Graph {
        Graph { graph: graph }
    }

    pub fn iter(&self) -> Iter<Connection> {
        self.graph.iter()
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
