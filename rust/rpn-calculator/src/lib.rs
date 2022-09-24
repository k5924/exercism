#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack = Vec::new();
    macro_rules! apply_op {
        ($a:ident, $b:ident => $val:expr) => {{
            let $a = stack.pop()?;
            let $b = stack.pop()?;
            stack.push($val);
        }};
    }
    for input in inputs {
        match input {
            CalculatorInput::Value(val) => {
                stack.push(*val);
            }
            CalculatorInput::Add => apply_op!(a, b => a + b),
            CalculatorInput::Subtract => apply_op!(a, b => b - a),
            CalculatorInput::Multiply => apply_op!(a, b => a * b),
            CalculatorInput::Divide => apply_op!(a, b => b / a),
        }
    }
    (stack.len() == 1).then(|| stack[0])
}
