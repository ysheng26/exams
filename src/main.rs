mod question;
mod unit_convert;

use question::*;

fn main() {
    let mut qs: Vec<Box<dyn Question>> = Vec::new();
    unit_convert::add_questions(&mut qs);

    for q in qs {
        println!("{}", q.question());
        println!("{}", q.answer());
    }
}
