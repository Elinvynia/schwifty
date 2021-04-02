use crate::country::Country;

impl Country {
    pub(crate) fn custom_validation(&self, input: &str) -> bool {
        use Country::*;
        match self {
            Albania => {
                let check_digit = self.check_digits(input);
                let account_number = &input[4..=11];

                let mut total = 0;
                for (ch, w) in account_number.chars().zip([9, 7, 3, 1, 9, 7, 3, 1].iter()) {
                    let ch = ch.to_digit(10).unwrap();
                    total += ch * w;
                }

                total % 10 == check_digit
            }
            Belgium => {
                let check_digits = self.check_digits(input) as u128;
                let check_number: u128 = input[4..=13].parse().unwrap();
                check_number % 97 == check_digits
            }
            CzechRepublic => {
                let account_number = &input[14..];

                let mut total = 0;
                for (ch, w) in account_number
                    .chars()
                    .zip([6, 3, 7, 9, 10, 5, 8, 4, 2, 1].iter())
                {
                    let ch = ch.to_digit(10).unwrap();
                    total += ch * w;
                }

                if total % 11 != 0 {
                    return false;
                }

                let branch_number = &input[8..=13];
                let mut total = 0;
                for (ch, w) in branch_number.chars().zip([10, 5, 8, 4, 2, 1].iter()) {
                    let ch = ch.to_digit(10).unwrap();
                    total += ch * w;
                }

                total % 11 == 0
            }
            _ => true,
        }
    }

    pub(crate) fn check_digits(&self, input: &str) -> u32 {
        use Country::*;
        let end = input.len() - 1;
        let (start, stop) = match self {
            Albania => (12, 12),
            Belgium | Montenegro => (end - 1, end),
            _ => unreachable!(),
        };

        input[start..=stop].parse().unwrap()
    }
}
