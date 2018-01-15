//!Flavor contains all the silly println! calls we've made until such time as we can pause to implement a graphics/UI lib solution for Windows deployments
use colored::*;

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
pub fn hr() {

}