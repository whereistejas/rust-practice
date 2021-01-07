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
    quantity: i32,
    sum: i32,
}
// }}}

fn main() {
    // Set the value and quantity of each type of coin in the jar
    let penny = Coins::get_coin("Penny".to_string(), 10).unwrap();
    let nickel = Coins::get_coin("Nickel".to_string(), 10).unwrap();
    let dime = Coins::get_coin("Dime".to_string(), 10).unwrap();
    let quarter = Coins::get_coin("Quarter".to_string(), 10).unwrap();

    // put all the different types of coins into one vector
    let contents = vec![penny, nickel, dime, quarter];

    let mut coinjar = CoinJar {
        quantity: 0,
        sum: 0,
    };

    // find the total number of coins and the sume of their values
    for coin in &contents {
        match coin {
            Coins::Penny(coin) => sum_and_quantity(&mut coinjar, &coin),
            Coins::Nickel(coin) => sum_and_quantity(&mut coinjar, &coin),
            Coins::Dime(coin) => sum_and_quantity(&mut coinjar, &coin),
            Coins::Quarter(coin) => sum_and_quantity(&mut coinjar, &coin),
        }
    }

    println!("{:#?}", coinjar)
}

fn sum_and_quantity(coinjar: &mut CoinJar, coin: &GenericCoin) {
    coinjar.quantity = coinjar.quantity + coin.quantity; // qty_already_in_the_jar = qty_already_in_the_jar + qty_coin
    coinjar.sum = coinjar.sum + (coin.quantity * coin.value); // existing_sum = existing_sum + (qty_coin * value_of_coin)
}
