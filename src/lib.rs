use std::collections::HashMap;

pub struct Config<'a> {
    input_file: &'a str,
    num_words: usize,
}

impl<'a> Config<'a> {
    pub fn build(params: &'a [String]) -> Result<Config<'a>, &'static str> {
        todo!();
    }
}

struct TopNComputer<'a> {
    raw_content: &'a str,
    words: HashMap<&'a str, usize>,
    max_words: usize,
}

impl<'a> TopNComputer<'a> {
    fn build(raw_content: &'a str, num_words: usize) -> Result<TopNComputer<'a>, &'static str> {
        Ok(TopNComputer {
            raw_content,
            words: HashMap::new(),
            max_words: num_words,
        })
    }

    fn compute(&mut self) {
        self.raw_content.trim().split_whitespace().for_each(|w| {
            self.words
                .entry(w)
                .and_modify(|element| *element += 1)
                .or_insert(1);
        });
    }

    fn get_top_words(&self) -> Vec<&str> {
        self.words.iter().map(|(w, c)| *w).collect()
    }
}

pub fn run(config: Config) -> Result<Vec<String>, &'static str> {
    todo!();
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use super::*;

    #[test]
    fn test_sample_content() -> Result<(), &'static str> {
        let raw_content = "\
foo bar blu
ahahaha ciao";
        let num_words = 3;

        let mut computer = TopNComputer::build(raw_content, num_words)?;
        computer.compute();

        let top_n = computer.get_top_words();
        assert_eq!(top_n.len(), 5);

        Ok(())
    }
}
