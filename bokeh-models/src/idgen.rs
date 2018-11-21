use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

static DEFAULT_COUNTER_START: i32 = 1000;

struct IdGenerator {
    value: i32,
}

impl IdGenerator {
    fn next_id(&mut self) -> i32 {
        self.value += 1;
        self.value
    }
}

lazy_static! {
    static ref GENERATOR: Arc<Mutex<IdGenerator>> = Arc::new(Mutex::new(IdGenerator {
        value: DEFAULT_COUNTER_START
    }));
}

pub(crate) fn create_id() -> i32 {
    GENERATOR.lock().unwrap().next_id()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter() {
        let mut gen = IdGenerator {
            value: DEFAULT_COUNTER_START,
        };

        let ids: Vec<i32> = (0..5).map(|_| gen.next_id()).collect();
        assert_eq!(ids, &[1001, 1002, 1003, 1004, 1005]);
    }
}
