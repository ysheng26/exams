use std::collections::HashMap;

use rand::seq::SliceRandom;
use rand::Rng;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Unit {
    Meter,
    Centimetre,

    Pound,
    Ounce,

    Foot,
    Inch,
}

impl std::fmt::Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Unit::*;
        let res = match &self {
            Meter => "Meter",
            Centimetre => "Centimetre",
            Pound => "Pound",
            Ounce => "Ounce",
            Foot => "Foot",
            Inch => "Inch",
        };
        write!(f, "{}", res)
    }
}

struct Question {
    lhs: f32,
    lhs_unit: Unit,
    rhs: f32,
    rhs_unit: Unit,
}

impl std::fmt::Display for Question {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let res = format!(
            "Question: {} {} = ? {}\nAnswer: {} {} = {} {}",
            self.lhs,
            self.lhs_unit,
            self.rhs_unit,
            self.lhs,
            self.lhs_unit,
            self.rhs,
            self.rhs_unit
        );
        write!(f, "{res}",)
    }
}

fn generate_question(lhs_unit: Unit) -> Question {
    use Unit::*;
    let hashmap = HashMap::from([
        (Meter, Centimetre),
        (Centimetre, Meter),
        (Pound, Ounce),
        (Ounce, Pound),
        (Foot, Inch),
        (Inch, Foot),
    ]);
    let lhs = rand::thread_rng().gen_range(0..100) as f32;

    // let mut rhs = 0;
    let rhs = match &lhs_unit {
        Meter => lhs * 100.0,
        Centimetre => lhs / 100.0,
        Pound => lhs * 16.0,
        Ounce => lhs / 16.0,
        Foot => lhs * 12.0,
        Inch => lhs / 12.0,
    };

    let rhs_unit = *hashmap.get(&lhs_unit).unwrap();
    Question {
        lhs,
        lhs_unit,
        rhs,
        rhs_unit,
    }
}

fn main() {
    use Unit::*;

    let xs = vec![Meter, Centimetre, Pound, Ounce, Foot, Inch];

    for _ in 0..50 {
        let current_unit = xs.choose(&mut rand::thread_rng());
        let current_unit = current_unit.unwrap();
        let q = generate_question(*current_unit);
        println!("{}", q);
    }
}
