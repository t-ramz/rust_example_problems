fn main() {
    // For an unordered list of integers, use a vector and return the mean, median, and mode of the
    // list
    // A hash map may be used for finding the mode
    let mut integer_vector: Vec<i32> = vec![-15, 10, 10, -10, 25];
    println!("While working on the set {:?}", integer_vector);
    
    integer_vector.sort();  // use provided sort method, should yield 10 as median
    let index = integer_vector.len()/2;
    let mut sum = 0;
    for value in &integer_vector {
        sum += value;
    }
    let mean = sum/integer_vector.len() as i32;
    println!("The mean value is: {}", mean);

    if let Some(median) = integer_vector.get(index) {
        println!("The median value is {}", median);
    } 

    use std::collections::HashMap;

    let mut occurance_count = HashMap::new();
    for number in &integer_vector {
        let count = occurance_count.entry(number).or_insert(0);
        *count += 1;
    }
    
    if let Some(max_value) = occurance_count.values().max() {
        for (key, value) in &occurance_count {
            if value == max_value {
                println!("The modal value is {}", key);
            }
        }
    }

    // Convert strings to pig latin. First consonant of each word is moved to the end of the word
    // and 'ay' is added. Words starting with a vowel have 'hay' added instead
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];
    let mut input_string = String::new();
    use std::io;
    println!("Please enter phrase to convert to pig latin");
    io::stdin().read_line(&mut input_string).expect("Failed to read line");
    let mut pig_latin_string = String::new();
    for word in input_string.split_whitespace() {
        for word_character in word.chars() {
            let mut needs_value = true;
            for vowel in &vowels {
                if &word_character == vowel {
                    let vowel_case_string = format!("{}hay ", word);
                    pig_latin_string.push_str(&vowel_case_string);
                    needs_value = false;
                }       
            }
            if needs_value {
                let trimmed_word = word.trim_start_matches(word_character);
                let consonant_case_string = format!("{}{}ay ", trimmed_word, word_character);
                pig_latin_string.push_str(&consonant_case_string);
            }
            break;
        }
    }
    println!("Your pig latin string is: {}", pig_latin_string);

    // Using a hash map and vectors, create a text interface to allow a user to
    // add employee names to a department in a company. For example, “Add
    // Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve
    // a list of all people in a department or all people in the company by
    // department, sorted alphabetically.
    println!("Employee management");
    let mut query_input = String::new();
    let mut dept_employee_rel = HashMap::new();
    let query_prompt = "
        Please input your query
        Valid keywords are: ADD, GET, QUIT
        Usage:
        \tADD <employee> TO <department> -- adds an employee to desired department
        \tGET -- retrieves all employees for all departments
        \tGET <department> -- retrieves all employees for desired department
        \tQUIT -- quits program
        ";
        println!("{}", query_prompt);
    loop {
        io::stdin().read_line(&mut query_input).expect("Failed to read line");
        if let Some(start_value) = query_input.to_lowercase().find("add") {
            if let Some(end_value) = query_input.to_lowercase().find(" to ") {  //required whitespace for common names containing "to"
                let name = &query_input[start_value+3..end_value].trim().to_lowercase();
                let department = &query_input[end_value+3..].trim().to_lowercase();
                let dept_list = dept_employee_rel.entry(department.to_string()).or_insert(Vec::new());
                dept_list.push(name.to_string());
            }
        }
        else if let Some(start_value) = query_input.to_lowercase().find("get") {
           let department = &query_input[start_value+3..].trim().to_lowercase();
           if let Some(list) = dept_employee_rel.get_mut(department) {
               list.sort();
               println!("Employees for department {}:", department.to_uppercase());
               for employee in list {
                   println!("\t{}", employee.to_uppercase());
               }
           }
           else {
               for (department, emp_list) in dept_employee_rel.iter_mut() {
                   emp_list.sort();
                   println!("Employees for deparment {}:", department.to_uppercase());
                   for employee in emp_list {
                       println!("\t{}", employee.to_uppercase());
                   } 
               }
           }
        }
        else if let Some('q') = query_input.to_lowercase().chars().next() {
            break;
        }
        else {
            println!("Invalid input, please try one provided here:");
            println!("{}", query_prompt);
        }
        query_input.clear();
    }

}










