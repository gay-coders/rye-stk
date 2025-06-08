use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct TokenStream {
    name: char,
    position: (usize, usize),
}

fn tokenize_file(filename: &str) -> Vec<TokenStream> {
    let file = File::open(filename).expect("[ERR] Couldn't Open File.");
    let reader = BufReader::new(file);
    let mut tokens: Vec<TokenStream> = Vec::new();
    let mut rnum: usize = 0;

    for (lnum, line) in reader.lines().enumerate() {
        for ch in line
            .expect("[ERR] Reading Lines Failed")
            .to_lowercase()
            .chars()
        {
            if ch == '\u{20}' || ch == '\u{09}' {
                rnum += 1;
                continue;
            }
            rnum += 1;
            let curr_token = TokenStream {
                name: ch,
                position: (lnum + 1, rnum),
            };

            tokens.push(curr_token);
        }
        rnum = 0;
    }
    return tokens;
}

fn main() {
    let filename: &str = "hello.c";
    let content: Vec<TokenStream> = tokenize_file(filename);

    for token in content.iter() {
        println!(
            "\"{0}\": {1}:{2}:{3}",
            token.name, filename, token.position.0, token.position.1
        );
    }
}
