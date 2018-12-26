use std::io;

fn main() {
    //This is an example of references and slices with strings
    println!("Please input your word(s)");
    let mut words = String::new();

    io::stdin().read_line(&mut words)//Take in the users phrase
        .expect("Failed to read line");

    let single_word = first_word(&mut words);//Pass the words to the funcion 
    println!("{}", single_word);

    words.clear();
}

fn first_word(phrase: &String) -> &str {//Returns a reference
    let bytes = phrase.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {//Loop through each of the bytes
        if item == b' ' {//The b is the byte operator
            return &phrase[0..i]; //If there is a space return the word
        }
    }
    &phrase[..]//else return the whole word
}