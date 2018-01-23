//!Flavor contains all the silly println! calls we've made until such time as we can pause to implement a graphics/UI lib solution for Windows deployments
use colored::*;
use textwrap::termwidth;

pub fn welcome() {
    println!(
        "\n{}",
        "                           WELCOME TO".bright_yellow()
    );
    println!("                       |----------------------|");
    println!(
        "                       |        {}{}{}{}{}         |",
        "S".white(),
        "U".yellow(),
        "P".bright_yellow(),
        "E".red(),
        "R".bright_red()
    );
    println!(
        "                       |      {}{}{}{}{}{}{}{}        |",
        "T".cyan(),
        "R".bright_purple(),
        "O".bright_green(),
        "U".green(),
        "P".purple(),
        "E".bright_purple(),
        "R".cyan(),
        "S".bright_green()
    );
    println!("                       |----------------------|");
}
pub fn lines_prompt() {
    println!(
        "\n     \"Quite a bit of material, I think!\" \n      \"Should we keep the poem to a set number of lines?\"\n"
    );
    println!(
        "{}",
        "  |---------------------------------------------------------------------------|"
            .bright_yellow()
    );
    println!(
        "{}{}{}{}{}{}{}",
        "  |".bright_yellow(),
        "  ENTER:".clear(),
        " N".red(),
        " or ".clear(),
        "n".red(),
        " to generate lines equal to the number of total lines read".clear(),
        "  |".bright_yellow()
    );
    println!(
        "{}{}{}{}{}{}{}",
        "  |".bright_yellow(),
        "  ENTER:".clear(),
        " Y".green(),
        " or ".clear(),
        "y".green(),
        " to specify the number of lines to generate".clear(),
        "                 |".bright_yellow()
    );
    println!(
        "{}",
        "  |---------------------------------------------------------------------------|"
            .bright_yellow()
    );
}
pub fn bard_intro() {
    println!("{}", "  from the mist...".clear());
    println!("{}", "      ~~~~~".purple());
    println!("{}", "         ~~~~~~~~~".bright_blue());
    println!("{}", "           ~~~~~~~~~~~".blue());
    println!("{}", "          a shadow nears...".clear());
    println!("{}", "              ~~~~~~~~~~~~".blue());
    println!("{}", "                  ~~~~~~~".purple());
    println!("{}", "                 ~~~~".blue());
    println!(
        "{}",
        "  no, not death--the figure of a BARD appears!".clear()
    );
    println!("{}", "             ~~~~~".bright_blue());
    println!("{}", "           ~~~~~~~~~".blue());
    println!("{}", "             ~~~~~~~~~~~".bright_blue());
    println!(
        "{}",
        "        \"I fear death less, perhaps...\" you think,s\n            \"than being bored to tears!\""
    );
    println!("{}", "               ~~~~~~~~~~~~".purple());
    println!("{}", "                 ~~~~~~~".blue());
    println!("{}", "                 ~~~~".bright_blue());
    println!(
        "{}",
        "              hurry though as you might,\n               before you drain your beer"
    );
    println!("{}", "           an apprehensive patron cries--");

}

pub fn hr() {
    let width = termwidth() - 12;
    println!("  |{:-<1$}|", "-", width + 6);
}
