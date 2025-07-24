use std::sync::Arc;

#[tokio::main]
async fn main() {
    let mut args = String::new();
    for arg in std::env::args() {
        args.push_str(&arg);
        args.push_str(" ");
    }
    let mut avg: u8 = 0;
    let mut min: u8 = 0;
    let mut max: u8 = 0;

    let nums = parse2vec(&args);
    let nums = Arc::new(nums);
    let avg_nums = Arc::clone(&nums);
    let avg_handle = tokio::spawn(async move { calc_avg(avg_nums) });
    let min_nums = Arc::clone(&nums);
    let min_handle = tokio::spawn(async move { calc_min(min_nums) });
    let max_nums = Arc::clone(&nums);
    let max_handle = tokio::spawn(async move { calc_max(max_nums) });

    let res = tokio::join!(avg_handle, min_handle, max_handle);
    res.0
        .and_then(|x| {
            avg = x;
            Ok(())
        })
        .unwrap();
    res.1
        .and_then(|x| {
            min = x;
            Ok(())
        })
        .unwrap();
    res.2
        .and_then(|x| {
            max = x;
            Ok(())
        })
        .unwrap();
    println!("avg:{},min:{},max:{}", avg, min, max);
}

fn parse2vec(nums: &str) -> Vec<u8> {
    let mut res = Vec::new();
    for s in nums.split(" ") {
        if let Ok(n) = s.parse::<u8>() {
            res.push(n);
        }
    }
    res
}

fn calc_avg(nums: Arc<Vec<u8>>) -> u8 {
    let count = nums.len();
    let mut sum: u16 = 0;
    for n in nums.iter() {
        sum += *n as u16;
    }
    (sum / count as u16) as u8
}

fn calc_min(nums: Arc<Vec<u8>>) -> u8 {
    *nums.iter().min().unwrap()
}

fn calc_max(nums: Arc<Vec<u8>>) -> u8 {
    *nums.iter().max().unwrap()
}
