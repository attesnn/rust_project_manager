use std::io;

mod assign;

fn main() {
    let p_numb = assign::assign_project_number();
    let p_name = assign_project_manager();
    build_project(p_numb, p_name);
}

fn assign_project_manager () -> String {
    loop {
        let mut proj_man = String::new();
    
           println!("Please input your Project Manager's first name.");
    
            io::stdin()
                .read_line(&mut proj_man)
                .expect("failed to read line");
            
            
            println!("You project managers name is: {proj_man}");
            return proj_man;
        };
}

fn build_project (p_id: u32, p_name: String) {
    struct Project {
        p_id: u32,
        p_name: String,
    }
    let project = Project {
        p_id,
        p_name    
    };
    println!("Project created with: ID - {}, PM name - {}", project.p_id, project.p_name);
     
}
