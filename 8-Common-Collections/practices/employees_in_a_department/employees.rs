/*
*
* Problem Exercise:
*
* Using a hash map and vectors, create a text interface to allow a
* user to add employee names to a department in a company. For example,
* “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user
* retrieve a list of all people in a department or all people in the company
* by department, sorted alphabetically.
*
*/

use std::collections::HashMap;
use std::io;

fn main() {
    let mut tracker: u8 = 0;
    let mut user_input = String::new();
    let mut employee: String = String::from(" ");
    let mut department: String = String::from(" ");
    let mut department_record: HashMap<String, &mut Vec<String>> = HashMap::new();

    println!("Add a user in this format: (Add Sally to Engineering): ");
    io::stdin()
        .read_line(&mut user_input)
        .expect("Unable to read file");

    for input in user_input.split_whitespace() {
        if tracker == 1 {
            employee = input.to_string();
        }
        if tracker == 3 {
            department = input.to_string();
        }
        tracker += 1;
    }

    println!("\nEmployee => {employee}, Department => {department}\n");

    // check if the department already exists
    if let Some(record) = department_record.get(&department) {
        record.push(employee);
    } else {
        let mut new_employee_list: Vec<String> = vec![employee];
        department_record.insert(department, &mut new_employee_list);
    }

    let sorted_department_record = sortRecord(department_record);

    // display employee with their deparmtnet
    for (department, employees_list) in sorted_department_record {
        println!("------------\n{department} Department\n------------- ");
        for (idx, employee) in employees_list.iter().enumerate() {
            println!("({idx}). {employee}");
        }
        println!("\n");
    }
}

fn sortRecord(record: HashMap<String, &mut Vec<String>>) -> HashMap<String, &mut Vec<String>> {
    let mut sortedRecord: HashMap<String, &mut Vec<String>> = HashMap::new();
    let mut record_keys: Vec<_> = record.keys().cloned().collect();

    record_keys.sort();
    for key in record_keys {
        let value = record.get(&key).unwrap();
        sortedRecord.insert(key, &mut value.to_vec());
    }
    sortedRecord
}
