use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::io;
use std::io::Write;

fn main() {
    // department -> [employees]
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    println!(
        "Commands:
    add <employee> to <department>
    list employees in <department>
    list employees in company"
    );

    loop {
        let input = get_input();
        process_input(&input, &mut company);
    }
}

fn get_input() -> String {
    let mut input = String::new();
    print!(">> ");
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input
}

fn process_input(input: &str, company: &mut HashMap<String, Vec<String>>) {
    let input = input.split_whitespace().collect::<Vec<_>>();

    if input.len() != 4 {
        invalid_input();
    } else if input[0] == "add" && input[2] == "to" {
        let employee = input[1];
        let department = input[3];

        add_employee_to_department(employee, department, company);
    } else if input == ["list", "employees", "in", "company"] {
        list_employees_in_company(company);
    } else if input[0..=2] == ["list", "employees", "in"] {
        let department = input[3];
        list_employees_in_department(department, company);
    } else {
        invalid_input();
    }
}

fn invalid_input() {
    println!("Invalid input");
}

fn add_employee_to_department(
    employee: &str,
    department: &str,
    company: &mut HashMap<String, Vec<String>>,
) {
    // assumption: input formatted as `add <employee> to <department>`
    let employee = employee.to_owned();
    let department = department.to_owned();

    match company.entry(department) {
        Entry::Occupied(mut entry) => {
            let department = entry.get_mut();

            if department.contains(&employee) {
                println!("Employee already exists in this department");
            } else {
                department.push(employee);
            }
        }
        Entry::Vacant(entry) => {
            entry.insert(vec![employee]);
        }
    }
}

fn list_employees_in_company(company: &mut HashMap<String, Vec<String>>) {
    let mut departments = company.keys().collect::<Vec<_>>();
    departments.sort_unstable();

    for department in departments {
        let mut employees = company.get(department).unwrap().to_vec();
        employees.sort_unstable();

        println!("Department: {}", department);

        for employee in employees {
            println!("  Employee: {}", employee);
        }
    }
}

fn list_employees_in_department(department: &str, company: &mut HashMap<String, Vec<String>>) {
    // assumption: input formatted as `list employees in <department>`
    match company.get(department) {
        Some(employees) => {
            let mut employees_sorted = employees.to_vec();
            employees_sorted.sort_unstable();

            for employee in employees_sorted {
                println!("Employee: {}", employee);
            }
        }
        None => println!("Department does not exist"),
    }
}
