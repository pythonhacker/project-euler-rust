// Coin sums
// How many different ways can 2 pounds be made using any number of coins?

fn coin_sums(total: i32, max_coin: i32) -> u32 {

    let coins = [200,100,50,20,10,5,2,1];
    let mut sum: u32 = 0;
    let mut diff: i32;
    
    if max_coin == 7 {
        return 1;
    }

    for i in max_coin..8 {
        println!("{} {}", total, coins[i as usize]);
        diff = total - coins[i as usize];
        if diff == 0 { sum += 1; }
        if diff > 0 { sum += coin_sums(diff, i); }
    }

    return sum;
}

fn main() {
    println!("#ways: {}",coin_sums(200, 0));
}
