fn sort_vec(xs: &mut Vec<i32>) {
    xs.sort();
}

fn print_vec(xs: Vec<i32>) {
    for x in xs {
        println!("{}", x);
    }
}

fn main() {
    let mut xs = vec![6, 5, 3];
    sort_vec(&mut xs);    
    print_vec(xs)
}
