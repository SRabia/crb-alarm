use std::time::Instant;

pub struct Fps {
    last_frame_update: Instant,
    frame_count: u32,
    fps: f64,
}

impl Fps {
    pub fn new() -> Self {
        Self {
            last_frame_update: Instant::now(),
            frame_count: 0,
            fps: 0.0,
        }
    }

    pub fn update(&mut self) {
        self.frame_count += 1;
        let now = Instant::now();
        let elapsed = (now - self.last_frame_update).as_secs_f64();
        if elapsed >= 1.0 {
            self.fps = self.frame_count as f64 / elapsed;
            self.last_frame_update = now;
            self.frame_count = 0;
        }
    }

    pub fn fps(&self) -> f64 {
        self.fps
    }
}

impl Default for Fps {
    fn default() -> Self {
        Self::new()
    }
}
