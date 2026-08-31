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
