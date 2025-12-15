// build.rs
use std::fs::File;
use std::io::Write;

fn main() {
    let mut output = String::from("_version: 2\n\n");

    // Basic numbers 0-19
    let ones = [
        "zero",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
    ];

    for (i, word) in ones.iter().enumerate() {
        output.push_str(&format!("numbers.{}:\n  en: {}\n", i, word));
    }

    output.push('\n');

    // Tens
    let tens = [
        "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];

    for (i, word) in tens.iter().enumerate() {
        output.push_str(&format!("numbers.{}:\n  en: {}\n", (i + 2) * 10, word));
    }

    output.push('\n');

    // Combined numbers (21-99)
    for (tens_i, tens_word) in tens.iter().enumerate() {
        #[allow(clippy::needless_range_loop)]
        for ones_i in 1..10 {
            output.push_str(&format!(
                "numbers.combined.{}_{}:\n  en: {} {}\n",
                (tens_i + 2) * 10,
                ones_i,
                tens_word,
                ones[ones_i]
            ));
        }
    }

    output.push('\n');

    // Large numbers and special cases
    let special_cases = [
        ("numbers.hundred", "hundred"),
        ("numbers.thousand", "thousand"),
        ("numbers.million", "million"),
        ("numbers.billion", "billion"),
        ("numbers.negative", "negative"),
        ("numbers.one_eighth", "one eighth"),
        ("numbers.one_quarter", "one quarter"),
        ("numbers.one_half", "one half"),
        ("numbers.three_quarter", "three quarter"),
    ];

    for (key, value) in special_cases.iter() {
        output.push_str(&format!("{}:\n  en: {}\n", key, value));
    }

    output.push('\n');

    // Units
    let units = [
        ("units.singular.foot", "foot"),
        ("units.singular.kilometer", "kilometer"),
        ("units.singular.meter", "meter"),
        ("units.singular.mile", "mile"),
        ("units.plural.feet", "feet"),
        ("units.plural.kilometers", "kilometers"),
        ("units.plural.meters", "meters"),
        ("units.plural.miles", "miles"),
    ];

    for (key, value) in units.iter() {
        output.push_str(&format!("{}:\n  en: {}\n", key, value));
    }

    // Write to file
    let mut file = File::create("../locales/numbers.yml").expect("Failed to create file");
    file.write_all(output.as_bytes())
        .expect("Failed to write to file");
}
