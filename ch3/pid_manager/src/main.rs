use std::{sync::Arc, time::Duration};
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let pid_manager = Arc::new(Mutex::new(PidManager::new()));
    let mut handlers = Vec::new();
    for _ in 0..100 {
        let pid_manager_clone = Arc::clone(&pid_manager);
        handlers.push(tokio::spawn(async move {
            let pid = pid_manager_clone.lock().await.allocate_pid();
            match pid {
                Ok(pid) => {
                    println!("Allocated PID: {}", pid);
                    tokio::time::sleep(Duration::from_secs(3)).await;
                    pid_manager_clone.lock().await.relesase_pid(pid);
                    println!("Released PID: {}", pid);
                }
                Err(_e) => {
                    println!("Allocated PID failed");
                }
            }
        }));
    }
    for handler in handlers {
        handler.await.expect("Task failed");
    }
}

const MIN_PID: u16 = 300;
const MAX_PID: u16 = 350;
struct PidManager {
    released: Vec<u16>,
}
impl PidManager {
    pub fn new() -> Self {
        Self {
            released: Vec::new(),
        }
    }
    pub fn allocate_pid(&mut self) -> Result<u16, PidManagerError> {
        let maxcount = (MAX_PID - MIN_PID) as usize;
        if self.released.iter().count() >= maxcount {
            return Err("there is no more pid to release".into());
        }
        if self.released.len() == 0 {
            let released = MIN_PID;
            self.released.push(released);
            return Ok(released);
        } else {
            self.released.sort();
            for pid in &self.released {
                let npid = pid + 1;
                if !self.released.contains(&npid) {
                    self.released.push(npid);
                    return Ok(npid);
                }
            }
        }

        Err("no more pid to release".into())
    }
    pub fn relesase_pid(&mut self, pid: u16) {
        if let Ok(index) = self.released.binary_search(&pid) {
            self.released.remove(index);
        }
    }
}

#[derive(Debug)]
struct PidManagerError {
    message: String,
}

impl std::fmt::Display for PidManagerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for PidManagerError {}
unsafe impl Send for PidManagerError {}

impl From<&str> for PidManagerError {
    fn from(value: &str) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}
