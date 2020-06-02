fn saved_status(text: &str) -> Result<i64, &'static str> {
    if text.len() > 200 {
        return Err("status text is too long");
    }

    let record = saved_to_database(text)?;
    // let record = try!(saved_to_database(text)); // old

    // let record = match saved_to_database(text) {
    //     Ok(rec) => rec,
    //     Err(e) => return Err(e),
    // };

    Ok(record.id)
}

fn saved_to_database(text: &str) -> Result<StatusRecord, &'static str> {
    Err("database unavailable")
}

struct StatusRecord {
    id: i64,
    text: String,
    // created_at: std:time:Instant,
}

fn add_five(n: &str) -> Result<i32, std::num::ParseIntError> {
    let num: i32 = n.parse()?;
    Ok(num + 5)
}

fn main() {
    println!("{:?}", add_five("14"));
}
