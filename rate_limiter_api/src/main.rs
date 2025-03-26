use std::collections::HashMap;
use std::time::{SystemTime,UNIX_EPOCH};

// Struct to hold rate limit data for a client
#[derive(Debug)]
struct RateLimit{
    count : u32,
    window_start : u64,
}

// Struct for the rate limiter
#[derive(Debug)]
struct RateLimiter {
    limits : HashMap<String, RateLimit>,
    max_requests : u32,
    window_size : u64,
}

impl RateLimiter {
    fn new(max_requests : u32, window_size : u64) -> Self{
        RateLimiter { limits: HashMap::new(), max_requests, window_size }
    }

    fn check_limit(&mut self, api_key : &str) -> bool {
        let now = SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .unwrap()
                                .as_secs();
        let entry = self.limits.entry(api_key.to_string())
                                                    .or_insert(RateLimit { 
                                                        count: 0, window_start: now
                                                     });

        if now - entry.window_start >= self.window_size {
            entry.count = 0;
            entry.window_start = now;
        }

        if entry.count < self.max_requests{
            entry.count += 1;
            true // Request allowed
        }
        else {
            false // Request denied
        }
    }
}

fn main() {
    let mut limiter = RateLimiter::new(5, 60);

    let api_key = "user123";

    for _ in 0..7{
        if limiter.check_limit(api_key){
            println!("Request allowed for {}", api_key);
        }else {
            println!("Rate limit exceeded for {}", api_key);
        }
    }

    println!("Limiter state: {:?}", limiter);
}
