// This programs finds the number of coins and their summed up value
// Data model {{{
#[derive(Debug)]
struct GenericCoin {
    value: i32,
    quantity: i32,
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
        match coin.as_str() {
            "Penny" => {
                return Some(Coins::Penny(GenericCoin {
                    value: 1,
                    quantity: qty,
                }));
            }
            "Nickel" => {
                return Some(Coins::Nickel(GenericCoin {
                    value: 5,
                    quantity: qty,
                }))
            }
            "Dime" => {
                return Some(Coins::Dime(GenericCoin {
                    value: 10,
                    quantity: qty,
                }))
            }
            "Quarter" => {
                return Some(Coins::Quarter(GenericCoin {
                    value: 25,
                    quantity: qty,
                }))
            }
            _ => return None,
        }
    }
}

#[derive(Debug)]
struct CoinJar {
    coinjar: Vec<Coins>,
    quantity: i32,
    sum: i32,
}

impl CoinJar {
    pub fn new(coinjar: &Vec<Coins>) -> CoinJar {
        CoinJar {
            coinjar,
            quantity: 0,
            sum: 0,
        }
    }

    pub fn get_total_qty(&mut self) -> i32 {
        for coin in self.coinjar.iter() {
            self.quantity = self.quantity + coin.quantity;
        }

        self.quantity
    }

    pub fn get_total_sum(&mut self) -> i32 {
        for coin in self.coinjar.iter() {
            self.sum = self.sum + (*coin.quantity * *coin.value);
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
