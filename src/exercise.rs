use std::sync::atomic::{AtomicU64, Ordering};

pub(crate) struct Exercise {
    pub(crate) id: u64,
    pub(crate) name: &'static str,
    pub(crate) runnable: fn(),
}

impl Exercise {
    pub(crate) fn new(name: &'static str, runnable: fn()) -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);
        Exercise {
            id,
            name: name,
            runnable,
        }
    }
    pub(crate) fn run(&self, will_run: bool) {
        if will_run {
            use std::time::Instant;
            let now = Instant::now();
            println!(
                "--|{:4}|--{:-<64}-------------------------------------------",
                self.id, self.name
            );
            println!("      start {:?}\n", now);
            (self.runnable)();
            println!("\n      done {:.2?}", now.elapsed());
            println!(
                "--|{:4}|--{:-<64}-------------------------------------------",
                self.id, self.name
            );
        } else {
            println!("(skipped) [{:4}] {}", self.id, self.name);
        }
    }
}

#[macro_export]
macro_rules! exercise {
    ( $id:expr, $n:expr , $b:block ) => {{
        Exercise {
            id: $id,
            name: $n,
            runnable: || $b,
        }
    }};
}
