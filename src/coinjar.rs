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
    // Define types of coin {{{
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

    // put all the different coins into one vector
    let contents = vec![penny, nickel, dime, quarter];

    let mut coinjar = CoinJar {
        quantity: 0,
        sum: 0,
    };

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
    coinjar.quantity = coinjar.quantity + coin.quantity;
    coinjar.sum = coinjar.sum + (coin.quantity * coin.value);
}
