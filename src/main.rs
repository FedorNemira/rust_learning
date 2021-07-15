use std::collections::HashMap;
use rand::Rng;



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

    // for (key, value) in map{
    //     if value > 1 {
    //         println!("{}", key);
    //     }
        // println!("{:?} - {:?}", key, value);
    // }

    


}


fn pig_latin(){

    let word = String::from("car");

    println!("{:}", word);

    let aaa = word.chars();
    println!("{:?}", aaa[1]);
}