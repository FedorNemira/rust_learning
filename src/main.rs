use std::collections::HashMap;
use rand::Rng;
use regex::Regex;
use std::io;



fn main() {
    // calc_middle()
    pig_latin()
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
