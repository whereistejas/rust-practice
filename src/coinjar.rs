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

#[derive(Debug)]
struct CoinJar {
    quantity: i32,
    sum: i32,
}
// }}}

fn main() {
    // Set the value and quantity of each type of coin in the jar {{{
    let penny = Coins::Penny(GenericCoin {
        value: 1,
        quantity: 2,
    });
    let nickel = Coins::Nickel(GenericCoin {
        value: 5,
        quantity: 2,
    });
    let dime = Coins::Dime(GenericCoin {
        value: 10,
        quantity: 2,
    });
    let quarter = Coins::Quarter(GenericCoin {
        value: 25,
        quantity: 2,
    });
    // }}}

    // put all the different types of coins into one vector
    let contents = vec![penny, nickel, dime, quarter];

    let mut coinjar = CoinJar {
        quantity: 0,
        sum: 0,
    };

    // find the total number of coins and the sume of their values
    for coin in &contents {
        match coin {
            Coins::Penny(_coin) => sum_and_quantity(&mut coinjar, &_coin),
            Coins::Nickel(_coin) => sum_and_quantity(&mut coinjar, &_coin),
            Coins::Dime(_coin) => sum_and_quantity(&mut coinjar, &_coin),
            Coins::Quarter(_coin) => sum_and_quantity(&mut coinjar, &_coin),
        }
    }

    println!("{:#?}", coinjar)
}

fn sum_and_quantity(coinjar: &mut CoinJar, coin: &GenericCoin) {
    coinjar.quantity = coinjar.quantity + coin.quantity; // qty_already_in_the_jar = qty_already_in_the_jar + qty_coin
    coinjar.sum = coinjar.sum + (coin.quantity * coin.value); // existing_sum = existing_sum + (qty_coin * value_of_coin)
}
