mod arithmetic;
mod question;
mod unit_convert;

use question::*;

fn main() {
    let mut qs: Vec<Box<dyn Question>> = Vec::new();
    unit_convert::add_questions(&mut qs);
    arithmetic::add_questions(&mut qs);

    for (i, q) in qs.iter().enumerate() {
        // println!("{}", q.question(i + 1));
        println!("{}", q.answer(i + 1));
    }
}
