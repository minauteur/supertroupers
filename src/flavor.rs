//!Flavor contains all the silly println! calls we've made until such time as we can pause to implement a graphics/UI lib solution for Windows deployments
// use colored::*;
use textwrap::termwidth;

pub fn welcome() {
    println!(
        "\n{}",
        "                           WELCOME TO"
    );
    println!("                       |----------------------|");
    println!(
        "                       |        {}{}{}{}{}         |",
        "S",
        "U",
        "P",
        "E",
        "R"
    );
    println!(
        "                       |      {}{}{}{}{}{}{}{}        |",
        "T",
        "R",
        "O",
        "U",
        "P",
        "E",
        "R",
        "S"
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
            
    );
    println!(
        "{}{}{}{}{}{}{}",
        "  |",
        "  ENTER:",
        " N",
        " or ",
        "n",
        " to generate lines equal to the number of total lines read",
        "  |"
    );
    println!(
        "{}{}{}{}{}{}{}",
        "  |",
        "  ENTER:",
        " Y",
        " or ",
        "y",
        " to specify the number of lines to generate",
        "                 |"
    );
    println!(
        "{}",
        "  |---------------------------------------------------------------------------|"
            
    );
}
pub fn bard_intro() {
    println!("{}", "  from the mist...");
    println!("{}", "      ~~~~~");
    println!("{}", "         ~~~~~~~~~");
    println!("{}", "           ~~~~~~~~~~~");
    println!("{}", "          a shadow nears...");
    println!("{}", "              ~~~~~~~~~~~~");
    println!("{}", "                  ~~~~~~~");
    println!("{}", "                 ~~~~");
    println!(
        "{}",
        "  no, not death--the figure of a BARD appears!"
    );
    println!("{}", "             ~~~~~");
    println!("{}", "           ~~~~~~~~~");
    println!("{}", "             ~~~~~~~~~~~");
    println!(
        "{}",
        "        \"I fear death less, perhaps...\" you think,s\n            \"than being bored to tears!\""
    );
    println!("{}", "               ~~~~~~~~~~~~");
    println!("{}", "                 ~~~~~~~");
    println!("{}", "                 ~~~~");
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
