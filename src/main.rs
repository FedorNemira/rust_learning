use std::collections::HashMap;
use rand::Rng;
use regex::Regex;
use std::io;
// use std::io::Write;



fn main() {
    // calc_middle();
    // pig_latin();
    employees_to_departments();
}


fn calc_middle() {
    let mut my_vector : Vec<u32> = (1..111).map(|_x| rand::thread_rng().gen_range(1,50)).collect();
    let sum: u32 = my_vector.iter().sum();
    let my_vector_len = my_vector.len() as u32;

    println!("My vector before sorting: {:?}", my_vector); 
    println!("My vector middle value is: {:?}", sum / my_vector_len); 

    my_vector.sort();
    let my_vector_middle = my_vector_len / 2 - 1;

    println!("My vector after sorting: {:?}", my_vector);
    println!("My vector mediana: {:?}", my_vector[my_vector_middle as usize]);

    let mut map = HashMap::new();
    
    for i in my_vector {
        let count = map.entry(i).or_insert(0);
        *count +=1;
    }

    println!("{:?}", map);

    if let Some(max) = map.values().max(){
        println!("max:{}", max);
 
    }
    let mut hash_vec: Vec<(&u32, &u32)> = map.iter().collect();

    println!("{:?}", hash_vec);
    hash_vec.sort_by(|a, b| b.1.cmp(a.1));

    println!("Maximum is {:?} - {:?} times", hash_vec[0].0, hash_vec[0].1);

}



fn pig_latin(){
    let re = Regex::new(r"^(?i)[aeiouy]").unwrap();
    let ay_add = String::from("ay");
    println!("WELCOME TO PIG LATIN TRANSLATOR");

        loop {
            let mut user_string = String::new();
            println!("Type any word or CTRL+C to exit");
            io::stdin().read_line(&mut user_string).expect("Can't read input");

            for word in user_string.split_whitespace() {
                if re.is_match(&word) {
                    print!("{}-h{} ", word.trim(), ay_add)
                }
                
                else {
                    let word_vec: Vec<char> = word.chars().collect();
                    print!("{}-{}{} ", &word[1..].trim(), word_vec[0].to_lowercase(), ay_add);
                }
            }
            println!("\n")
    }
}


fn employees_to_departments() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();
    let mut employees: Vec<String> = Vec::new();

    loop {
        let mut department = String::new();
        let mut employee = String::new();
        
        println!("\nPlease enter department to witch you want to add, or type 'list' to check list of employees");
        io::stdin().read_line(&mut department).expect("Can't read line");
        let department = department.trim();
        
        if department.to_lowercase() == "list".to_string() {
            let mut sorted: Vec<_> = company.iter().collect();
            sorted.sort_by_key(|a| a.0);
            for (key, value) in sorted.iter() {
                println!("{:?} : {:?}", key, value);
            }
        }
        
        else {
            println!("Enter employee name");
            io::stdin().read_line(&mut employee).expect("Can't read line");
            let employee = employee.trim();
        
            match company.get_mut(&*department) {
                Some(v) => {
                    v.push(String::from(employee).trim().to_string());
                    v.sort();
                },
                None => {
                    let employees = vec![String::from(employee).to_string()];
                    company.insert(department.to_string(), employees);
                }
            }
        }
    }
}



