use bson::ordered::OrderedDocument;
use bson::Bson;

use super::step::Step;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Recipe {
    pub name: String,
    pub description: Option<String>,
    pub steps: Vec<Step>,
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
