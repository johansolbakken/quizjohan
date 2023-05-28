mod generator;
mod lexer;
mod parser;
mod quiz;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <input file> <output directory>", args[0]);
        std::process::exit(1);
    }
    let input_file = &args[1];
    let output_dir = &args[2];

    // Lexing
    let input_file = std::fs::read_to_string(input_file).unwrap();
    let mut lexer = lexer::Lexer::new(&input_file);
    let mut tokens: Vec<lexer::Token> = Vec::new();
    loop {
        let token = lexer.lex();
        tokens.push(token);
        if tokens[tokens.len() - 1].token_type == lexer::TokenType::EOF {
            break;
        }
    }

    // Parsing
    parser::parse(&mut tokens);

    // Generation

    generator::create_directory(output_dir);
    generator::create_style_sheet(
        &"templates/style.css".to_string(),
        &format!("{}/style.css", output_dir),
    );
    generator::basic_generate_html(
        &"templates/index.html".to_string(),
        &format!("{}/index.html", output_dir),
        &tokens,
    );
}
