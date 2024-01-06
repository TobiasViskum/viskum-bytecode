use colorize::AnsiColor;

pub fn print_error(str: &str) {
    let str = vec![":", str].join(" ");

    eprintln!("{}{}", "[error]".red().bold(), str.red())
}
