use std::collections::VecDeque;

struct RecentCounter {
    time_queue: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {
    fn new() -> Self {
        RecentCounter {
            time_queue: VecDeque::new(),
        }
    }

    fn ping(&mut self, t: i32) -> i32 {
        self.time_queue.push_back(t);
        while *self.time_queue.front().unwrap() < t - 3000 {
            self.time_queue.pop_front();
        }
        self.time_queue.len() as i32
    }
}

/**
 * Your RecentCounter object will be instantiated and called as such:
 * let obj = RecentCounter::new();
 * let ret_1: i32 = obj.ping(t);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0933() {
        let mut recent_counter = RecentCounter::new();
        assert_eq!(recent_counter.ping(1), 1);
        assert_eq!(recent_counter.ping(100), 2);
        assert_eq!(recent_counter.ping(3001), 3);
        assert_eq!(recent_counter.ping(3002), 3);
    }
}
