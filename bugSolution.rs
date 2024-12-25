fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    let index = 10;

    match numbers.get(index) {
        Some(num) => println!("The number at index {} is: {}", index, num),
        None => println!("Index out of bounds")
    }
    //Alternative solution using if let
    if let Some(num) = numbers.get(index){
        println!("The number at index {} is: {}", index, num);
    }else{
        println!("Index out of bounds");
    }
} 