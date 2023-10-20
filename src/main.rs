use clap::Command;

static HELP_TEXT: &str = "Provide the text you want to re-wrap via stdin, \
either by redirection or piping. Redirection:

$ softwrap < README.md

Piping:

$ echo \"Hi\\nthere\" | softwrap

The re-wrapped text will be printed to stdout. Text lines separated by a single newline \
will be combined into a single line with a space between the original lines. Text lines separated \
by one or more blank lines will be separated by exactly one blank line.
";

fn main() {
    Command::new("softwrap")
        .about("Remove hard line wraps in a plain text file")
        .after_help(HELP_TEXT)
        .get_matches();

    let stdin = std::io::stdin();
    let mut found_blank = false;
    let mut needs_space = false;

    for line in stdin.lines() {
        let line = line.expect("Could not read line");
        let line = line.trim();
        if needs_space {
            print!(" ");
        }

        if line.is_empty() {
            found_blank = true;
            needs_space = false;
        } else if found_blank {
            print!("\n\n{line}");
            found_blank = false;
            needs_space = true;
        } else {
            print!("{line}");
            needs_space = true;
        }
    }
}
