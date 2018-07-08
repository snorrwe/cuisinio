use bson::ordered::OrderedDocument;
use bson::Bson;

use super::step::Step;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Recipe {
    pub name: String,
    pub description: Option<String>,
    pub steps: Vec<Step>,
}

impl Recipe {
    // TODO: tests
    pub fn is_valid(&self) -> bool {
        let len = self.steps.len();
        let has_invalid = self.steps.iter().enumerate().any(|(index, ref step)| {
            if let Some(ref deps) = step.dependencies {
                deps.iter().any(|&dep| {
                    let dep = dep as usize;
                    dep == index || dep >= len
                })
            } else {
                false
            }
        });
        !has_invalid
    }
}

impl From<Recipe> for OrderedDocument {
    fn from(recipe: Recipe) -> OrderedDocument {
        let steps = recipe.steps.iter().map(|s| bson!(s)).collect();
        doc! {
            "name" => recipe.name,
            "description" => match recipe.description {
                Some(d) => d,
                None => "".to_string(),
            },
            "steps" => Bson::Array(steps),
        }
    }
}

impl From<OrderedDocument> for Recipe {
    fn from(doc: OrderedDocument) -> Recipe {
        Recipe {
            name: match doc.get("name").unwrap() {
                Bson::String(s) => s.to_string(),
                _ => panic!("Expected name to be a string!"),
            },
            description: match doc.get("description").unwrap() {
                Bson::String(s) => Some(s.to_string()),
                _ => panic!("Expected description to be a string!"),
            },
            steps: match doc.get("steps").unwrap() {
                Bson::Array(array) => array.iter().map(|i| Step::from(i)).collect(),
                _ => panic!("Steps should be an array!"),
            },
        }
    }
}
