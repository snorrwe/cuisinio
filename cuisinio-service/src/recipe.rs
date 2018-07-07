#[derive(Serialize, Deserialize)]
pub struct Step {
    pub name: String,
    pub description: Option<String>,
    pub duration_ms: u64,
}

#[derive(Serialize, Deserialize)]
pub struct Recipe {
    pub name: String,
    pub description: Option<String>,
    pub steps: Vec<Step>,
}
