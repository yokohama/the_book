pub enum Command {
    AddBranch(String),
    ListBranchs,
    Members,
    BranchMembers(String),
    AddMember(String, String),
    Quit,
    NotFound,
}

impl Command {
    pub fn parse(input: &str) -> Self {
        let parts: Vec<&str> = input.split_whitespace().collect();
        match parts.as_slice() {
            ["add", "branch", name] => Command::AddBranch(name.to_string()),
            ["branches"] => Command::ListBranchs,
            ["add", name, "in", branch_name] => Command::AddMember(name.to_string(), branch_name.to_string()),
            ["members"] => Command::Members,
            ["members", "in", branch_name] => Command::BranchMembers(branch_name.to_string()),
            ["quit"] => Command::Quit,
            _ => Command::NotFound,
        }
    }
}
