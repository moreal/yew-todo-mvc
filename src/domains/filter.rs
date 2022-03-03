#[derive(Copy, Clone, PartialEq)]
pub enum Filter {
    All,
    Active,
    Completed,
}

impl Filter {
    pub fn to_string(self) -> String {
        match self {
            Filter::All => "All",
            Filter::Active => "Active",
            Filter::Completed => "Completed",
        }
        .to_string()
    }
}
