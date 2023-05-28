mod generator;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <input file> <output directory>", args[0]);
        std::process::exit(1);
    }
    let input_file = &args[1];
    let output_dir = &args[2];

    println!("input file: {}", input_file);
    println!("output directory: {}", output_dir);

    generator::create_directory(output_dir);
    generator::create_style_sheet(
        &"templates/style.css".to_string(),
        &format!("{}/style.css", output_dir),
    );
    generator::create_html_file(
        &"templates/index.html".to_string(),
        &format!("{}/index.html", output_dir),
    );
}
