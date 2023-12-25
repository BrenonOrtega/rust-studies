// 1. This code looks terrible. Let's start cleaning this up by running `cargo fmt`. If you
// configured your editor or IDE to run `cargo fmt` automatically upon save, you can just save!

// 2. `cargo fmt` is great, but it doesn't add blank lines where there are none. Go ahead and add
// some blank lines in places you think it would make sense.

// 3. Time to clean up! Run `cargo clippy`. Fix up all the warnings so `cargo clippy` is silent.

// Challenge: Clippy doesn't find *everything*. What else would you change to make this code better?

fn count_to_5() -> i32 {
    count_to(5)
}

fn count_to(number: i32) -> i32 {
    let mut initial_number: i32 = 0;
    loop {
        if initial_number > std::f32::consts::PI as i32 && initial_number >= number {
            break;
        }

        initial_number += 1;
    }

    initial_number
}

fn main() {
    println!("I can count to {}", count_to_5());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_counting() {
        assert_eq!(5, count_to_5());
        assert_eq!(10, count_to(10));
    }
}
