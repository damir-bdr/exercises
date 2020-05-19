struct S;

trait T1 {
    fn foo(&self);
}

trait T2 {
    fn foo(&self);
}

impl T1 for S {
    fn foo(&self) {
        println!("foo-1")
    }        
}

impl T2 for S {
    fn foo(&self) {
        println!("foo-2")
    }        
}

fn bar(s: &S) {
    T1::foo(s);
    T2::foo(s);

    <S as T1>::foo(s);
    <S as T2>::foo(s);
}

fn main(){
    let s = S;
    bar(&s);
}
