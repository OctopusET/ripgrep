fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <man|complete-bash|complete-zsh|complete-fish|complete-powershell>", args[0]);
        std::process::exit(1);
    }

    let output = match args[1].as_str() {
        "man" => ripgrep::generate_man_page(),
        "complete-bash" => ripgrep::generate_complete_bash(),
        "complete-zsh" => ripgrep::generate_complete_zsh(),
        "complete-fish" => ripgrep::generate_complete_fish(),
        "complete-powershell" => ripgrep::generate_complete_powershell(),
        _ => {
            eprintln!("Unknown mode: {}", args[1]);
            std::process::exit(1);
        }
    };

    println!("{}", output.trim_end());
}
