

fn main(){

    let integers = vec![1, 2, 3, 4, 5, 6];
    let filtred_integers = get_odd_vector(integers.clone());

    for i in filtred_integers.iter() {
        //println!("{}", i); 
    }

    let filtred_integers_pair = get_one_plus_and_pair_vector(integers.clone());

    for i in filtred_integers_pair.iter() {
        //println!("{}", i);
    }

    let pair_vector = get_pair_vector_with_reference_params(&integers);
    for i in pair_vector.iter() {
        println!("{}", i);
    }

    println!("integers");

    for i in integers.iter() {
        println!("{}", i);
    }

}

fn get_pair_vector_with_reference_params(vector: &Vec<i32>) -> Vec<i32> {
    vector.into_iter().filter_map(|x|{get_pair(x)}).collect()
}

fn get_pair(x: &i32) -> Option<i32> {

    if x % 2 == 0{
        Some(*x)
    }else{
        None
    }

}

fn get_one_plus_and_pair_vector(integers: Vec<i32>) -> Vec<i32> {
    integers.into_iter().map(|i| -> i32 {i+1}).filter(|i| i%2==0).collect()

}

fn get_odd_vector(integers: Vec<i32>) -> Vec<i32> {
    integers.into_iter().filter(|i|-> bool {i%2 == 1}).collect()

}

