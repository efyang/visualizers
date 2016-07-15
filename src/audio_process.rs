use dft;
use dft::{Operation, Plan, unpack};
use pulse_simple::Record;

const SAMPLE_RATE: f64 = 44_100.0;
const CHANNELS: usize = 1;
const FRAMES: usize = 256;

// lazy_static initialize list of audio sources and their ids
// also intialize global mutexed id counter

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
            recorder: Record::new("Test", "Record", Some("alsa_output.pci-0000_00_1b.0.analog-stereo.monitor"), 44100),
            audio_buffer: [[0f32]; 256],
            secondary_buffer: Vec::with_capacity(FRAMES),
        }
    }

    pub fn get_data_frame<'a>(&mut self) -> Vec<f64> {
        self.recorder.read(&mut self.audio_buffer);
        for (new, orig) in self.secondary_buffer.iter_mut().zip(self.audio_buffer.iter()) {
            *new  = orig[0] as f64;
        }
        dft::transform(&mut self.secondary_buffer, &self.dft_plan);
        dft::unpack(&self.secondary_buffer).iter().map(|ref c| c.norm()).collect::<Vec<_>>()
    }

    pub fn switch_source(&mut self) -> Result<(), String> {
        unimplemented!()
    }
}

fn get_dft_plan() -> Plan {
    Plan::new(Operation::Forward, CHANNELS * FRAMES)
}

// shrink vector down to wanted size n by averaging parts
// before shrink
// [0.,1.,2.,3.,4.,5.,6.,7.,8.,9.,10.]
// after shrink
// [1, 4, 7, 9.5]
// use to make dft'd data viewable in n bars
fn shrink(items: &mut Vec<f64>, n: usize) {
    assert!(n != 0);

    fn average(items: &[f64]) -> f64 {
        items.iter().fold(0., |acc, &x| acc + x) / items.len() as f64
    }
    
    // most of the elements
    let a = items.len()/n;
    // any remaining elements
    let b = items.len() % n;
    for i in 0..a {
        items[i] = average(&items[i*n..(i+1)*n]);
    }
    if b != 0 {
        items[a] = average(&items[a*n..items.len()]);
    }
    //&items[0..n+1] <- not in-place
    items.split_off(n+1);
}

fn get_default_output_device_name<'a>() -> Option<&'a str> {
    unimplemented!();
}
