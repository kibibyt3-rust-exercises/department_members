use std::collections::HashMap;
use std::io;

const BAD_COMMAND: &str = "Command misunderstood! Please try again.";

fn main() {
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        let mut command = String::new();

        println!("Enter command:");

        io::stdin()
            .read_line(&mut command)
            .expect(BAD_COMMAND);

        let lowercase_command = command.as_str().to_lowercase();

        let words: Vec<&str> = lowercase_command.split_whitespace().collect();

        if words.len() <= 0 {
            println!("{BAD_COMMAND}");
            continue;
        }

        match words[0] {
            "add" => {
                if words.len() != 4 {
                    println!("{BAD_COMMAND}");
                    continue;
                }
                add_employee_to_department(words[1], words[3], &mut departments);
            }
            "list" => {
                if words.len() == 1 {
                    print_employees_by_department(&departments);
                } else if words.len() == 2 {
                    print_department(words[1], &departments);
                } else {
                    println!("{BAD_COMMAND}");
                    continue
                }
            }
            "exit" => {
                if words.len() != 1 {
                    println!("{BAD_COMMAND}");
                    continue
                }
                break
                
            }
            _ => {
                println!("{BAD_COMMAND}");
                continue;
            }
        }
    }
}

fn add_employee_to_department(
    employee: &str,
    department: &str,
    departments: &mut HashMap<String, Vec<String>>,
) {
    let employees = departments
        .entry(department.to_string())
        .or_insert(Vec::new());

    employees.push(employee.to_string());
}

fn print_department(department: &str, departments: &HashMap<String, Vec<String>>) {
    let employees = match departments.get_key_value(department) {
        Some(vector) => {vector}
        None => {
            println!("Department not found!");
            return
        }
    };

    let mut employees_copy = employees.1.clone();
    employees_copy.sort();

    println!("{}\n------------", employees.0);
    for employee in employees_copy {
        println!("  - {}", employee);    
    }
    println!();
}

fn print_employees_by_department(departments: &HashMap<String, Vec<String>>) {
    let mut department_names: Vec<&String> = departments.keys().collect();
    department_names.sort();

    for department_name in department_names {
        print_department(department_name, departments);
    }
}
