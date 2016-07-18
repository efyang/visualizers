use audio_devices::{get_devices, PaSourceInfo};
use dft;
use dft::{Operation, Plan};
use pa_simple::{Builder, Reader};

const FRAMES: usize = 256;

pub struct AudioProcessor {
    source_index: usize,
    channels: usize,
    rate: u32,
    dft_plan: Plan,
    recorder: Reader<f32>,
    audio_buffer: Vec<f32>,
    secondary_buffers: Vec<Vec<f64>>,
}

impl AudioProcessor {
    pub fn new(sources: &[Option<PaSourceInfo>], source_index: usize) -> Option<AudioProcessor> {
        // temporary
        match sources[source_index] {
            Some(ref source) => {
                Some(AudioProcessor {
                    source_index: source_index,
                    channels: source.channels as usize,
                    rate: source.rate,
                    dft_plan: Plan::new(Operation::Forward, source.channels as usize * FRAMES),
                    recorder: Builder::new("Test", "Record")
                        .channels(source.channels)
                        .rate(source.rate)
                        .device(&source.name)
                        .reader_f32(),
                        audio_buffer: vec![0f32; FRAMES * source.channels as usize],
                        secondary_buffers: vec![Vec::with_capacity(FRAMES * source.channels as usize); source.channels as usize],
                })
            }
            None => None,
        }

    }

    // NOTE: change this to allow an input of &mut [Vec<f64>] to prevent realloc?
    // get partially processed data from 1 reading
    // raw audio data -> fourier transform -> absolute value of each complex value
    pub fn get_data_frame(&mut self) -> Vec<Vec<f64>> {
        self.recorder.read(&mut self.audio_buffer);
        // cast to f64
        for frame_n in 0..FRAMES {
            let frame_idx = frame_n * self.channels;
            for channel_offset in 0..self.channels {
                let orig_idx = frame_idx + channel_offset;
                for buf in self.secondary_buffers.iter_mut() {
                    buf[frame_n] = self.audio_buffer[orig_idx] as f64
                }
            }
        }
        let mut out_data = Vec::with_capacity(self.channels);
        for buf in self.secondary_buffers.iter_mut() {
            // perform fourier transform on each channel
            dft::transform(buf.as_mut_slice(), &self.dft_plan);
            // unpack
            out_data.push(dft::unpack(buf).iter().map(|ref c| c.norm()).collect::<Vec<_>>());
        }

        out_data
    }

    // NOTE: maybe instead of audio source index just switch by name and display description to
    // user?

    // switch the audio source index
    pub fn switch_to_by_index(&mut self, new_index: usize) -> Result<(), String> {
        unimplemented!()
    }

    // switch the audio source by name
    fn switch_source_by_name(&mut self, name: &str) -> Result<(), String> {
        unimplemented!()
    }
}
