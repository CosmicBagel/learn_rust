use std::io::Write;



fn main() {
    calcs();
    pig_latin();
    employee_app();
}

fn employee_app() {
    use std::collections::HashMap;
    use std::io::stdin;
    use std::io::stdout;
    use std::collections::hash_map::Entry;

    // not using parallel vectors to store the vector of employees and department names
    // this would require tricky sorting. Instead we'll just use the hashmap and sort by the keys
    // when needed

    // text interface to allow user to add employee name
    // to a department in a company
    // eg "Add Sally to Engineering" or "Add Amir to Sales"
    // user can retrieve a list of all people in a department sorted alphabetically
    // user can retrieve a list of all people in the company sorted alphabetically

    let menu_str =
        "1) Add department\n\
        2) Add employee\n\
        3) List employees\n\
        4) Exit\n";

    // add department
        // What?
    // add user
        // check if there are any departments
        // Who?
        // Which dept?
        // 1) dept 1
        // 2) dept 2
        // ...
    // list users in department or company
        // 0) company
        // 1) dept 1
        // ...
    // exit

    let mut dept_employees = HashMap::new();
    let emps =
        vec![String::from("Alex"), String::from( "Rachel")];
    let dept = String::from("Finance");
    dept_employees.insert(dept, emps);

    loop {
        println!("{}", menu_str);
        let mut input_str = String::new();
        stdin()
            .read_line(&mut input_str)
            .expect("Failed to read stdin");

        let selection: u32 = match input_str.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("Please enter a number: {}", e);
                continue
            },
        };

        println!("Selection: {}", selection);

        match selection {
            1 => add_department(&mut dept_employees),
            2 => add_employee(&mut dept_employees),
            3 => list_employees(&mut dept_employees ),
            4 => break,
            _ => continue,
        };
    }

    fn add_department(map: &mut HashMap<String, Vec<String>>) {
        let employee_list = Vec::new();

        loop {
            let mut entered_name = String::new();
            print!("Enter a department name: ");
            stdout().flush().expect("Failed to flush stdout");
            stdin().read_line(&mut entered_name)
                .expect("Failed to read stdin");

            match map.entry(String::from(entered_name.trim())) {
                Entry::Occupied(_) => {
                    println!("Department name already exists");
                    continue;
                },
                Entry::Vacant(v) =>  {
                    v.insert(employee_list);
                    break;
                },
            };
        }
    }

    fn add_employee(map: &mut HashMap<String, Vec<String>>) {
        // print department list
        println!("Departments:");
        let mut k: Vec<String> = map.keys().cloned().collect();
        k.sort();

        // determine the department key
        let dept_key = loop {
            for (ind, key) in k.iter().enumerate() {
                println!("{}) {}", ind, *key);
            }

            // get input for which department
            let mut dept_selection = String::new();
            stdin().read_line(&mut dept_selection)
                .expect("Failed to read stdin");

            match dept_selection.trim().parse::<usize>() {
                Ok(num) => {
                    if num < k.len() {
                        break &k[num];
                    } else {
                        println!("Invalid number");
                    }
                },
                Err(e) => {
                    println!("Please enter a number: {}", e);
                    continue;
                }
            };
        };

        println!("{} selected!", dept_key);

        let mut emp_name= String::new();
        print!("Enter a name: ");
        stdout().flush().expect("Failed to flush stdout");
        stdin().read_line(&mut emp_name)
            .expect("Failed to read from stdin");

        match map.entry(String::from(dept_key)) {
            Entry::Occupied(o) => {
                let v = o.into_mut();
                v.push(String::from(emp_name.trim()));
            }
            Entry::Vacant(_) => {
                println!("how...");
            }
        }
    }

    fn list_employees(map: &mut HashMap<String, Vec<String>>)  {
        let mut emp_list = Vec::new();

        let mut k: Vec<String> = map.keys().cloned().collect();
        k.sort();
        k.push(String::from("Whole company"));

        // determine the department key
        let dept_key = loop {
            for (ind, key) in k.iter().enumerate() {
                println!("{}) {}", ind, *key);
            }

            // get input for which department
            let mut dept_selection = String::new();
            stdin().read_line(&mut dept_selection)
                .expect("Failed to read stdin");

            match dept_selection.trim().parse::<usize>() {
                Ok(num) => {
                    if num < k.len() {
                        break &k[num];
                    } else {
                        println!("Invalid number");
                    }
                },
                Err(e) => {
                    println!("Please enter a number: {}", e);
                    continue;
                }
            };
        };

        if dept_key == "Whole company" {
            for dept_emps in map.values() {
                for emp in dept_emps.iter() {
                    emp_list.push(emp.clone());
                }
            }
        } else {
            match map.entry(String::from(dept_key)) {
                Entry::Occupied(o) => {
                    for emp in o.get().iter() {
                        emp_list.push(emp.clone());
                    }
                }
                Entry::Vacant(_) => {}
            }
        }

        emp_list.sort();
        for emp in emp_list.iter() {
            println!("{}", emp);
        }
    }
}

fn pig_latin() {
    let sentence = "This is a test sentence";
    let vowels = ['a','e','i','o','u'];

    let mut new_sentence = String::new();
    for word in sentence.split_whitespace() {
        let first_char = word.chars().next().unwrap();
        if vowels.contains(&first_char) {
            //apple-hay
            new_sentence.push_str(word);
            new_sentence.push_str("-hay ");
        } else {
            //irst-fay
            new_sentence.push_str(&word[1..]);
            new_sentence.push_str("-");
            new_sentence.push(first_char);
            new_sentence.push_str("ay ");
        }
    }

    println!("The pig latin sentence: {}", new_sentence);
}

fn calcs() {
    let x = [1,2,3,4,5,6,7,8,9,10,11,12,13,1];

    // calc mean, median, mode (most frequent)

    let x = x.to_vec();

    let calc_mean = || {
        let mut mean = 0.0;
        for &i in x.iter() {
            mean += i as f64;
        }

        mean /= x.len() as f64;

        println!("Mean: {}", mean);
    };
    calc_mean();

    let calc_median = || {
        let mut x = x.clone();
        x.sort_unstable();

        let mut median:f64;

        if x.len() % 2 == 1 {
            // odd number of elements, pick the middlest element
            let ind = x.len() / 2;
            median = x[ind] as f64;
        } else {
            let ind = x.len() / 2;
            median = (x[ind] + x[ind - 1]) as f64;
            median /= 2.0;
        }

        println!("Median: {}", median);
    };
    calc_median();

    let calc_mode = || {
        use std::collections::HashMap;
        let mut map = HashMap::new();

        for &i in x.iter() {
            *map.entry(i).or_insert(0) += 1;
        }

        let mut mode = 0;
        let mut mode_count = i32::MIN;

        for (&k, &v) in map.iter() {
            if mode_count < v {
                mode = k;
                mode_count = v;
            }
        }

        println!("Mode: {}", mode);
    };
    calc_mode();
}