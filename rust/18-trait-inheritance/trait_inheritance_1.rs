struct Spam;

trait A {
    fn a(&self);
}

trait B: A {
    fn b(&self);
}

impl B for Spam {
    fn b(&self) {
        println!("spam.b");        
    }
}

impl A for Spam {
    fn a(&self) {
        self.b(); // call B!
    }
}

fn main() {
    let spam = Spam;
    spam.a();
    spam.b();
}
