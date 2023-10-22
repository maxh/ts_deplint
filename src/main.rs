use std::env;
use std::error::Error;
use std::path::Path;

use ts_deplint::{list_violations, pretty_print_violations};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <path>", args[0]);
        std::process::exit(1);
    }

    let target = Path::new(&args[1]);
    let violations = list_violations(target)?;
    pretty_print_violations(violations);
    Ok(())
}
