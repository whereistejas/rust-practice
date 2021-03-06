// This programs finds the number of coins and their summed up value
// Data model {{{
#[derive(Debug)]
struct GenericCoin {
    pub value: i32,
    pub quantity: i32,
}

#[derive(Debug)]
enum Coins {
    Penny(GenericCoin),
    Dime(GenericCoin),
    Nickel(GenericCoin),
    Quarter(GenericCoin),
}

impl Coins {
    pub fn get_coin(coin: String, qty: i32) -> Option<Coins> {
        let get_generic_coin = |value| {
            GenericCoin {
                value,
                quantity: qty,
            }
        };

        match coin.as_str() {
            "Penny" => Some(Coins::Penny(get_generic_coin(1))),
            "Nickel" => Some(Coins::Nickel(get_generic_coin(5))),
            "Dime" => Some(Coins::Dime(get_generic_coin(10))),
            "Quarter" => Some(Coins::Quarter(get_generic_coin(25))),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct CoinJar<'a> {
    coins: &'a Vec<Coins>,
    quantity: i32,
    sum: i32,
}

impl CoinJar<'_> {
    pub fn new(coins: &Vec<Coins>) -> CoinJar {
        CoinJar {
            coins,
            quantity: 0,
            sum: 0,
        }
    }

    fn match_coin(coin: &Coins) -> &GenericCoin {
        match coin {
            Coins::Penny(gc) => gc,
            Coins::Dime(gc) => gc,
            Coins::Nickel(gc) => gc,
            Coins::Quarter(gc) => gc,
        }
    }

    pub fn get_total_qty(&mut self) -> i32 {
        for coin in self.coins.iter() {
            self.quantity = self.quantity + Self::match_coin(coin).quantity;
        }

        self.quantity
    }

    pub fn get_total_sum(&mut self) -> i32 {
        for coin in self.coins.iter() {
            self.sum = self.sum + (Self::match_coin(coin).quantity * Self::match_coin(coin).value);
        }

        self.sum
    }
}

fn main() {
    // Set the value and quantity of each type of coin in the jar
    let penny = Coins::get_coin("Penny".to_string(), 10).unwrap();
    let nickel = Coins::get_coin("Nickel".to_string(), 10).unwrap();
    let dime = Coins::get_coin("Dime".to_string(), 10).unwrap();
    let quarter = Coins::get_coin("Quarter".to_string(), 10).unwrap();

    // put all the different types of coins into one vector
    let contents = vec![penny, nickel, dime, quarter];

    let mut coinjar = CoinJar::new(&contents);
    println!("Quantity of coins: {}", coinjar.get_total_qty());
    println!("Sum of coins: {}", coinjar.get_total_sum());
}

// @whereistejas in your specific case, you might have wanted a more orthogonal design for Coins, since all the variants actually share the same data
// You could have done something like
// ```rust
// enum CoinKind {
//     Penny,
//     Dime,
//     Nickel,
//     Quarter,
// }
// ```
// and you'd then have
// ```rust
// struct Coin {
//     value: i32,
//     quantity: i32,
//     kind: CoinKind
// }
// ```
