use std::collections::HashMap;

fn main() {
    let text = "hello world hello";

    let mut freqs = HashMap::new();

    for word in text.split_whitespace() {
        // match freqs.get_mut(word) {
        //     Some(value) => *value +=1,
        //     None => {
        //         freqs.insert(word, 1);
        //     },
        // }
        *freqs.entry(word).or_insert(0) += 1;    
    }

    println!("Word frequencies: {:#?}", freqs);
}