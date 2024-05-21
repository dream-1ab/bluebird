// use std::env::{self, args};

// /**
//  * @author مۇختەرجان مەخمۇت
//  * @email ug-project@outlook.com
//  * @create date 2024-05-18 23:27:38
//  * @modify date 2024-05-18 23:27:38
//  * @desc [description]
// */

// pub fn example() {
//     let a_whole_number = env::args().skip(1).next().unwrap().parse::<u64>().unwrap();
//     println!("{}", a_whole_number);
//     let mut sum = 0u64;
//     for a in 0..20_000u64 {
//         for b in 0..20_000u64 {
//             let number = {
//                 // match a_whole_number - a + b % 2 {
//                 //     0 => Numbers::Zero,
//                 //     1 => Numbers::One,
//                 //     2 => Numbers::Two,
//                 //     3 => Numbers::Three,
//                 //     4 => Numbers::Four,
//                 //     5 => Numbers::Five,
//                 //     6 => Numbers::Six,
//                 //     7 => Numbers::Seven,
//                 //     8 => Numbers::Eight,
//                 //     9 => Numbers::Nine,
//                 //     _ => {
//                 //         Numbers::Zero
//                 //     }
//                 // }
                
//                 match a_whole_number - a + b % 2 {
//                     0 => 0,
//                     1 => 1,
//                     2 => 2,
//                     3 => 3,
//                     4 => 4,
//                     5 => 5,
//                     6 => 6,
//                     7 => 7,
//                     8 => 8,
//                     9 => 9,
//                     _ => {
//                         0
//                     }
//                 }
//             };
//             // sum += process(number, a + b);
//             sum += process_with_index(number, a + b);
//         }
//     }
//     println!("{}", sum);
// }

// enum Numbers {
//     Zero,
//     One,
//     Two,
//     Three,
//     Four,
//     Five,
//     Six,
//     Seven,
//     Eight,
//     Nine
// }

// fn process(number: Numbers, value: u64) -> u64 {
//     match number {
//         Numbers::Zero => value + 0,
//         Numbers::One => value + 1,
//         Numbers::Two => value + 2,
//         Numbers::Three => value + 3,
//         Numbers::Four =>  value + 4,
//         Numbers::Five => value + 5,
//         Numbers::Six => value + 6,
//         Numbers::Seven => value + 7,
//         Numbers::Eight => value+ 8,
//         Numbers::Nine => value + 9,
//     }
// }

// fn add_0(value: u64) -> u64 {
//     value + 0
// }
// fn add_1(value: u64) -> u64 {
//     value + 1
// }
// fn add_2(value: u64) -> u64 {
//     value + 2
// }
// fn add_3(value: u64) -> u64 {
//     value + 3
// }
// fn add_4(value: u64) -> u64 {
//     value + 4
// }
// fn add_5(value: u64) -> u64 {
//     value + 5
// }
// fn add_6(value: u64) -> u64 {
//     value + 6
// }
// fn add_7(value: u64) -> u64 {
//     value + 7
// }
// fn add_8(value: u64) -> u64 {
//     value + 8
// }
// fn add_9(value: u64) -> u64 {
//     value + 9
// }

// const ADDERS: [fn(u64) -> u64; 10] = [
//     add_0,
//     add_1,
//     add_2,
//     add_3,
//     add_4,
//     add_5,
//     add_6,
//     add_7,
//     add_8,
//     add_9,
// ];

// fn process_with_index(numbers: u8, value: u64) -> u64 {
//     let adder = &ADDERS[numbers as usize];
//     adder(value)
// }

// fn process_with_if_else(numbers: u8, value: u64) -> u64 {
//     if numbers == 0 {
//         return value + 0;
//     }
//     if numbers == 1 {
//         return value + 1;
//     }
//     if numbers == 2 {
//         return value + 2;
//     }
//     if numbers == 3 {
//         return value + 3;
//     }
//     if numbers == 4 {
//         return value + 4;
//     }
//     if numbers == 5 {
//         return value + 5;
//     }
//     if numbers == 6 {
//         return value + 6;
//     }
//     if numbers == 7 {
//         return value + 7;
//     }
//     if numbers == 8 {
//         return value + 8;
//     }
//     if numbers == 9 {
//         return value + 9;
//     }
//     0
// }

use bluebird_vm_proc_macro::{hello_my_macro, my_embed_source_code};

hello_my_macro!(20);

pub fn example() {
    my_embed_source_code!("my_plugin.rs");
    let me = Person {
        name: "Hello".into(),
        age: 26
    };

    me.say_hello();
}

