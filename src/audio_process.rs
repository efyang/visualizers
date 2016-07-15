use dft;
use dft::{Operation, Plan, unpack};
use pulse_simple::Record;

const SAMPLE_RATE: f64 = 44_100.0;
const CHANNELS: usize = 1;
const FRAMES: usize = 256;

// lazy_static initialize list of audio sources and their ids
// also intialize global mutexed id counter and dft plan
fn get_dft_plan() -> Plan {
    Plan::new(Operation::Forward, CHANNELS * FRAMES)
}

pub struct AudioProcessor {
    id: usize,
    source_index: usize,
    dft_plan: Plan,
    recorder: Record<[f32; 1]>,
    audio_buffer: [[f32; 1]; 256],
    secondary_buffer: Vec<f64>,
}

impl AudioProcessor {
    pub fn new(source_index: usize) -> AudioProcessor {
        // temporary
        AudioProcessor {
            id: 0,
            source_index: source_index,
            dft_plan: get_dft_plan(),
            recorder: Record::new("Test",
                                  "Record",
                                  Some("alsa_output.pci-0000_00_1b.0.analog-stereo.monitor"),
                                  44100),
            audio_buffer: [[0f32]; 256],
            secondary_buffer: Vec::with_capacity(FRAMES),
        }
    }

    // get partially processed data from 1 reading
    // raw audio data -> fourier transform -> absolute value of each complex value
    pub fn get_data_frame<'a>(&mut self) -> Vec<f64> {
        self.recorder.read(&mut self.audio_buffer);
        // cast to f64
        for (new, orig) in self.secondary_buffer.iter_mut().zip(self.audio_buffer.iter()) {
            *new = orig[0] as f64;
        }
        // perform fourier transform
        dft::transform(&mut self.secondary_buffer, &self.dft_plan);
        // unpack to complex and get absolute value of complex to convert to real
        dft::unpack(&self.secondary_buffer).iter().map(|ref c| c.norm()).collect::<Vec<_>>()
    }

    // NOTE: maybe instead of audio source index just switch by name and display description to
    // user?

    // switch the audio source index
    pub fn switch_source_index(&mut self, new_index: usize) -> Result<(), String> {
        unimplemented!()
    }

    // switch the audio source by name
    fn switch_source_by_name(&mut self, name: &str) -> Result<(), String> {
        unimplemented!()
    }

    // get the associated device name from index
    fn get_device_name_from_index<'a>(source_index: usize) -> Option<&'a str> {
        unimplemented!()
    }

    // get the default output device name
    fn get_default_output_device_name<'a>() -> Option<&'a str> {
        unimplemented!();
    }
}
