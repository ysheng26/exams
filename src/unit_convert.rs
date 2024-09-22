use std::collections::HashMap;

use super::question::*;
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

struct UnitConvertQuestion {
    id: i32,
    lhs: f32,
    lhs_unit: Unit,
    rhs: f32,
    rhs_unit: Unit,
}

impl Question for UnitConvertQuestion {
    fn question(&self) -> String {
        let res = format!(
            "Question {}: {} {} = ? {}",
            self.id, self.lhs, self.lhs_unit, self.rhs_unit,
        );
        res
    }

    fn answer(&self) -> String {
        let res = format!(
            "Answer {}: {} {} = {} {}",
            self.id, self.lhs, self.lhs_unit, self.rhs, self.rhs_unit
        );
        res
    }
}

impl UnitConvertQuestion {
    fn new(id: i32, lhs_unit: Unit) -> Self {
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

        let rhs = match &lhs_unit {
            Meter => lhs * 100.0,
            Centimetre => lhs / 100.0,
            Pound => lhs * 16.0,
            Ounce => lhs / 16.0,
            Foot => lhs * 12.0,
            Inch => lhs / 12.0,
        };

        let rhs_unit = *hashmap.get(&lhs_unit).unwrap();
        UnitConvertQuestion {
            id,
            lhs,
            lhs_unit,
            rhs,
            rhs_unit,
        }
    }
}

pub(crate) fn add_questions(qs: &mut Vec<Box<dyn Question>>) {
    use Unit::*;
    // let xs = vec![Meter, Centimetre, Pound, Ounce, Foot, Inch];
    let xs = vec![Meter, Centimetre];
    for i in 0..50 {
        let current_unit = xs.choose(&mut rand::thread_rng());
        let current_unit = current_unit.unwrap();
        let q = UnitConvertQuestion::new(i + 1, *current_unit);
        qs.push(Box::new(q));
    }
}
