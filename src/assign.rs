use std::io;
    pub fn assign_project_number () -> u32 {
        loop {
            let mut proj_num = String::new();    
                println!("Please input your Project Number. It must be an integer.");
         
                io::stdin()
                    .read_line(&mut proj_num)
                    .expect("failed to read line");
                 
                let proj_num: u32 = match proj_num.trim().parse() {
                    Ok(num) => num,            
                    Err(_) => continue,
                };
                
                println!("You project number is: {proj_num}");
                return proj_num;
        };
    }
//test