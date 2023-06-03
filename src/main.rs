fn main() {
    println!("\n\n---- expect: 2023 => MMXXIII ------------");
    println!("------------ 2023 => {} ----------------- \n\n", arabic_to_roman(2023));

    println!("\n\n---- expect: 1984 => MCMLXXXIV ------------");
    println!("------------ 1984 => {} ----------------- \n\n", arabic_to_roman(1984));

    println!("\n\n---- expect: 1944 => MCMXLIV ------------");
    println!("------------ 1944 => {} ----------------- \n\n", arabic_to_roman(1944));
}

fn arabic_to_roman(mut number: i32) -> String {
    let mut res = String::from("");

    while number > 1000 {
        res.push_str("M");
        number -= 1000;
    }

    if number >= 900 {
        res.push_str("CM");
        number -= 900;
    }

    while number > 500 {
        res.push('D');
        number -= 500
    }

    if number >= 400 {
        res.push_str("CD");
        number -= 400;
    }

    while number > 100 {
        res.push('C');
        number -= 100
    }

    if number >= 90 {
        res.push_str("XC");
        number -= 90;
    }

    while number > 50 {
        res.push('L');
        number -= 50;
    }

    if number >= 40 {
        res.push_str("XL");
        number -= 40;
    }

    while number > 10 {
        res.push('X');
        number -= 10;
    }

    if number >= 9 {
        res.push_str("IX");
        number -= 9;
    }

    while number >= 5 {
        res.push('V');
        number -= 5;
    }

    if number >= 4 {
        res.push_str("IV");
        number -= 4;
    }

    while number >= 1 {
        res.push('I');
        number -= 1;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3999() {
        assert_eq!(arabic_to_roman(3999), "MMMCMXCIX");
    }

    #[test]
    fn test_3456() {
        assert_eq!(arabic_to_roman(3456), "MMMCDLVI");
    }

    #[test]
    fn test_2421() {
        assert_eq!(arabic_to_roman(2421), "MMCDXXI");
    }

    #[test]
    fn test_155() {
        assert_eq!(arabic_to_roman(155), "CLV");
    }
    #[test]
    fn test_1984() {
        assert_eq!(arabic_to_roman(1984), "MCMLXXXIV");
    }

    #[test]
    fn test_1944() {
        assert_eq!(arabic_to_roman(1944), "MCMXLIV");
    }

    #[test]
    fn test_2023() {
        assert_eq!(arabic_to_roman(2023), "MMXXIII");
    }

    #[test]
    fn test_1() {
        assert_eq!(arabic_to_roman(1), "I");
    }

    #[test]
    fn test_2() {
        assert_eq!(arabic_to_roman(2), "II");
    }

    #[test]
    fn test_3() {
        assert_eq!(arabic_to_roman(3), "III");
    }

    #[test]
    fn test_4() {
        assert_eq!(arabic_to_roman(4), "IV");
    }

    #[test]
    fn test_5() {
        assert_eq!(arabic_to_roman(5), "V");
    }

    #[test]
    fn test_6() {
        assert_eq!(arabic_to_roman(6), "VI");
    }

    #[test]
    fn test_7() {
        assert_eq!(arabic_to_roman(7), "VII");
    }

    #[test]
    fn test_8() {
        assert_eq!(arabic_to_roman(8), "VIII");
    }

    #[test]
    fn test_9() {
        assert_eq!(arabic_to_roman(9), "IX");
    }
}
