fn main () {
    println!("Hello world"); 

    let string1 = String::from("Hello Solana");
    let string2 = String::from(", Hello Rise in web3");

    let concantenated_string = concantenate_strings(&string1, &string2);

    println!("The strings concatenated here are as: {}", concantenated_string);
}

fn concantenate_strings (str1: &str, str2: &str) -> String {
    let mut result = String::new();
    result.push_str(str1);
    result.push_str(str2);
    result
}