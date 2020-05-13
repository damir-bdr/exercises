fn main(){
    let x = 5;
    let r: &i32;
    {
        let y = 2;
        r = &y; // borrowed value does not live long enough
    }
    println!("{}", r);
}
