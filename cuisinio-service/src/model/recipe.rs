use bson::ordered::OrderedDocument;
use bson::Bson;

use super::graph::Graph;
use super::step::Step;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Recipe {
    pub name: String,
    pub description: Option<String>,
    pub steps: Vec<Step>,
    pub graph: Graph,
}

impl Recipe {
    pub fn is_valid(&self) -> bool {
        let len = self.steps.len();
        let has_invalid = self.graph.iter().any(|(x, y)| *x >= len || *y >= len);
        !has_invalid || self.graph.has_circle()
    }
}

impl From<Recipe> for OrderedDocument {
    fn from(recipe: Recipe) -> OrderedDocument {
        let steps = recipe.steps.iter().map(|s| bson!(s)).collect();
        let graph = recipe
            .graph
            .iter()
            .map(|(x, y)| bson!([*x as u64, *y as u64]))
            .collect();
        doc! {
            "name" => recipe.name,
            "description" => match recipe.description {
                Some(d) => d,
                None => "".to_string(),
            },
            "steps" => Bson::Array(steps),
            "graph" => Bson::Array(graph),
        }
    }
}

impl From<OrderedDocument> for Recipe {
    fn from(doc: OrderedDocument) -> Recipe {
        let graph = doc
            .get("graph")
            .expect("Document did not contain 'graph' field!");
        let graph = match graph {
            Bson::Array(array) => Graph::new(
                array
                    .iter()
                    .map(|pair| match pair {
                        Bson::Array(pair) => {
                            let mut pair = pair[0..2].iter().map(|x| match x {
                                Bson::I64(x) => x.clone() as usize,
                                _ => panic!("Expected graph pairs to be i32"),
                            });
                            (pair.next().unwrap(), pair.next().unwrap())
                        }
                        _ => (0, 0),
                    })
                    .collect(),
            ),
            _ => graph![],
        };
        Recipe {
            name: doc
                .get("name")
                .unwrap_or(&Bson::String("MISSING_NAME".to_string()))
                .to_string(),
            description: Some(
                doc.get("description")
                    .unwrap_or(&Bson::String(String::new()))
                    .to_string(),
            ),
            steps: match doc
                .get("steps")
                .expect("Document did not contain 'steps' field!")
            {
                Bson::Array(array) => {
                    let result = array.iter().map(|i| Step::from(i)).collect();
                    result
                }
                _ => vec![],
            },
            graph: graph,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_positive_validation_empty() {
        let recipe = Recipe {
            name: "sumsum".to_string(),
            description: None,
            steps: vec![],
            graph: graph![],
        };
        assert_eq!(recipe.is_valid(), true)
    }

    #[test]
    fn test_positive_validation_small_graph() {
        let recipe = Recipe {
            name: "sumsum".to_string(),
            description: None,
            steps: vec![
                Step {
                    name: "sum1".to_string(),
                    description: None,
                    duration_ms: 1,
                },
                Step {
                    name: "sum2".to_string(),
                    description: None,
                    duration_ms: 1,
                },
            ],
            graph: graph![(0, 1)],
        };
        assert_eq!(recipe.is_valid(), true)
    }

    #[test]
    fn test_negative_validation_non_existing_index() {
        let recipe = Recipe {
            name: "sumsum".to_string(),
            description: None,
            steps: vec![
                Step {
                    name: "sum1".to_string(),
                    description: None,
                    duration_ms: 1,
                },
                Step {
                    name: "sum2".to_string(),
                    description: None,
                    duration_ms: 1,
                },
            ],
            graph: graph![(0, 2)],
        };
        assert_eq!(recipe.is_valid(), false)
    }

    #[test]
    fn test_negative_validation_non_existing_indexes() {
        let recipe = Recipe {
            name: "sumsum".to_string(),
            description: None,
            steps: vec![],
            graph: graph![(0, 1)],
        };
        assert_eq!(recipe.is_valid(), false)
    }
}
