use rust_i18n::t;

pub fn number_to_words(num: f64) -> String {
    if num < 0.125 {
        t!("numbers.one_eighth").to_string()
    } else if num < 0.375 {
        t!("numbers.one_quarter").to_string()
    } else if num < 0.625 {
        t!("numbers.one_half").to_string()
    } else if num < 0.875 {
        t!("numbers.three_quarter").to_string()
    } else if num < 1.0 {
        t!("numbers.1").to_string()
    } else {
        whole_number_to_words(num as i64)
    }
}

fn whole_number_to_words(num: i64) -> String {
    let mut result = String::new();
    let mut n = num.abs();

    if num < 0 {
        result.push_str(&t!("numbers.negative"));
        result.push(' ');
    }

    if n >= 1000 {
        let thousands = n / 1000;
        n %= 1000;
        result.push_str(&format!(
            "{} {}",
            whole_number_to_words(thousands),
            t!("numbers.thousand")
        ));
        if n > 0 {
            result.push(' ');
        }
    }

    if n >= 100 {
        let hundreds = n / 100;
        n %= 100;
        result.push_str(&format!(
            "{} {}",
            whole_number_to_words(hundreds),
            t!("numbers.hundred")
        ));
        if n > 0 {
            result.push(' ');
        }
    }

    if n > 0 {
        if n < 20 {
            result.push_str(&t!(format!("numbers.{}", n)));
        } else {
            let tens = n / 10;
            let ones = n % 10;
            result.push_str(&t!(format!("numbers.{}", tens * 10)));
            if ones > 0 {
                result.push(' ');
                result.push_str(&t!(format!("numbers.{}", ones)));
            }
        }
    }

    result.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_number_conversions() {
        let test_cases = vec![
            // Basic numbers
            0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, // Teens
            11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0, // Tens
            20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0, // Combined numbers
            21.0, 45.0, 67.0, 89.0, 99.0, // Hundreds
            100.0, 101.0, 110.0, 111.0, 199.0, 200.0, 999.0, // Thousands
            1000.0, 1001.0, 1100.0, 1111.0, 1999.0, 2000.0, 9999.0, // Negative numbers
            -1.0, -10.0, -100.0, -1000.0, // Decimals
            0.5, 0.25, 0.75, 1.5,
        ];

        let mut snapshot = String::new();
        for num in test_cases {
            let words = number_to_words(num);
            snapshot.push_str(&format!("{}: {}\n", num, words));
        }

        assert_snapshot!(snapshot);
    }

    #[test]
    fn test_edge_cases() {
        let test_cases = vec![
            f64::MIN_POSITIVE, // Smallest positive float
            f64::MAX, // FIXME: This output is nonsensical without adding millions, billions, etc.
            -0.0,     // Negative zero
            0.1,      // Small decimal
            0.49,     // Just under 0.5
            0.51,     // Just over 0.5
        ];

        let mut snapshot = String::new();
        for num in test_cases {
            let words = number_to_words(num);
            snapshot.push_str(&format!("{}: {}\n", num, words));
        }

        assert_snapshot!(snapshot);
    }
}
