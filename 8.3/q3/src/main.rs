use std::io;
use std::collections::HashMap;

enum Command {
    AddBranch(String),
    ListBranchs,
    Members,
    BranchMembers(String),
    AddMember(String, String),
    Quit,
    NotFound,
}

impl Command {
    fn parse(input: &str) -> Self {
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

struct Company {
    branches: HashMap<String, Vec<String>>,
}

impl Company {
    fn new() -> Self {
        Company {
            branches: HashMap::new(),
        }
    }

    fn add_branch(&mut self, name: &str) {
        self.branches.insert(name.to_string(), Vec::new());
        self.print_branches();
    }

    fn list_branches(&self) {
        self.print_branches();
    }

    fn members(&self) {
        let all_members = self.branches
            .values()
            .flat_map(|members| members.iter().cloned())
            .collect();
        self.print_members("all", Some(&all_members));
    }

    fn branch_members(&self, branch_name: &str) {
        if let Some(members) = self.branches.get(branch_name) {
            self.print_members(branch_name, Some(members));
        } else {
            self.print_branch_is_not_found(branch_name);
        }
    }

    fn add_member(&mut self, name: &str, branch_name: &str) {
        if let Some(members) = self.branches.get_mut(branch_name) {
            members.push(name.to_string());
            self.print_branches();
        } else {
            self.print_branch_is_not_found(branch_name);
        }
    }

    fn quit(&self) {
        std::process::exit(0);
    }

    fn print_branches(&self) {
        println!("--- Branches ----");
        for (branch, members) in self.branches.iter() {
            println!("- {} ({})", branch, members.len());
        }
        println!("");
    }

    fn print_branch_is_not_found(&self, branch_name: &str) {
        println!("branch name {} is not found.", branch_name);
        println!("");
    }

    fn print_members(
        &self, 
        branch_name: &str, 
        members: Option<&Vec<String>>) {

        match members {
            Some(value) => {
                println!("--- {} members --", branch_name);
                for member in value {
                    println!("- {}", member);
                }
            },
            None => ()
        }
        println!("");
    }
}

fn main() {
    let mut company = Company::new();
    
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match Command::parse(&input) {
            Command::AddBranch(name) => company.add_branch(&name),
            Command::ListBranchs => company.list_branches(),
            Command::Members => company.members(),
            Command::BranchMembers(branch_name) => company.branch_members(&branch_name),
            Command::AddMember(name, branch_name) => company.add_member(&name, &branch_name),
            Command::Quit => company.quit(),
            Command::NotFound => println!("command not found"),
        }
    }
}
