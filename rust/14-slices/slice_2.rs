fn main() {
    let mut v = vec![10, 20, 30];
    v.push(40);    

    let v_slice = &v[..4];
    println!("{:?}", v_slice);
}
