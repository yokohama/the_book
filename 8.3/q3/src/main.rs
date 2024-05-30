use std::io;

mod company;
use company::Company;

mod command;
use command::Command;

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
