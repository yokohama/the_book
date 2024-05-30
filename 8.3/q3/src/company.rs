use std::collections::HashMap;

pub struct Company {
    branches: HashMap<String, Vec<String>>,
}

impl Company {
    pub fn new() -> Self {
        Company {
            branches: HashMap::new(),
        }
    }

    pub fn add_branch(&mut self, name: &str) {
        self.branches.insert(name.to_string(), Vec::new());
        self.print_branches();
    }

    pub fn list_branches(&self) {
        self.print_branches();
    }

    pub fn members(&self) {
        let all_members = self.branches
            .values()
            .flat_map(|members| members.iter().cloned())
            .collect();
        self.print_members("all", Some(&all_members));
    }

    pub fn branch_members(&self, branch_name: &str) {
        if let Some(members) = self.branches.get(branch_name) {
            self.print_members(branch_name, Some(members));
        } else {
            self.print_branch_is_not_found(branch_name);
        }
    }

    pub fn add_member(&mut self, name: &str, branch_name: &str) {
        if let Some(members) = self.branches.get_mut(branch_name) {
            members.push(name.to_string());
            self.print_branches();
        } else {
            self.print_branch_is_not_found(branch_name);
        }
    }

    pub fn quit(&self) {
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
