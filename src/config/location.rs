use std::fmt::Display;

#[derive(Clone)]
pub struct Location {
    parents: Vec<String>
}

impl Location {
    pub fn new(starting_location: &str) -> Self {
        let parents = vec![starting_location.to_string()];

        Self { parents }
    }

    pub fn add_parent(&self, parent: &str) -> Self {
        let mut new = self.clone();
        new.parents.push(parent.to_string());
        new
    } 

    pub fn last(&self) -> String {
        self.parents.last().unwrap().clone()
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.parents.join("."))
    }
}
