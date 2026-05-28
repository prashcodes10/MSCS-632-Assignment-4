use std::collections::HashMap;

#[derive(Clone)]
struct Employee {
    name: String,
    preferred_shift: String,
    days_worked: u32,
    worked_days: Vec<String>,
}

fn main() {
    let days = vec![
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
        "Sunday",
    ];

    let shifts = vec!["Morning", "Afternoon", "Evening"];

    let mut employees = vec![
        Employee {
            name: "Alice".to_string(),
            preferred_shift: "Morning".to_string(),
            days_worked: 0,
            worked_days: Vec::new(),
        },
        Employee {
            name: "Bob".to_string(),
            preferred_shift: "Morning".to_string(),
            days_worked: 0,
            worked_days: Vec::new(),
        },
        Employee {
            name: "Charlie".to_string(),
            preferred_shift: "Afternoon".to_string(),
            days_worked: 0,
            worked_days: Vec::new(),
        },
        Employee {
            name: "Diana".to_string(),
            preferred_shift: "Afternoon".to_string(),
            days_worked: 0,
            worked_days: Vec::new(),
        },
        Employee {
            name: "Ethan".to_string(),
            preferred_shift: "Evening".to_string(),
            days_worked: 0,
            worked_days: Vec::new(),
        },
        Employee {
            name: "Fiona".to_string(),
            preferred_shift: "Evening".to_string(),
            days_worked: 0,
            worked_days: Vec::new(),
        },
        Employee {
            name: "George".to_string(),
            preferred_shift: "Morning".to_string(),
            days_worked: 0,
            worked_days: Vec::new(),
        },
        Employee {
            name: "Hannah".to_string(),
            preferred_shift: "Afternoon".to_string(),
            days_worked: 0,
            worked_days: Vec::new(),
        },
        Employee {
            name: "Ian".to_string(),
            preferred_shift: "Evening".to_string(),
            days_worked: 0,
            worked_days: Vec::new(),
        },
    ];

    let mut schedule: HashMap<String, HashMap<String, Vec<String>>> = HashMap::new();

    for day in &days {
        let mut shift_map: HashMap<String, Vec<String>> = HashMap::new();

        for shift in &shifts {
            shift_map.insert(shift.to_string(), Vec::new());
        }

        schedule.insert(day.to_string(), shift_map);
    }

    for day in &days {
        for shift in &shifts {
            for i in 0..employees.len() {
                let current_shift_count = schedule
                    .get(*day)
                    .unwrap()
                    .get(*shift)
                    .unwrap()
                    .len();

                if current_shift_count >= 2 {
                    break;
                }

                let already_working_today = employees[i]
                    .worked_days
                    .contains(&day.to_string());

                if employees[i].preferred_shift == *shift
                    && employees[i].days_worked < 5
                    && !already_working_today
                {
                    schedule
                        .get_mut(*day)
                        .unwrap()
                        .get_mut(*shift)
                        .unwrap()
                        .push(employees[i].name.clone());

                    employees[i].days_worked += 1;
                    employees[i].worked_days.push(day.to_string());
                }
            }

            while schedule
                .get(*day)
                .unwrap()
                .get(*shift)
                .unwrap()
                .len()
                < 2
            {
                let mut assigned = false;

                for i in 0..employees.len() {
                    let already_working_today = employees[i]
                        .worked_days
                        .contains(&day.to_string());

                    if employees[i].days_worked < 5 && !already_working_today {
                        schedule
                            .get_mut(*day)
                            .unwrap()
                            .get_mut(*shift)
                            .unwrap()
                            .push(employees[i].name.clone());

                        employees[i].days_worked += 1;
                        employees[i].worked_days.push(day.to_string());

                        assigned = true;
                        break;
                    }
                }

                if !assigned {
                    println!("Warning: Not enough employees available for {} {}", day, shift);
                    break;
                }
            }
        }
    }

    println!("FINAL WEEKLY EMPLOYEE SCHEDULE");
    println!("================================");

    for day in &days {
        println!("\n{}", day);
        println!("--------------------");

        for shift in &shifts {
            print!("{} Shift: ", shift);

            let assigned_employees = schedule
                .get(*day)
                .unwrap()
                .get(*shift)
                .unwrap();

            for i in 0..assigned_employees.len() {
                if i > 0 {
                    print!(", ");
                }

                print!("{}", assigned_employees[i]);
            }

            println!();
        }
    }

    println!("\nEMPLOYEE WORKLOAD SUMMARY");
    println!("==========================");

    for employee in &employees {
        println!("{} worked {} day(s)", employee.name, employee.days_worked);
    }
}