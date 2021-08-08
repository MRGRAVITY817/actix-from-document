use std::rc::Rc;

fn main() {
    let rc_examples = "Rc examples".to_string();
    {
        let rc_a = Rc::new(rc_examples);
        println!("Reference count of rc_a: {}", Rc::strong_count(&rc_a));
        {
            println!("--- rc_a is cloned to rc_b ---");
            let rc_b: Rc<String> = Rc::clone(&rc_a);
            println!("Reference count of rc_b: {}", Rc::strong_count(&rc_b));
            println!("Reference count of rc_a: {}", Rc::strong_count(&rc_a));

            // Two rcs are equal if their inner values are equal
            println!("Rc_a Rc_b are equal: {}", rc_a.eq(&rc_b));

            // We can use methods of a value directly
            println!("Length fo the value inside rc_a: {}", rc_a.len());
            println!("Value of rc_b: {}", rc_b);

            println!("--- rc_b is dropped out of the scope ---");
        }
        println!("Reference count of rc_a: {}", Rc::strong_count(&rc_a));
        println!("--- rc_a is dropped out of scope ---");
    }
}
