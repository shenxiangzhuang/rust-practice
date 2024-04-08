use std::collections::HashMap;

// https://www.codewars.com/kata/52f787eb172a8b4ae1000a34/train/rust
fn zeros(n: u64) -> u64 {
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
                println!("hit {j}");
                break
            }
        }
        cache.insert(i, five_times);
        // println!("{:?}", cache);
        count += five_times;
    }
    count
}


fn main() {
    let res = zeros(100);
    println!("{res}");
}
