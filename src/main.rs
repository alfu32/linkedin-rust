mod exercise;
use crate::exercise::Exercise;

use std::vec::Vec;

type Challenge = Exercise;

const exercises: [(u64, &str); 83] = [
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
    (25u64, "6.4 Ownership :: strings"),
    (26u64, "6.5 Ownership :: ownership"),
    (27u64, "6.6 Ownership :: moving, cloning, copying data"),
    (28u64, "6.7 Ownership :: transfering ownership"),
    (29u64, "7.1 References :: Borrowing references"),
    (30u64, "7.2 References :: Mutable references"),
    (31u64, "7.3 References :: Dangling references"),
    (32u64, "7.4 References :: Slices as function parameters"),
    (33u64, "7.5 References :: Challenge: Trim Spaces"),
    (34u64, "8.1 Modules :: Rust std lib and prelude"),
    (35u64, "8.2 Modules :: standard Input"),
    (36u64, "8.3 Modules :: parse Strings"),
    (37u64, "8.4 Modules :: Crates"),
    (38u64, "8.5 Modules :: Challenge : higher or lower"),
    (39u64, "9.1 Input and Output :: command Line Arguments"),
    (40u64, "9.2 Input and Output :: reading from Files"),
    (41u64, "9.3 Input and Output :: writing to Files"),
    (
        42u64,
        "9.4 Input and Output :: Challenge : check the Roster",
    ),
    (43u64, "10.1 Structs :: defining Structs"),
    (44u64, "10.2 Structs :: update Syntax"),
    (45u64, "10.3 Structs :: Methods"),
    (46u64, "10.4 Structs :: associated Functions"),
    (47u64, "10.5 Structs :: tuple Structs"),
    (48u64, "10.6 Structs :: Challenge : represent Shapes"),
    (48u64, "11.1 Generics :: struct Definitions"),
    (59u64, "11.2 Generics :: method Definitions"),
    (50u64, "11.3 Generics :: function Definitions"),
    (51u64, "11.4 Generics :: Box data Type"),
    (52u64, "11.5 Generics :: Challenge : Sum Boxes"),
    (53u64, "12.1 Traits :: implement Traits"),
    (54u64, "12.2 Traits :: default Trait Implementation"),
    (55u64, "12.3 Traits :: derive Traits"),
    (56u64, "12.4 Traits :: Trait bounds"),
    (57u64, "12.5 Traits :: multiple Trait Bounds"),
    (50u64, "12.6 Traits :: Return Types with implemented Traits"),
    (
        51u64,
        "12.7 Traits :: Challenge : implement the display Trait",
    ),
    (62u64, "13.1 Lifetimes :: the borrow Checker"),
    (63u64, "13.2 Lifetimes :: Lifetime Annotation Syntax"),
    (64u64, "13.3 Lifetimes :: multiple Lifetime Annotations"),
    (65u64, "13.4 Lifetimes :: Lifetime elision Rules"),
    (66u64, "13.5 Lifetimes :: Struct Lifetime Annotations"),
    (67u64, "13.6 Lifetimes :: Static Lifetime"),
    (68u64, "14.1 Enums :: define Enums"),
    (69u64, "14.2 Enums :: Match Operator"),
    (70u64, "14.3 Enums :: Match with default Placeholder"),
    (71u64, "14.4 Enums :: Enum Methods"),
    (72u64, "14.5 Enums :: Option<T> Enum"),
    (73u64, "14.6 Enums :: matching Option<T>"),
    (74u64, "14.7 Enums :: if-let syntax"),
    (75u64, "14.8 Enums :: Challenge : represent a Location"),
    (76u64, "15.1 Error Handling :: Unrecoverable Errors"),
    (77u64, "15.2 Error Handling :: Result<T,E> enum"),
    (78u64, "15.3 Error Handling :: Matching Result<T,E>"),
    (79u64, "15.4 Error Handling :: Propagating Errors"),
    (80u64, "15.5 Error Handling :: Challenge : Handle Errors"),
    (81u64, "16.1 Collections :: Vectors"),
    (82u64, "16.2 Collections :: HashMaps"),
    (83u64, "16.3 Collections :: Challenge : Count Words"),
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
        runnable: || {},
    }
    .run(false);
    exercise!(33, "test", {
        println!("hello");
    })
    .run(true);
    #[rustfmt::skip]Exercise { id: 23, name: "6.2 Ownership :: shadowing variables", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 24, name: "6.3 Ownership :: stack and heap memory", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 25, name: "6.4 Ownership :: strings", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 26, name: "6.5 Ownership :: ownership", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 27, name: "6.6 Ownership :: moving, cloning, copying data", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 28, name: "6.7 Ownership :: transfering ownership", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 29, name: "7.1 References :: Borrowing references", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 30, name: "7.2 References :: Mutable references", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 31, name: "7.3 References :: Dangling references", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 32, name: "7.4 References :: Slices as function parameters", runnable: || {}, } .run(false);
    #[rustfmt::skip]Challenge { id: 33, name: "7.5 References :: Challenge: Trim Spaces", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 34, name: "8.1 Modules :: Rust std lib and prelude", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 35, name: "8.2 Modules :: standard Input", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 36, name: "8.3 Modules :: parse Strings", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 37, name: "8.4 Modules :: Crates", runnable: || {}, } .run(false);
    #[rustfmt::skip]Challenge { id: 38, name: "8.5 Modules :: Challenge : higher or lower", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 39, name: "9.1 Input and Output :: command Line Arguments", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 40, name: "9.2 Input and Output :: reading from Files", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 41, name: "9.3 Input and Output :: writing to Files", runnable: || {}, } .run(false);
    #[rustfmt::skip]Challenge { id: 42, name: "9.4 Input and Output :: Challenge : check the Roster", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 43, name: "10.1 Structs :: defining Structs", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 44, name: "10.2 Structs :: update Syntax", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 45, name: "10.3 Structs :: Methods", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 46, name: "10.4 Structs :: associated Functions", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 47, name: "10.5 Structs :: tuple Structs", runnable: || {}, } .run(false);
    #[rustfmt::skip]Challenge { id: 48, name: "10.6 Structs :: Challenge : represent Shapes", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 48, name: "11.1 Generics :: struct Definitions", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 59, name: "11.2 Generics :: method Definitions", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 50, name: "11.3 Generics :: function Definitions", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 51, name: "11.4 Generics :: Box data Type", runnable: || {}, } .run(false);
    #[rustfmt::skip]Challenge { id: 52, name: "11.5 Generics :: Challenge : Sum Boxes", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 53, name: "12.1 Traits :: implement Traits", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 54, name: "12.2 Traits :: default Trait Implementation", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 55, name: "12.3 Traits :: derive Traits", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 56, name: "12.4 Traits :: Trait bounds", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 57, name: "12.5 Traits :: multiple Trait Bounds", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 50, name: "12.6 Traits :: Return Types with implemented Traits", runnable: || {}, } .run(false);
    #[rustfmt::skip]Challenge { id: 51, name: "12.7 Traits :: Challenge : implement the display Trait", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 62, name: "13.1 Lifetimes :: the borrow Checker", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 63, name: "13.2 Lifetimes :: Lifetime Annotation Syntax", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 64, name: "13.3 Lifetimes :: multiple Lifetime Annotations", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 65, name: "13.4 Lifetimes :: Lifetime elision Rules", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 66, name: "13.5 Lifetimes :: Struct Lifetime Annotations", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 67, name: "13.6 Lifetimes :: Static Lifetime", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 68, name: "14.1 Enums :: define Enums", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 69, name: "14.2 Enums :: Match Operator", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 70, name: "14.3 Enums :: Match with default Placeholder", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 71, name: "14.4 Enums :: Enum Methods", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 72, name: "14.5 Enums :: Option<T> Enum", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 73, name: "14.6 Enums :: matching Option<T>", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 74, name: "14.7 Enums :: if-let syntax", runnable: || {}, } .run(false);
    #[rustfmt::skip]Challenge { id: 75, name: "14.8 Enums :: Challenge : represent a Location", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 76, name: "15.1 Error Handling :: Unrecoverable Errors", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 77, name: "15.2 Error Handling :: Result<T,E> enum", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 78, name: "15.3 Error Handling :: Matching Result<T,E>", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 79, name: "15.4 Error Handling :: Propagating Errors", runnable: || {}, } .run(false);
    #[rustfmt::skip]Challenge { id: 80, name: "15.5 Error Handling :: Challenge : Handle Errors", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 81, name: "16.1 Collections :: Vectors", runnable: || {}, } .run(false);
    #[rustfmt::skip]Exercise { id: 82, name: "16.2 Collections :: HashMaps", runnable: || {}, } .run(false);
    #[rustfmt::skip]Challenge { id: 83, name: "16.3 Collections :: Challenge : Count Words", runnable: || {}, } .run(false);
}
