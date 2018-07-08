use bson::ordered::OrderedDocument;
use bson::Bson;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Step {
    pub name: String,
    pub description: Option<String>,
    pub duration_ms: i64,
    pub dependencies: Option<Vec<i64>>,
}

impl From<Step> for OrderedDocument {
    fn from(s: Step) -> OrderedDocument {
        let dependencies = match s.dependencies {
            Some(d) => d.iter().map(|i| Bson::I64(i.clone() as i64)).collect(),
            None => vec![],
        };
        let doc = doc! {
            "name" => s.name.clone(),
            "description" => match s.description {
                Some(ref d) => d.clone(),
                None => "".to_string(),
            },
            "duration_ms" => s.duration_ms,
            "dependencies" => Bson::Array(dependencies),
        };
        doc
    }
}

impl<'a> From<&'a Step> for Bson {
    fn from(s: &'a Step) -> Bson {
        Bson::Document(OrderedDocument::from(s.clone()))
    }
}

impl<'a> From<&'a Bson> for Step {
    fn from(doc: &'a Bson) -> Step {
        match doc {
            Bson::Document(doc) => Step {
                name: match doc.get("name").unwrap() {
                    Bson::String(s) => s.to_string(),
                    _ => panic!("Expected name to be a string!"),
                },
                description: match doc.get("description").unwrap() {
                    Bson::String(s) => Some(s.to_string()),
                    _ => panic!("Expected description to be a string!"),
                },
                duration_ms: match doc.get("duration_ms").unwrap() {
                    Bson::I64(s) => s.clone(),
                    _ => panic!("Expected duration to be a number!"),
                },
                dependencies: match doc.get("dependencies").unwrap() {
                    Bson::Array(a) => {
                        let result = a
                            .iter()
                            .map(|i| match i {
                                Bson::I64(i) => i.clone(),
                                _ => panic!("Expected dependencies to be numbers!"),
                            })
                            .collect();
                        Some(result)
                    }
                    _ => panic!("Expected dependencies to be an array!"),
                },
            },
            _ => panic!("Steps must be documents!"),
        }
    }
}
