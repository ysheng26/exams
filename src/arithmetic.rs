use super::question::*;
use rand::seq::SliceRandom;
use rand::Rng;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Operator::*;
        let res = match &self {
            Plus => "+",
            Minus => "-",
            Multiply => "*",
            Divide => "/",
        };
        write!(f, "{}", res)
    }
}

struct ArithmeticQuestion {
    lhs: f32,
    op: Operator,
    rhs: f32,
    answer: f32,
}

impl Question for ArithmeticQuestion {
    fn question(&self, id: usize) -> String {
        let res = format!("Question {id}: {} {} {} = ?", self.lhs, self.op, self.rhs,);
        res
    }

    fn answer(&self, id: usize) -> String {
        let res = format!(
            "Answer {id}: {} {} {} = {}",
            self.lhs, self.op, self.rhs, self.answer,
        );
        res
    }
}

impl ArithmeticQuestion {
    fn new(lhs: f32, op: Operator, rhs: f32, decimal_plates: u32) -> Self {
        let answer = match &op {
            Operator::Plus => lhs + rhs,
            Operator::Minus => lhs - rhs,
            Operator::Multiply => lhs * rhs,
            Operator::Divide => lhs / rhs,
        };
        let answer = ArithmeticQuestion::round_to_decimal_plates(answer, decimal_plates);
        ArithmeticQuestion {
            lhs,
            op,
            rhs,
            answer,
        }
    }

    fn round_to_decimal_plates(num: f32, decimal_plates: u32) -> f32 {
        let fact = i32::pow(10, decimal_plates) as f32;

        let num = (num * fact).round() / fact;
        num
    }
}

fn get_number(decimal_plates: u32) -> f32 {
    let fact = i32::pow(10, decimal_plates) as f32;

    let num: f32 = rand::thread_rng().gen();
    // this rounds to decimal plates
    let num = (num * fact).round() / fact;
    num
}

pub(crate) fn add_questions(qs: &mut Vec<Box<dyn Question>>) {
    use Operator::*;
    // let xs = vec![Plus, Minus, Multiply, Divide];
    let xs = vec![Plus, Minus, Multiply, Divide];
    for _ in 0..50 {
        let current_operator = xs.choose(&mut rand::thread_rng());
        let current_operator = current_operator.unwrap();

        let lhs = get_number(2);
        let rhs = get_number(2);
        let q = ArithmeticQuestion::new(lhs, *current_operator, rhs, 2);
        qs.push(Box::new(q));
    }
}
