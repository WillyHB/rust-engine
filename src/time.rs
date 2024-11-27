use std::time::Instant;

pub struct Timer {

    start : Instant,
    now : f64,
    last : f64,

}

impl Timer {

    pub fn new() -> Timer {

        Timer { start : Instant::now(), now : 0.0, last : 0.0 }
    }

    pub fn update(&mut self) {

        self.last = self.now;
        self.now = self.elapsed_secs_f64();

    }

    pub fn delta_time(&self) -> f64 {

        self.now - self.last
    }

    pub fn elapsed_milis(&self) -> u128 {

        self.start.elapsed().as_millis()
    }

    pub fn elapsed_secs(&self) -> u64 {

        self.start.elapsed().as_secs()
    }

    pub fn elapsed_secs_f64(&self) -> f64 {

        self.start.elapsed().as_secs_f64()
    }

    pub fn elapsed_micros(&self) -> u128 {

        self.start.elapsed().as_micros()
    }

    pub fn elapsed_nanos(&self) -> u128 {

        self.start.elapsed().as_nanos()
    }

    pub fn run_time(&self, f : &dyn Fn()) -> u128 {

        let start = self.elapsed_nanos();
        f();
        let end = self.elapsed_nanos();

        end - start
    }

}