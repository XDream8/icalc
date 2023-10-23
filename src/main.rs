use seahorse::{App, Context};
use std::{env, process::exit};

struct LinearEquationParser;

fn main() {
    // cli
    let args: Vec<String> = env::args().collect();

    let app = App::new(env!("CARGO_PKG_NAME"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage(format!("{} [flags]", env!("CARGO_PKG_NAME")))
        .action(action);

    app.run(args);
    exit(0);
}

fn action(c: &Context) {
    let expr: String = c.args.join(" ");

    if expr.contains('=') {
        dbg!(LinearEquationParser::parse(&expr));
    } else {
        match calculate(expr) {
            Ok(result) => println!("{}", result),
            Err(error) => eprintln!("Error: {}", error),
        }
    }
}

impl LinearEquationParser {
    fn parse(equation: &String) -> Option<(f64, f64)> {
        let sides: Vec<&str> = equation.split('=').collect();

        if sides.len() != 2 {
            return None;
        }

        let left_side = sides[0].trim();
        let right_side = sides[1].trim();

        let (coefficient_x, constant) = Self::parse_left_side(left_side);

        if let Ok(constant_c) = right_side.parse::<f64>() {
            Some((coefficient_x, constant_c - constant))
        } else {
            None
        }
    }

    fn parse_left_side(left_side: &str) -> (f64, f64) {
        let mut coefficient_x = 0.0;
        let mut constant = 0.0;

        let mut terms: Vec<(&str, bool)> = Vec::new();

        let mut term_start = 0;
        let mut is_addition = true;

        for (i, c) in left_side.chars().enumerate() {
            if c == '+' || c == '-' {
                if i > term_start {
                    let term = left_side[term_start..i].trim();
                    terms.push((term, is_addition));
                }
                term_start = i + 1;
                is_addition = c == '+';
            }
        }

        if term_start < left_side.len() {
            let term = left_side[term_start..].trim();
            terms.push((term, is_addition));
        }

        for (term, is_addition) in terms {
            let term = term.trim();

            if term.ends_with("*x") {
                if let Ok(coefficient) = term.trim_end_matches("*x").parse::<f64>() {
                    coefficient_x += if is_addition {
                        -coefficient
                    } else {
                        coefficient
                    }
                }
            } else if let Ok(constant_term) = term.parse::<f64>() {
                constant += if is_addition {
                    constant_term
                } else {
                    -constant_term
                }
            }
        }

        dbg!(coefficient_x, constant);
        (coefficient_x, constant)
    }
}

// fn solve_equation(equation: String) -> Result<(Some(f64), Some(f64)), String> {}

fn calculate(expr: String) -> Result<f64, String> {
    let result = exmex::eval_str::<f64>(&expr);

    match result {
        Ok(result) => Ok(result),
        Err(err) => Err(err.to_string()),
    }
}
