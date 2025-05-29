use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::io;
use std::io::Write;

pub fn run() {
    // department -> [employees]
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    display_usage();

    loop {
        let input = prompt();

        let input: Vec<_> = input.split_whitespace().collect();

        match input.first() {
            Some(&"h" | &"help") => display_usage(),
            Some(&"a" | &"add") => {
                if let Some(employee) = input.get(1) {
                    if let Some(department) = input.get(2) {
                        if let Err(e) =
                            add_employee_to_department(employee, department, &mut company)
                        {
                            eprintln!("{e}");
                        };
                    } else {
                        eprintln!("error: bad arguments");
                    }
                } else {
                    eprintln!("error: bad arguments");
                }
            }
            Some(&"l" | &"list") => {
                if let Some(department) = input.get(1) {
                    if let Err(e) = list_employees_in_department(department, &company) {
                        eprintln!("{e}");
                    };
                } else {
                    list_employees_in_company(&company);
                }
            }
            Some(&"q" | &"quit") => break,
            Some(_) => eprintln!("error: bad command"),
            None => (),
        }
    }
}

fn prompt() -> String {
    let mut input = String::new();

    print!(">> ");
    let _ = io::stdout().flush();

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");

    input
}

fn display_usage() {
    println!(
        "\
usage:
q, quit                        : quit
h, help                        : displays usage
a, add [employee] [department] : add [employee] to [department]
l, list [department]           : list employees in [department]
l, list                        : list all employees in company"
    );
}

fn add_employee_to_department(
    employee: &str,
    department: &str,
    company: &mut HashMap<String, Vec<String>>,
) -> Result<(), &'static str> {
    let employee = employee.to_owned();
    let department = department.to_owned();

    match company.entry(department) {
        Entry::Occupied(mut entry) => {
            let department = entry.get_mut();

            if department.contains(&employee) {
                return Err("error: employee already exists in this department");
            } else {
                department.push(employee);
            }
        }
        Entry::Vacant(entry) => {
            entry.insert(vec![employee]);
        }
    }

    Ok(())
}

fn list_employees_in_company(company: &HashMap<String, Vec<String>>) {
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

fn list_employees_in_department(
    department: &str,
    company: &HashMap<String, Vec<String>>,
) -> Result<(), &'static str> {
    match company.get(department) {
        Some(employees) => {
            let mut employees_sorted = employees.to_vec();
            employees_sorted.sort_unstable();

            for employee in employees_sorted {
                println!("Employee: {}", employee);
            }

            Ok(())
        }
        None => Err("error: department does not exist"),
    }
}
