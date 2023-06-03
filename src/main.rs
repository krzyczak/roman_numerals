pub mod numerals;

use crate::numerals::arabic_to_roman;

fn main() {
    println!("\n\n---- expect: 2023 => MMXXIII ------------");
    println!("------------ 2023 => {} ----------------- \n\n", arabic_to_roman(2023));

    println!("\n\n---- expect: 1984 => MCMLXXXIV ------------");
    println!("------------ 1984 => {} ----------------- \n\n", arabic_to_roman(1984));

    println!("\n\n---- expect: 1944 => MCMXLIV ------------");
    println!("------------ 1944 => {} ----------------- \n\n", arabic_to_roman(1944));
}
