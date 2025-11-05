use sdl3::{
    AudioSubsystem,
    audio::{AudioCallback, AudioFormat, AudioSpec, AudioStreamWithCallback},
};

#[derive(Clone)]
struct Wave {
    pattern: [u8; 16],
    phase: f32,
    inc: f32,
    volume: u8,
}

impl AudioCallback<u8> for Wave {
    fn callback(&mut self, out: &mut [u8]) {
        for x in out.iter_mut() {
            let bit = self.phase as usize;
            if self.pattern[bit / 8] & (1 << (bit % 8)) > 0 {
                *x = self.volume;
            } else {
                // *x = -self.volume;
                *x = 0;
            }
            self.phase += self.inc;
            self.phase %= 128.0
        }
    }
}

pub struct Audio {
    device: AudioStreamWithCallback<Wave>,
    spec: AudioSpec,
    subsystem: AudioSubsystem,
    wave: Wave,
}

impl Audio {
    pub fn new() -> Self {
        let sdl_context = sdl3::init().unwrap();
        let spec = AudioSpec {
            freq: Some(32768),
            channels: Some(1),
            format: Some(AudioFormat::U8),
        };
        let wave = Wave {
            pattern: [0x0F; 16],
            phase: 0.0,
            inc: 4000.0 / 32768.0,
            volume: 255,
        };
        let subsystem = sdl_context.audio().unwrap();
        let device = subsystem.open_playback_stream(&spec, wave.clone()).unwrap();
        Audio {
            device,
            spec,
            subsystem,
            wave,
        }
    }
    pub fn pause(&self) {
        let _ = self.device.pause();
    }
    pub fn play(&self) {
        let _ = self.device.resume();
    }
    pub fn set_pattern(&mut self, pattern: &[u8; 16]) {
        self.wave.pattern = *pattern;
        self.device = self
            .subsystem
            .open_playback_stream(&self.spec, self.wave.clone())
            .unwrap();
    }
    pub fn set_pitch(&mut self, pitch: u16) {
        // println!("Changed pitch!");
        // self.spec.freq = Some((4000.0 * 2.0_f32.powf(((pitch as f32) - 64.0) / 48.0)) as i32);
        self.wave.inc = 4000.0 * 2.0_f32.powf(((pitch as f32) - 64.0) / 48.0) / 32768.0;
        self.wave.phase = 0.0;
        self.device = self
            .subsystem
            .open_playback_stream(&self.spec, self.wave.clone())
            .unwrap();
    }
}
