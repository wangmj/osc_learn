use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    let max_num = std::env::args().skip(1).nth(0);
    match max_num {
        Some(num_str) => {
            if let Ok(num) = num_str.parse::<u16>() {
                let res = Arc::new(Mutex::new(Vec::new()));
                let res_clone=Arc::clone(&res);
                let handle = tokio::spawn(async move { calc_all_prime_nums(num, res) });
                handle.await.unwrap();
                for i in res_clone.lock().unwrap().iter(){
                    println!("{}",i);
                }
            }
        }
        None => {
            println!("please input some num to calc prime num");
        }
    }
}

fn calc_all_prime_nums(max: u16, res: Arc<Mutex<Vec<u16>>>) {
    for i in 0..max {
        if is_prime_num(i) {
            res.lock().unwrap().push(i);
        }
    }
}

fn is_prime_num(num: u16) -> bool {
    let mut res = true;
    for i in 2..num {
        if num % i == 0 {
            res = false;
            break;
        }
    }
    res
}
