fn main() {
    let a = [11, 22, 33];    
    only_array(&a);

    let v = vec![40, 50, 60];
    only_vector(&v);

    either_array_or_vector(&a[..]); 
    either_array_or_vector(&a[2..]); 
    either_array_or_vector(&a); 

    let v_slice = &v[0..2];    
    either_array_or_vector(&v); 
    either_array_or_vector(v_slice); 
}

fn only_array(param: &[i32; 3]) {
    println!("this is an array: {:?}", param);
}

fn only_vector(param: &Vec<i32>) {
    println!("this is a vector: {:?}", param);
}

fn either_array_or_vector(param: &[i32]) {
    println!("this is a slice: {:?}", param);
}
