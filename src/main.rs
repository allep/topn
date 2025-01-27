use std::{env, error};

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("--- TopN extractor ---");

    let args: Vec<String> = env::args().collect();
    let config = topn::Config::build(&args)?;
    let topNWords = topn::run(config)?;

    Ok(())
}
