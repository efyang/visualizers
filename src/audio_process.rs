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
        for buf in out_data.iter_mut() {
            scale_by_fft_max(buf);
        }

        out_data
    }

    pub fn channels(&self) -> usize {self.channels}

    pub fn source_index(&self) -> usize {self.source_index}
}

fn scale_by_fft_max(items: &mut [f64]) {
    for i in 0..FRAMES {
        let scaled = items[i] / FFT_MAX[i];
        if scaled > 1. {
            items[i] = 1.;
        } else {
            items[i] = scaled;
        }
    }
}

// Copied from impulse - how the hell does this work? idk.
const FFT_MAX: [f64; 256] = [ 12317168., 7693595., 5863615., 4082974., 5836037., 4550263., 3377914., 3085778., 3636534., 3751823., 2660548., 3313252., 2698853., 2186441., 1697466., 1960070., 1286950., 1252382., 1313726., 1140443., 1345589., 1269153., 897605., 900408., 892528., 587972., 662925., 668177., 686784., 656330., 1580286., 785491., 761213., 730185., 851753., 927848., 891221., 634291., 833909., 646617., 804409., 1015627., 671714., 813811., 689614., 727079., 853936., 819333., 679111., 730295., 836287., 1602396., 990827., 773609., 733606., 638993., 604530., 573002., 634570., 1015040., 679452., 672091., 880370., 1140558., 1593324., 686787., 781368., 605261., 1190262., 525205., 393080., 409546., 436431., 723744., 765299., 393927., 322105., 478074., 458596., 512763., 381303., 671156., 1177206., 476813., 366285., 436008., 361763., 252316., 204433., 291331., 296950., 329226., 319209., 258334., 388701., 543025., 396709., 296099., 190213., 167976., 138928., 116720., 163538., 331761., 133932., 187456., 530630., 131474., 84888., 82081., 122379., 82914., 75510., 62669., 73492., 68775., 57121., 94098., 68262., 68307., 48801., 46864., 61480., 46607., 45974., 45819., 45306., 45110., 45175., 44969., 44615., 44440., 44066., 43600., 57117., 43332., 59980., 55319., 54385., 81768., 51165., 54785., 73248., 52494., 57252., 61869., 65900., 75893., 65152., 108009., 421578., 152611., 135307., 254745., 132834., 169101., 137571., 141159., 142151., 211389., 267869., 367730., 256726., 185238., 251197., 204304., 284443., 258223., 158730., 228565., 375950., 294535., 288708., 351054., 694353., 477275., 270576., 426544., 362456., 441219., 313264., 300050., 421051., 414769., 244296., 292822., 262203., 418025., 579471., 418584., 419449., 405345., 739170., 488163., 376361., 339649., 313814., 430849., 275287., 382918., 297214., 286238., 367684., 303578., 516246., 654782., 353370., 417745., 392892., 418934., 475608., 284765., 260639., 288961., 301438., 301305., 329190., 252484., 272364., 261562., 208419., 203045., 229716., 191240., 328251., 267655., 322116., 509542., 498288., 341654., 346341., 451042., 452194., 467716., 447635., 644331., 1231811., 1181923., 1043922., 681166., 1078456., 1088757., 1221378., 1358397., 1817252., 1255182., 1410357., 2264454., 1880361., 1630934., 1147988., 1919954., 1624734., 1373554., 1865118., 2431931. ];
