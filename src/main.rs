use seahorse::{App, Context};
use std::{env, process::exit};

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

    match calculate(&expr) {
        Ok(result) => println!("{}", result),
        Err(error) => eprintln!("Error: {}", error),
    }
}

fn calculate(expr: &str) -> Result<f64, String> {
    if expr.contains('=') {
        todo!("equation solver");
        // let parts: Vec<_> = parsed.split("=").collect();

        // let result: Result<f64, String> = Ok(parts[1].parse().unwrap());

        // match result {
        //     Ok(result) => Ok(result),
        //     Err(_) => Err(String::from("Error")),
        // }
    } else {
        let result = exmex::eval_str::<f64>(expr);

        match result {
            Ok(result) => Ok(result),
            Err(err) => Err(err.to_string()),
        }
    }
}
