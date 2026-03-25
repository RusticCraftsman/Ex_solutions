use unicode_segmentation::UnicodeSegmentation;
pub fn reverse(input: &str) -> String {
    // todo!("Write a function to reverse {input}");

    let result: String = UnicodeSegmentation::graphemes(input, true).rev().collect();
    println!("\n\n");
    println!("input is : {}", input);
    for (i, letter) in UnicodeSegmentation::graphemes(input, true)
        .rev()
        .enumerate()
    {
        let test = format!("{} letter is {}", i, letter);
        println!("{}", test);
    }
    println!("\n\n");
    println!("result: {}", result);
    // let result = input.to_lowercase().to_string();
    return result;
}
