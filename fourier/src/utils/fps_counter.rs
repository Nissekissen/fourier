use std::time::{Duration, Instant};

pub struct FpsCounter {
    start_time: Instant,
    frame_count: u64,
}

impl FpsCounter {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            frame_count: 0,
        }
    }

    pub fn execute(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.start_time);
        self.frame_count += 1;

        if elapsed >= Duration::from_secs(1) {
            let fps = self.frame_count as f32 / elapsed.as_secs_f32();
            println!("FPS: {}", fps);
            self.frame_count = 0;
            self.start_time = now;
        }
    }
}
