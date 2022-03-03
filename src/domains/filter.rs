use std::fmt::Display;

#[derive(Copy, Clone, PartialEq)]
pub enum Filter {
    All,
    Active,
    Completed,
}

impl Display for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Filter::All => "All",
            Filter::Active => "Active",
            Filter::Completed => "Completed",
        })
    }
}

impl Filter {
    pub fn all() -> Vec<Filter> {
        vec![Filter::All, Filter::Active, Filter::Completed]
    }
}
