//bring code from other libraries in scope of this file
extern crate clap;
use ansi_term::Colour::{Green, Red};
use ansi_term::Style;
use clap::{App, Arg};
use date_diff_lib;

///The starting point of the application
fn main() {
    //this function is different for Windows and for Linux.
    //Look at the code of this function.
    enable_ansi_support();

    //define the CLI input line parameters using the clap library
    let matches = App::new("bestia_date_diff")
        .version("1.0")
        .author("Bestia")
        .about("date diff in days")
        .arg(
            Arg::with_name("first_date")
                .value_name("first_date")
                .help("first date for date diff"),
        )
        .arg(
            Arg::with_name("second_date")
                .value_name("second_date")
                .help("second date for date diff"),
        )
        .get_matches();

    //Get the values of input line parameters as strings
    let first_date = matches.value_of("first_date").unwrap_or("");
    println!("Value for first_date: {}", Red.paint(first_date));
    let second_date = matches.value_of("second_date").unwrap_or("");
    println!("Value for second_date: {}", Green.paint(second_date));
    //convert the strings to i32 (integer)
    //here I use 'variable shadowing'. The names stay the same, but the content is different.
    //It is confusing, because, this variables are immutable, but "shadowing" allows to use
    //the same name for other variable content.
    //Immutability is only for the memory content, not for the variable name.
    //And importantly I changed the datatype here.
    //So later there is not too much confusion,
    //because Rust compiler will return an error if the wrong datatype is used.
    let first_date = first_date.parse::<i32>().unwrap();
    let second_date = second_date.parse::<i32>().unwrap();
    //I have to split this integer into parts: YYYYMMDD
    let first_year = first_date / 10000;
    let first_month = first_date % 10000 / 100;
    let first_day = first_date % 100;
    //just playing with bold
    println!(
        "parsed first_date: {}-{}-{}",
        Style::new().bold().paint(first_year.to_string()),
        first_month,
        Style::new().bold().paint(first_day.to_string())
    );

    let second_year = second_date / 10000;
    let second_month = second_date % 10000 / 100;
    let second_day = second_date % 100;

    println!(
        "parsed second_date: {}-{}-{}",
        Style::new().bold().paint(second_year.to_string()),
        second_month,
        Style::new().bold().paint(second_day.to_string())
    );
    //now I can call the library function. The code is in the lib.rs file.
    let daydiff = date_diff_lib::daydiff(
        first_year,
        first_month,
        first_day,
        second_year,
        second_month,
        second_day,
    );
    //Finally output the result
    println!(
        "Date diff in days: {}",
        Style::new().underline().paint(daydiff.to_string())
    );
}
//region: different function code for Linux and Windows
#[cfg(target_family = "windows")]
///only on windows "enable ansi support" must be called
pub fn enable_ansi_support() {
    let _enabled = ansi_term::enable_ansi_support();
}

#[cfg(target_family = "unix")]
//on Linux "enable ansi support" must not be called
pub fn enable_ansi_support() {
    //do nothing
}
//endregion
