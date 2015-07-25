extern crate ansi_term;
use ansi_term::Colour::{Black, Red, Green, Yellow, Blue, Purple, Cyan, Fixed};
use ansi_term::Style;
use ansi_term::ANSIStrings;

fn main() {
    println!("Hello, world!");

    println!("This is in red: {}!", Red.paint("a red string"));

    println!("Demonstrating {} and {}!",
             Blue.bold().paint("blue bold"),
             Yellow.underline().paint("yellow underline"));

    Blue.underline().bold().paint("Blue underline bold!");

    Blue.on(Yellow).paint("Blue on yellow!");

    Red.normal().paint("yet another red string");

    Fixed(134).paint("A sort of light purple.");

    Fixed(221).on(Fixed(124)).paint("Mustard in the ketchup.");

    println!("{}", Style::default().paint("No colours here."));


    println!("Make it format compact!\n{}", 
             ANSIStrings (&[
                          Red.paint("a red string"),
                          Blue.bold().paint("blue bold"),
                          Yellow.underline().paint("yellow underline") 
             ]));

}
