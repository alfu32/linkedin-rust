mod equations;
mod exercise;

use crate::exercise::Exercise;

use crate::equations::{first_degree, second_degree};
use std::vec::Vec;

type Challenge = Exercise;

const exercises: [(u64, &str); 84] = [
    (1u64, "2.4 Primitives :: arithmetic"),
    (2u64, "2.5 Primitives :: formatting"),
    (3u64, "2.6 Primitives :: bitwise"),
    (4u64, "2.7 Primitives :: bitwise"),
    (5u64, "2.8 Primitives :: COMparison_operators"),
    (6u64, "2.9 Primitives :: characters"),
    (7u64, "2.10 Primitives :: collatz"),
    (8u64, "2.11 Primitives :: challenge_arithmetic_mean"),
    (9u64, "3.1 Compound Types :: array_data_type"),
    (10u64, "3.2 Compound Types :: multidimensional_arrays"),
    (11u64, "3.3 Compound Types :: tuples"),
    (12u64, "4.1 Functions :: function parameters"),
    (13u64, "4.2 Functions :: statements and expressions"),
    (14u64, "4.3 Functions :: functions return values"),
    (14u64, "4.4 Functions :: functions return values"),
    (15u64, "5.1 Flow Control :: conditional execution"),
    (16u64, "5.2 Flow Control :: conditional execution"),
    (17u64, "5.3 Flow Control :: conditional assignment"),
    (18u64, "5.4 Flow Control :: loops"),
    (19u64, "5.5 Flow Control :: while"),
    (20u64, "5.7 Flow Control :: nested loops"),
    (21u64, "5.8 Flow Control :: challenge min,max, avg"),
    (22u64, "6.1 Ownership :: variable scope"),
    (23u64, "6.2 Ownership :: shadowing variables"),
    (24u64, "6.3 Ownership :: stack and heap memory"),
    (25u64, "6.4 Ownership :: string data type"),
    (26u64, "6.5 Ownership :: ownership"),
    (27u64, "6.6 Ownership :: moving, cloning, copying data"),
    (28u64, "6.7 Ownership :: transfering ownership"),
    (29u64, "7.1 References :: Borrowing references"),
    (30u64, "7.2 References :: Mutable references"),
    (31u64, "7.3 References :: Dangling references"),
    (32u64, "7.4 References :: Slices"),
    (33u64, "7.5 References :: Slices as function parameters"),
    (34u64, "7.6 References :: Challenge: Trim Spaces"),
    (35u64, "8.1 Modules :: Rust std lib and prelude"),
    (36u64, "8.2 Modules :: standard Input"),
    (37u64, "8.3 Modules :: parse Strings"),
    (38u64, "8.4 Modules :: Crates"),
    (39u64, "8.5 Modules :: Challenge : higher or lower"),
    (40u64, "9.1 Input and Output :: command Line Arguments"),
    (41u64, "9.2 Input and Output :: reading from Files"),
    (42u64, "9.3 Input and Output :: writing to Files"),
    (
        43u64,
        "9.4 Input and Output :: Challenge : check the Roster",
    ),
    (44u64, "10.1 Structs :: defining Structs"),
    (45u64, "10.2 Structs :: update Syntax"),
    (46u64, "10.3 Structs :: Methods"),
    (47u64, "10.4 Structs :: associated Functions"),
    (48u64, "10.5 Structs :: tuple Structs"),
    (48u64, "10.6 Structs :: Challenge : represent Shapes"),
    (59u64, "11.1 Generics :: struct Definitions"),
    (50u64, "11.2 Generics :: method Definitions"),
    (51u64, "11.3 Generics :: function Definitions"),
    (52u64, "11.4 Generics :: Box data Type"),
    (53u64, "11.5 Generics :: Challenge : Sum Boxes"),
    (54u64, "12.1 Traits :: implement Traits"),
    (55u64, "12.2 Traits :: default Trait Implementation"),
    (56u64, "12.3 Traits :: derive Traits"),
    (57u64, "12.4 Traits :: Trait bounds"),
    (50u64, "12.5 Traits :: multiple Trait Bounds"),
    (51u64, "12.6 Traits :: Return Types with implemented Traits"),
    (
        62u64,
        "12.7 Traits :: Challenge : implement the display Trait",
    ),
    (63u64, "13.1 Lifetimes :: the borrow Checker"),
    (64u64, "13.2 Lifetimes :: Lifetime Annotation Syntax"),
    (65u64, "13.3 Lifetimes :: multiple Lifetime Annotations"),
    (66u64, "13.4 Lifetimes :: Lifetime elision Rules"),
    (67u64, "13.5 Lifetimes :: Struct Lifetime Annotations"),
    (68u64, "13.6 Lifetimes :: Static Lifetime"),
    (69u64, "14.1 Enums :: define Enums"),
    (70u64, "14.2 Enums :: Match Operator"),
    (71u64, "14.3 Enums :: Match with default Placeholder"),
    (72u64, "14.4 Enums :: Enum Methods"),
    (73u64, "14.5 Enums :: Option<T> Enum"),
    (74u64, "14.6 Enums :: matching Option<T>"),
    (75u64, "14.7 Enums :: if-let syntax"),
    (76u64, "14.8 Enums :: Challenge : represent a Location"),
    (77u64, "15.1 Error Handling :: Unrecoverable Errors"),
    (78u64, "15.2 Error Handling :: Result<T,E> enum"),
    (79u64, "15.3 Error Handling :: Matching Result<T,E>"),
    (80u64, "15.4 Error Handling :: Propagating Errors"),
    (81u64, "15.5 Error Handling :: Challenge : Handle Errors"),
    (82u64, "16.1 Collections :: Vectors"),
    (83u64, "16.2 Collections :: HashMaps"),
    (84u64, "16.3 Collections :: Challenge : Count Words"),
];
fn main() {
    for (i, exercise) in exercises.iter().enumerate() {
        let (id, name) = exercise;
        println!(r#"Exercise {{ id: {id}, name: "{name}", runnable: || {{}}, }} .run(false);"#);
    }
    Exercise {
        id: 1,
        name: "arithmetic",
        runnable: || {
            let a = 10;
            let b = 3f64;
            let c = (a as f64) / b;
            println!("{}", c);
        },
    }
    .run(false);
    Exercise {
        id: 2,
        name: "formatting",
        runnable: || {
            let a = 23444;
            let b = 3f64;
            let c = (a as f64) / b;
            println!("a:08   |{a:08}|");
            println!("a:08x   |{a:08x}|");
            println!(":8   |{:8}|", c);
            println!(":.3  |{:.3}|", c);
            println!(":8.3 |{:08.3}|", c);
            println!("c is {:.3} and again {0:.3}", c);
            println!("captured from the surrounding course {a} / {b} = {c}");
        },
    }
    .run(false);

    Exercise {
        id: 3,
        name: "bitwise",
        runnable: || {
            let mut a = 0b1111_0101u8;
            let b = 0b1111_1101u8;
            println!("value is {a}");
            println!("value is {a:08b}");

            a = !a;
            println!("value is {a:08b}");
            a = !a;
            let c = a & b;
            println!("{a:08b} &\n{b:08b} =\n{c:08b}\n");
            let c = a | b;
            println!("{a:08b} |\n{b:08b} =\n{c:08b}");
            let mut c = a ^ b;
            println!("{a:08b} ^\n{b:08b} =\n{c:08b}");
            c = a.clone();
            while c > 0 {
                println!("value is {a:08b}");
                c = c >> 1;
            }
            c = a.clone();
            while c > 0 {
                println!("value is {a:08b}");
                c = c << 1;
            }
        },
    }
    .run(false);
    Exercise {
        id: 4,
        name: "bitwise",
        runnable: || {
            let a = true;
            let b = false;
            println!("a is {a} and b is {b}");
            println!("NOT({a}) is {}", !a);
            println!("{a} AND {b} is {}", a & b);
            println!("{a} OR {b} is {}", a | b);
            println!("{a} XOR {b} is {}", a ^ b);
            println!("({a} ^ {b}) | ({a} & {b}) is {}", (a ^ b) | (a & b));
        },
    }
    .run(false);
    Exercise {
        id: 5,
        name: "COMparison_operators",
        runnable: || {},
    }
    .run(false);
    Exercise {
        id: 6,
        name: "characters",
        runnable: || {
            let letter = 'a';
            let number = '1';
            let page_number = 0x2600u32;
            println!("letter is {letter}");
            println!("number is {number}");
            for page_number in 0x1000u32..=0x2600u32 {
                for ub in 0..16usize {
                    let mut chars = ['A'; 16];
                    for lb in 0..16usize {
                        let cc = (ub as u32) * 0x10 + (lb as u32);
                        chars[lb] = char::from_u32(page_number + (cc as u32)).unwrap();
                        // println!("26{cc:02X}  {}",);
                    }
                    let s2: String = chars.iter().collect();
                    println!(
                        "ucs page {:04X}-{:04X} {s2}",
                        page_number + (ub as u32) * 0x10,
                        page_number + (ub as u32 + 1) * 0x10 - 1
                    );
                }
            }
        },
    }
    .run(false);
    Exercise {
        id: 7,
        name: "collatz",
        runnable: || {
            fn is_prime(n: u64) -> bool {
                for t in 2..(n / 2) {
                    if n % t == 0 {
                        return false;
                    }
                }
                true
            }
            fn collatz(n: u64) -> Vec<u64> {
                let mut r: Vec<u64> = Vec::new();
                r.push(n);
                if n > 4 {
                    r.extend(collatz(if n % 2 == 0 { n / 2 } else { 3 * n + 1 }));
                }
                r.clone()
            }
            for n in 5..255u64 {
                if is_prime(n) {
                    print!("{n},")
                }
            }
            println!();
            println!("collatz(10) {:?}", collatz(10));
            println!("collatz(13) {:?}", collatz(13));
            println!("collatz(39) {:?}", collatz(39));
            use std::time::Instant;
            let now = Instant::now();
            for k in 5..100_000 {
                let _co = collatz(k);
                // print!("collatz({k}) {}\r",co.len());
            }
            let elapsed = now.elapsed();
            println!("Elapsed: {:.2?}", elapsed);
            println!();
        },
    }
    .run(false);
    Challenge {
        id: 8,
        name: "challenge_arithmetic_mean",
        runnable: || {
            fn average(a: &mut [f64]) -> f64 {
                let mut sum = 0.0;
                for i in 0..a.len() {
                    sum = a[i] + sum;
                }
                sum / a.len() as f64
            }
            assert_eq!(
                average([13f64, 2.3f64, 120.0f64].to_owned().as_mut()),
                45.1f64
            );
            println!("test passed !");
            let a = 13;
            let b = 2.3f64;
            let c = 120.0f64;
            let avg = (a as f64 + b as f64 + c as f64) / 3.0f64;
            assert_eq!(avg, 45.1f64);
            println!("test passed !");
        },
    }
    .run(false);
    Exercise {
        id: 9,
        name: "array_data_type",
        runnable: || {
            let mut letters = ['A', 'B', 'C'];
            letters[0] = 'x';
            let first_letter = letters[0];
            println!("first letter is {}", first_letter);

            let numbers: [i32; 5];
            numbers = [0; 5];
            println!("last number is {}", numbers[4]);
            let index = numbers.len();
            println!("last number is {}", numbers[index - 1]);
        },
    }
    .run(false);

    Exercise {
        id: 10,
        name: "multidimensional_arrays",
        runnable: || {
            let parkin_lot = [[1, 2, 3], [4, 5, 6]];
            let number = parkin_lot[0][1];
            println!("number is {}", number);
            let garage = [[[0; 2]; 3]; 4];
            println!("garage is {:?}", garage);
        },
    }
    .run(false);
    Exercise {
        id: 11,
        name: "tuples",
        runnable: || {
            type GloveBox = (u8, f64, char, &'static str);
            let mut glove_box: GloveBox = (10, 3.14, '2', "three");
            let glove_box_2 = (10, 3.14, '2', "three");
            glove_box.0 += 3;
            let first_item = glove_box.0;
            println!("first_item is {}", first_item);
            let (items, value, initial, name) = glove_box;
            println!("items is {}", items);
            println!("value is {}", value);
            println!("initial is {}", initial);
            println!("name is {}", name);
        },
    }
    .run(false);
    Exercise {
        id: 12,
        name: "function parameters",
        runnable: || {
            say_hello();
            say_hello();
            let x = 1;
            let y = 2;
            say_the_sum(x, y);
            say_a_number(x as i32);

            fn say_hello() {
                println!("hello");
                say_a_number(13);
            }

            fn say_a_number(number: i32) {
                println!("number is {:08.3}", number);
            }

            fn say_the_sum(a: u8, b: u8) {
                println!("{a} + {b}  is {}", a + b);
            }
        },
    }
    .run(false);

    Exercise {
        id: 13,
        name: "statements and expressions",
        runnable: || {
            let x = 1;
        },
    }
    .run(false);

    Exercise {
        id: 14,
        name: "functions return values",
        runnable: || {
            let result = square(12);
            println!(" {}*{0} {}", result.0, result.1);
            println!(" {:?}", result);
            fn square(a: i32) -> (i32, i32) {
                let s = a * a;
                print!("squring {a}");
                (a, s)
            }
        },
    }
    .run(false);

    Challenge {
        id: 14,
        name: "functions return values",
        runnable: || {
            #[derive(Debug)]
            struct Temperature {
                celsius: f64,
                fahrenheit: f64,
            }
            impl Temperature {
                fn from_celsius(c: f64) -> Temperature {
                    let f = c * 1.8 + 32.0;
                    Temperature {
                        celsius: c,
                        fahrenheit: f,
                    }
                }
                fn from_fahrenheit(f: f64) -> Temperature {
                    let c = (f - 32.0) / 1.8;
                    Temperature {
                        celsius: c,
                        fahrenheit: f,
                    }
                }
            }
            let t = Temperature::from_celsius(23.0);
            println!("{:?}", t);
            assert_eq!(t.fahrenheit, 73.4);
            println!("Test passed !");
        },
    }
    .run(false);

    Exercise {
        id: 15,
        name: "5.1 conditional execution",
        runnable: || {
            let x = 4;
            if x == 3 {
                println!("x is 3");
            } else {
                println!("x is not 3");
            }
        },
    }
    .run(false);
    Exercise {
        id: 16,
        name: "5.2 conditional execution",
        runnable: || {
            let x = 3;
            let y = 5;
            if x > y {
                println!("x is greater than y");
            } else if x == y {
                println!("x is equal to y");
            } else {
                println!("x is smaller than y")
            }
        },
    }
    .run(false);

    Exercise {
        id: 17,
        name: "5.3 conditional assignment",
        runnable: || {
            let x = if true { "x is 3" } else { "x is not 3" };
            println!("{}", x);
        },
    }
    .run(false);
    Exercise {
        id: 18,
        name: "5.4 loops",
        runnable: || {
            let mut i = 0;
            let y = loop {
                println!("loop {i}");
                if i >= 10 {
                    break (i, i * i);
                }
                i += 1;
            };
            println!("y is {y:?}");
        },
    }
    .run(false);
    Exercise {
        id: 19,
        name: "5.5 while",
        runnable: || {
            let mut count = 10;

            let result = while count < 10 {
                count += 1;
                println!("count is {count}");
                // if count == 9 {
                //     break (count, count * count);
                // }
            };
            println!("result is {result:?}");

            let letters = "abcdefghijklmnopqrstuvwxyz";
            let mut chars = letters.chars();
            count = 0;
            loop {
                let ch = match chars.next() {
                    Some(ch) => ch,
                    None => '-',
                };
                if ch == '-' {
                    break;
                }
                println!("letters[{count}] is {ch}",);
                count += 1;
            }

            for (i, c) in letters.clone().split("").enumerate() {
                println!("letter {i} is {c}");
            }
        },
    }
    .run(false);

    Exercise {
        id: 20,
        name: "5.7 nested loops",
        runnable: || {
            let mut matrix = [
                [1.0, 1.0, 0.0, 20.0],
                [1.0, 0.0, 1.0, 4.5],
                [0.0, 1.0, 1.0, 12.0],
            ];
            println!("{matrix:#?}");

            for row in matrix.iter_mut() {
                for col in row.iter_mut() {
                    print!("{col:5.2}\t");
                }
                println!();
            }
            println!();

            for row in matrix.iter_mut() {
                for col in row.iter_mut() {
                    *col += 1.0;
                    print!("{col:5.2}\t");
                }
                println!();
            }
        },
    }
    .run(false);

    Challenge {
        id: 21,
        name: "challengr min,max, avg",
        runnable: || {
            let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
            let mut max: i32 = numbers[0];
            let mut min: i32 = numbers[0];
            let mut avg: f64 = 0f64;

            for &num in numbers.iter() {
                if num > max {
                    max = num;
                }
                if num < min {
                    min = num;
                }
                avg += num as f64;
            }
            avg = avg / numbers.len() as f64;
            println!("max : {max}");
            println!("min : {min}");
            println!("avg : {avg}");
        },
    }
    .run(false);

    Exercise {
        id: 22,
        name: "6.1 Ownership :: variable scope",
        runnable: || {
            let planet = "Mars";
            println!("{planet:?}");
            if true {
                let planet = "Earth";
                println!("{planet:?}");
            }
            println!("{planet}");
        },
    }
    .run(false);
    Exercise {
        id: 23,
        name: "6.2 Ownership :: shadowing variables",
        runnable: || {
            let planet = "Mars";
            println!("{planet:?}");
            let planet = "Earth";
            println!("{planet:?}");
            let mut planet = "Venus";
            println!("{planet:?}");
            planet = "Jupiter";
            println!("{planet:?}");
            planet = "Saturn";
            println!("{planet:?}");
            planet = "Uranus";
            println!("{planet:?}");
            planet = "Neptune";
            println!("{planet:?}");
            planet = "Pluto";
            println!("{planet:?}");
            planet = "Mercury";
            println!("{planet:?}");
            let mut planet = ("Venus", 2554.453, 44);
            println!("{planet:?}");
            planet = ("Venus", 2554.453, 44);
        },
    }
    .run(false);
    Exercise {
        id: 24,
        name: "6.3 Ownership :: stack and heap memory",
        runnable: || {
            let x = 13;
            function1();
            fn function1() {
                let y = 3.12;
                let z = 'a';
                function2();
                println!("{:?}", (y, z));
            }
            fn function2() {
                let y = 3.14;
                let z = 'c';
                println!("{:?}", (y, z));
            }
            println!("x = {x}");
        },
    }
    .run(false);
    Exercise {
        id: 25,
        name: "6.4 Ownership :: string data type",
        runnable: || {
            let string_literal = "hello";
            println!("{string_literal}");
            let mut message = String::from("Earth");
            println!("MESSAGE : {message}");
            message.push_str(" is home");
            println!("MESSAGE : {message}");
        },
    }
    .run(false);
    Exercise {
        id: 26,
        name: "6.5 Ownership :: ownership",
        runnable: || {},
    }
    .run(false);
    Exercise {
        id: 27,
        name: "6.6 Ownership :: moving, cloning, copying data",
        runnable: || {
            let outer_planet: String;
            {
                let mut inner_planet = String::from("Mercury");
                println!("inner_planet is {inner_planet}");
                outer_planet = inner_planet.clone();
                println!("inner_planet is {inner_planet}");
            }
            println!("outer_planet is {outer_planet}");
            let aggregates = {
                let sz = 20u64;
                let mut sum = 0u64;
                let mut sum_of_squares = 0u64;
                let mut factorial = 1u64;
                let mut average = 0u64;
                for c in 1..sz {
                    sum += c;
                    sum_of_squares += c * c;
                    factorial *= c;
                }
                average = sum / sz;
                (sum, sum_of_squares, factorial, average)
            };
            println!("aggregates is {aggregates:?}");
            // println!("inner_planet is {inner_planet}");
        },
    }
    .run(false);
    Exercise {
        id: 28,
        name: "6.7 Ownership :: transfering ownership",
        runnable: || {
            let rocket_fuel = String::from("RP1");
            println!("rocket_fuel is {rocket_fuel}");
            let rocket_fuel = process_fuel(rocket_fuel);
            println!("rocket_fuel is now {rocket_fuel}");

            fn process_fuel(mut propellant: String) -> String {
                propellant.push_str(".");
                println!("Processing propellant {propellant:?} ...");
                let new_fuel = String::from("LNG");
                new_fuel
            }
        },
    }
    .run(false);
    Exercise {
        id: 29,
        name: "7.1 References :: Borrowing references",
        runnable: || {
            let rocket_fuel = String::from("RP1");
            println!("rocket_fuel is {rocket_fuel}");
            let length = process_fuel(&rocket_fuel);
            println!("rocket_fuel is now {rocket_fuel} has length {length}");

            fn process_fuel(mut propellant: &String) -> usize {
                println!("Processing propellant {propellant:?} ...");
                let length = propellant.len();
                length
            }
        },
    }
    .run(false);
    Exercise {
        id: 30,
        name: "7.2 References :: Mutable references",
        runnable: || {
            let mut rocket_fuel = String::from("RP1");
            println!("rocket_fuel is {rocket_fuel}");
            let length = process_fuel(&mut rocket_fuel);
            println!("rocket_fuel is now {rocket_fuel} has length {length}");

            fn process_fuel(propellant: &mut String) -> usize {
                println!("Processing propellant {propellant:?} ...");
                propellant.push_str(" is highly flammable.");
                let length = propellant.len();
                length
            }
        },
    }
    .run(false);
    Exercise { id: 31, name: "7.3 References :: Dangling references", runnable: || {
        let rocket_fuel = produce_fuel();
        println!("rocket_fuel is {rocket_fuel}");

        fn produce_fuel() -> String {
            let new_fuel = String::from("RP-1");
            new_fuel
        }
        // // danging reference
        // fn produce_fuel() -> &String {
        //     let new_fuel = String::from("RP-1");
        //     &new_fuel
        // }
    }, } .run(false);
    Exercise {
        id: 32,
        name: "7.4 References :: Slices",
        runnable: || {
            let message = String::from("Greetings from Earth !");
            println!("message is {message}");
            let last_word = &message[15..15 + 5];
            println!("last_word is {last_word}");

            let planets = [1, 2, 3, 4, 5, 6, 7, 8];
            let inner_planets: &[i32] = &planets[..4];
            println!("inner_planets are {inner_planets:?} ");
        },
    }
    .run(false);
    Exercise { id: 33, name: "7.5 References :: Slices as function parameters", runnable: || {
        let message = String::from("Greetings from Earth !");
        let first_word = get_first_word(&message);
        println!("first_word is {first_word}");
        let second_word = get_first_word(&message[10..]);
        println!("second_word is {second_word}");

        fn get_first_word(s: &str) -> &str {
            let bytes = s.as_bytes();

            for (index,&item) in bytes.iter().enumerate() {
                if item == b' '{
                    return &s[..index];
                }
            }
            &s
        }
    }, } .run(false);

    Challenge { id: 34, name: "7.6 References :: Challenge: Trim Spaces", runnable: || {
        fn trim_spaces(s: &str) -> &str {
            let mut bytes = s.as_bytes().iter().enumerate();
            let reversed = s.chars().rev().collect::<String>();
            let mut rev = reversed.as_bytes().iter().enumerate();
            let mut from:usize=0;
            let l = s.len();
            let mut to : usize = s.len();
            for (i,c) in s.chars().enumerate() {
                if c != ' ' {
                    from = i;
                    break;
                }
            }
            for (i,c) in s.chars().rev().collect::<String>().chars().enumerate() {
                if c != ' ' {
                    to = l- i;
                    break;
                }
            }
            &s[from..to]
        }
        println!("{:?}",trim_spaces(" some space "));
        println!("{:?}",trim_spaces("                                      some more space                 "));
        println!("{:?}",trim_spaces("             😶  😀 🥶     some more space                 "));
    }, } .run(true);
    #[rustfmt::skip]Exercise { id: 35, name: "8.1 Modules :: Rust std lib and prelude", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 36, name: "8.2 Modules :: standard Input", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 37, name: "8.3 Modules :: parse Strings", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 38, name: "8.4 Modules :: Crates", runnable: || {}, } .run(false);
    #[rustfmt::skip]Challenge { id: 39, name: "8.5 Modules :: Challenge : higher or lower", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 40, name: "9.1 Input and Output :: command Line Arguments", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 41, name: "9.2 Input and Output :: reading from Files", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 42, name: "9.3 Input and Output :: writing to Files", runnable: || {}, } .run(false);
    #[rustfmt::skip]Challenge { id: 43, name: "9.4 Input and Output :: Challenge : check the Roster", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 44, name: "10.1 Structs :: defining Structs", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 45, name: "10.2 Structs :: update Syntax", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 46, name: "10.3 Structs :: Methods", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 47, name: "10.4 Structs :: associated Functions", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 48, name: "10.5 Structs :: tuple Structs", runnable: || {}, } .run(false);
    #[rustfmt::skip]Challenge { id: 48, name: "10.6 Structs :: Challenge : represent Shapes", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 59, name: "11.1 Generics :: struct Definitions", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 50, name: "11.2 Generics :: method Definitions", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 51, name: "11.3 Generics :: function Definitions", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 52, name: "11.4 Generics :: Box data Type", runnable: || {}, } .run(false);
    #[rustfmt::skip]Challenge { id: 53, name: "11.5 Generics :: Challenge : Sum Boxes", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 54, name: "12.1 Traits :: implement Traits", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 55, name: "12.2 Traits :: default Trait Implementation", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 56, name: "12.3 Traits :: derive Traits", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 57, name: "12.4 Traits :: Trait bounds", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 50, name: "12.5 Traits :: multiple Trait Bounds", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 51, name: "12.6 Traits :: Return Types with implemented Traits", runnable: || {}, } .run(false);
    #[rustfmt::skip]Challenge { id: 62, name: "12.7 Traits :: Challenge : implement the display Trait", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 63, name: "13.1 Lifetimes :: the borrow Checker", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 64, name: "13.2 Lifetimes :: Lifetime Annotation Syntax", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 65, name: "13.3 Lifetimes :: multiple Lifetime Annotations", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 66, name: "13.4 Lifetimes :: Lifetime elision Rules", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 67, name: "13.5 Lifetimes :: Struct Lifetime Annotations", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 68, name: "13.6 Lifetimes :: Static Lifetime", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 69, name: "14.1 Enums :: define Enums", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 70, name: "14.2 Enums :: Match Operator", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 71, name: "14.3 Enums :: Match with default Placeholder", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 72, name: "14.4 Enums :: Enum Methods", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 73, name: "14.5 Enums :: Option<T> Enum", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 74, name: "14.6 Enums :: matching Option<T>", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 75, name: "14.7 Enums :: if-let syntax", runnable: || {}, } .run(false);
    #[rustfmt::skip]Challenge { id: 76, name: "14.8 Enums :: Challenge : represent a Location", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 77, name: "15.1 Error Handling :: Unrecoverable Errors", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 78, name: "15.2 Error Handling :: Result<T,E> enum", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 79, name: "15.3 Error Handling :: Matching Result<T,E>", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 80, name: "15.4 Error Handling :: Propagating Errors", runnable: || {}, } .run(false);
    #[rustfmt::skip]Challenge { id: 81, name: "15.5 Error Handling :: Challenge : Handle Errors", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 82, name: "16.1 Collections :: Vectors", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 83, name: "16.2 Collections :: HashMaps", runnable: || {}, } .run(false);
    #[rustfmt::skip]Challenge { id: 84, name: "16.3 Collections :: Challenge : Count Words", runnable: || {}, } .run(false);

    Challenge {
        id: 85,
        name: "equations",
        runnable: || {
            println!("3.5x + 4 = 0 -> x={:?}", first_degree(3.5, 4.0));
            let sd = second_degree(5.0, -40.0, 75.0);
            println!("x^2 +100 - 40x + 4x^2 = 25; 5x^2 -40x + 75 = 0; {:?}", sd);
            match sd {
                Some(pair) => {
                    println!("sd={:?}", pair);
                    println!("x1 = {}", pair.0);
                    println!("y1 = {}", 10.0 - 2.0 * pair.0);
                    println!("x1 + y1 = {}", 10.0 - pair.0);
                    println!("x2 = {}", pair.1);
                    println!("y2 = {}", 10.0 - 2.0 * pair.1);
                    println!("x2 + y2 = {}", 10.0 - pair.1);
                }
                None => println!("no solution"),
            }
        },
    }
    .run(false);
}
