#[macro_export]
macro_rules! days {
    ( $($x:expr),*) => {
        use std::time::{Duration, Instant};

        paste::paste! {
            $(
                mod [<day $x>];
            )*

            pub fn run_day(day: &str) {
                $(
                    if stringify!($x) == day {
                        println!("day {}:", $x);
                        let start = Instant::now();
                        [<day $x>]::main();
                        let dt =  Instant::now() - start;
                        println!("time: {:?}\n", dt);
                        return;
                    }
                )*
                panic!("invalid day: {}", day);
            }

            pub fn run_all() {
                let mut total = Duration::default();
                $(
                    println!("day {}:", $x);
                    let start = Instant::now();
                    [<day $x>]::main();
                    let dt =  Instant::now() - start;
                    println!("time: {:?}\n", dt);
                    total += dt;
                )*
                println!("total: {:?}", total);
            }
        }
    };
}
