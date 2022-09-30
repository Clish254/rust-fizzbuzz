fn main() {
    let result = fizzbuzz(5);
    println!("{:?}", result)
}

fn fizzbuzz(n: i32) -> Vec<String> {
    let mut arr: Vec<String> = Vec::new();
    for i in 1..=n {
        let is_divisible_by_three = i % 3 == 0;
        let is_divisible_by_five = i % 5 == 0;
        if is_divisible_by_five && is_divisible_by_three {
            arr.push(String::from("FizzBuzz"))
        } else if is_divisible_by_three {
            arr.push(String::from("Fizz"))
        } else if is_divisible_by_five {
            arr.push(String::from("Buzz"))
        } else {
            arr.push(i.to_string())
        }
    }
    arr
}
