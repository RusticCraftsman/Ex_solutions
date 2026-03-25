pub fn reverse(input: &str) -> String {
    // todo!("Write a function to reverse {input}");
    let result: String = input.chars().rev().collect();
    // println!("\n\n");
    // println!("input is : {}", input);
    // for (i, letter) in input.chars().rev().enumerate() {
    //     let test = format!("{} letter is {}", i, letter);
    //     println!("{}", test);
    // }
    // println!("\n\n");
    // let result = input.to_lowercase().to_string();
    return result;
}
