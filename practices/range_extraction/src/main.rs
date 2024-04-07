pub fn range_extraction(a: &[i32]) -> String {
    let mut res = "".to_string();
    let mut i = 0;
    let mut j = 0;
    while i < a.len() && j <= a.len() {
            if j != a.len() && (j - i) == (a[j] - a[i]) as usize {
                j += 1;
            }
            else if j - i >= 3 {
                res = format!("{},{}-{}", res, a[i], a[j-1]);
                i = j;
            }
            else if j - i >= 1 {
                for k in i..j {
                    res = format!("{},{}", res, a[k]);
                }
                i = j;
            }
            // println!("{i}, {j}: {res}");
    }
    res.strip_prefix(',').unwrap().to_string()
}


fn main() {
    let solution = range_extraction(
        &[-10, -9, -8, -6, -3, -2, -1, 0, 1, 3, 4, 5, 7, 8, 9, 10, 11, 14, 15, 17, 18, 19, 20]);
    println!("{}", solution);
}
