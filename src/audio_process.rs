use audio_devices::PaSourceInfo;
use dft;
use dft::{Operation, Plan};
use pa_simple::{Builder, Reader};

pub const FRAMES: usize = 256;

pub struct AudioProcessor {
    source_index: usize,
    channels: usize,
    rate: u32,
    dft_plan: Plan,
    recorder: Reader<i16>,
    audio_buffer: Vec<i16>,
    secondary_buffers: Vec<Vec<f64>>,
    previous: Vec<Vec<f64>>,
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
                    dft_plan: Plan::new(Operation::Forward, FRAMES),
                    recorder: Builder::new("visualizers", "visualizers")
                        .channels(source.channels)
                        .rate(source.rate)
                        .device(&source.name)
                        .reader_i16(),
                        audio_buffer: vec![0; FRAMES * source.channels as usize],
                        secondary_buffers: vec![vec![0f64; FRAMES]; source.channels as usize],
                        previous: vec![vec![0f64; FRAMES]; source.channels as usize],
                })
            }
            None => None,
        }

    }

    // NOTE: change this to allow an input of &mut [Vec<f64>] to prevent realloc?
    // get partially processed data from 1 reading
    // raw audio data -> fourier transform -> magnitude -> scale by impulse vec
    pub fn get_data_frame(&mut self) -> Vec<Vec<f64>> {
        self.recorder.read(self.audio_buffer.as_mut_slice());
        // cast to f64
        for frame_n in 0..FRAMES {
            let frame_idx = frame_n * self.channels;
            for channel_offset in 0..self.channels {
                let orig_idx = frame_idx + channel_offset;
                self.secondary_buffers[channel_offset][frame_n] = self.audio_buffer[orig_idx] as f64;
            }
        }
        let mut out_data = Vec::with_capacity(self.channels);
        for buf in self.secondary_buffers.iter_mut() {
            // perform fourier transform on each channel
            //println!("{:?}", buf);
            dft::transform(buf.as_mut_slice(), &self.dft_plan);
            //println!("{:?}", buf);
            // unpack
            out_data.push(dft::unpack(buf).iter().map(|ref c| c.norm_sqr().sqrt()).collect::<Vec<_>>());
        }

        out_data
    }

    pub fn channels(&self) -> usize {self.channels}

    pub fn source_index(&self) -> usize {self.source_index}
}

