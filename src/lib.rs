pub struct Config<'a> {
    input_file: &'a str,
    num_words: usize,
}

impl<'a> Config<'a> {
    pub fn build(params: &'a [String]) -> Result<Config<'a>, &'static str> {
        todo!();
    }
}

pub fn run(config: Config) -> Result<Vec<String>, &'static str> {
    todo!();
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use super::*;
}
