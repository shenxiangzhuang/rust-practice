// https://www.codewars.com/kata/52f787eb172a8b4ae1000a34/train/rust
use std::time::Instant;
use std::collections::HashMap;


fn zeros_skip_slow(n: u64) -> u64 {

    if n < 5 {return 0;}
    let mut count: u64 = 0;
    for i in (5..=n).step_by(5) {
        let mut j = i;
        let mut five_times: u64 = 0;
        while j % 5 == 0 {
                five_times += 1;
                j /= 5;
            }
        count += five_times;
        }
    count
}

fn zeros_cache_slow(n: u64) -> u64 {
    if n < 5 {return 0;}

    let mut cache: HashMap<u64, u64> = HashMap::new();
    let mut count: u64 = 0;

    for i in (5..=n).step_by(5) {
        let mut j = i;
        let mut five_times: u64 = 0;
        while j % 5 == 0 {
            five_times += 1;
            j /= 5;
            if cache.contains_key(&j) {
                five_times += cache.get(&j).unwrap();
                // println!("hit {j}");
                break
            }
        }
        cache.insert(i, five_times);
        // println!("{:?}", cache);
        count += five_times;
    }
    count
}

/// 先数5的的个数(5=5^1, 每个对应1个0)，再数5*5=25的个数(25=5^2, 每个对应2个0，之前数了一次，再加一次), 依次类推
fn zeros_fast(n: u64) -> u64 {
    if n < 5 { return 0; }
    let mut count: u64 = 0;
    let mut five_times: u64 = 5;
    while five_times < n {
        count += n / five_times;
        five_times *= 5;
    }
    count
}


fn main() {
    let n:u64 = 1000000;

    let start_skip = Instant::now();
    let res_skip_slow = zeros_skip_slow(n);
    let duration_skip = start_skip.elapsed();
    println!("{res_skip_slow}");
    println!("Time elapsed in zeros_skip_slow() is: {:?}", duration_skip);

    let start_cache = Instant::now();
    let res_cache_slow = zeros_cache_slow(n);
    let duration_cache = start_cache.elapsed();
    println!("Time elapsed in zeros_cache_slow() is: {:?}", duration_cache);
    println!("{res_cache_slow}");

    let start_fast = Instant::now();
    let res_cache_fast = zeros_fast(n);
    let duration_fast = start_fast.elapsed();
    println!("Time elapsed in zeros_fast() is: {:?}", duration_fast);
    println!("{res_cache_fast}");
}
