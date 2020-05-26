extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[derive(Deserialize, Debug)]
struct Person {
    name: String,
}

fn main() {
    let first = serde_json::from_str::<Person>(r#"{
        "name" : "Fallen Order"
    }"#);
    println!("first = {:?}", first);
    let first_inner = match first {
        Ok(inner) => inner,
        _ => unimplemented!(),
    };
    println!("first's name = {:?}", first_inner.name);

    // let first_inner_2 = match first {
    //     Ok(inner) => inner,     // >>> value used here after move
    //     _ => unimplemented!(),
    // };
    // println!("first_2's name = {:?}", first_inner_2.name);

    let second = serde_json::from_str::<Person>(r#"{
        "name" : "Fallen Order",
    }"#);
    println!("second = {:?}", second);

    let third = serde_json::from_str::<Person>(r#"{
        1 : 2,
    }"#);
    println!("third = {:?}", third);
}
