// modules2.rs
// You can bring module paths into scopes and provide new names for them with the
// 'use' and 'as' keywords. Fix these 'use' statements to make the code compile.
// Execute `rustlings hint modules2` or use the `hint` watch subcommand for a hint.

pub mod delicious_snacks {
    // Corrected use statements
    pub use self::fruits::PEAR as P;
    pub use self::veggies::CUCUMBER as C;

    pub mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    pub mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }
}

use delicious_snacks::*;

fn main() {
    println!(
        "favorite snacks: {} and {}",
        P,
        C
    );
}
