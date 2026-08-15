use clap::Args;

#[derive(Debug, Clone)]
pub struct SearchTerm {
    pub field: Option<String>,
    pub value: String,
}

impl SearchTerm {
    pub fn parse(query: &str) -> Self {
        if let Some((field, value)) = query.split_once(":") {
            return Self {
                field: Some(field.to_owned()),
                value: value.to_owned(),
            };
        }

        Self {
            field: None,
            value: query.to_owned(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum SearchExpr {
    And(Vec<SearchTerm>),
    Or(Vec<Vec<SearchTerm>>),
}

#[derive(Args)]
pub struct SearchArgs {
    pub query: Option<Vec<String>>,
}

impl SearchArgs {
    pub fn expr(&self) -> Option<SearchExpr> {
        let query = self.query.as_ref()?;

        let groups: Vec<Vec<SearchTerm>> = query
            .split(|query| query == "OR")
            .map(|group| group.iter().map(|query| SearchTerm::parse(query)).collect())
            .filter(|group: &Vec<SearchTerm>| !group.is_empty())
            .collect();

        match groups.len() {
            0 => None,
            1 => Some(SearchExpr::And(groups.into_iter().next().unwrap())),
            _ => Some(SearchExpr::Or(groups)),
        }
    }
}
