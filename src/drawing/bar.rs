use std::cell::RefCell;

use cairo::Context;

use super::color::Color;
use super::Draw;
use data_helpers::{scale, shrink_by_averaging};

pub struct BarData {
    pub double_sided: bool,
    pub num_bars: usize,
    // draw channels seperately or average them into 1
    pub split_audio_channels: bool,
    pub max_bar_pieces_vertical: usize,
    pub bar_piece_width: f64,
    pub bar_piece_height: f64,
    pub bar_piece_horizontal_spacing: f64,
    pub bar_piece_vertical_spacing: f64,
    pub draw_color: Color,
    pub bg_color: Color,
    pub top_padding: f64,
    pub bottom_padding: f64,
    pub left_padding: f64,
    pub right_padding: f64,
    pub peak_heights: RefCell<Vec<(isize, f64)>>,
}

impl Default for BarData {
    fn default() -> Self {
        BarData {
            double_sided: false,
            num_bars: 30,
            split_audio_channels: false,
            max_bar_pieces_vertical: 70,
            bar_piece_width: 16.,
            bar_piece_height: 4.,
            bar_piece_horizontal_spacing: 1.,
            bar_piece_vertical_spacing: 1.,
            draw_color: Color::black(),
            bg_color: Color::default_bg(),
            top_padding: 10.,
            bottom_padding: 10.,
            left_padding: 10.,
            right_padding: 10.,
            peak_heights: RefCell::new(vec![(0, 0.); 70]),
        }
    }
}

impl Draw for BarData {
    fn draw(&self, context: &Context, data: &mut Vec<Vec<f64>>) {
        for buf in data.iter_mut() {
            scale_by_fft_max(buf);
        }

        context.save();
        let (_, totalheight) = self.draw_area();
        // draw the background
        call_rgba_fn!(context, set_source_rgba, self.bg_color);
        context.paint();

        call_rgba_fn!(context, set_source_rgba, self.draw_color);
        context.scale(1., -1.);
        context.translate(0., -totalheight);
        context.translate(self.left_padding, self.top_padding);
        let maxbarheight;
        if self.double_sided {
            maxbarheight = (totalheight - self.top_padding - self.bottom_padding) / 2.;
        } else {
            maxbarheight = 0.;
        }
        let total_bars;
        let mut combined;
        if self.split_audio_channels {
            for datavec in data.iter_mut() {
                shrink_by_averaging(datavec, self.num_bars);
            }
            combined = Vec::with_capacity(self.num_bars * 2);
            combined.extend(data[0].iter());
            combined.extend(data[1].iter().rev());
            total_bars = self.num_bars * 2;
        } else {
            combined = Vec::with_capacity(self.num_bars);
            for i in 0..data[0].len() {
                let mut average = 0.;
                for datavec in data.iter() {
                    average += datavec[i];
                }
                average /= data.len() as f64;
                combined.push(average);
            }
            shrink_by_averaging(&mut combined, self.num_bars);
            total_bars = self.num_bars;
        }

        let draw_half: Box<Fn()> = Box::new(move || {
            for bar in 0..total_bars {
                let chunks = f64::min(self.max_bar_pieces_vertical as f64,
                                      combined[bar] *
                                      (self.max_bar_pieces_vertical as f64 -
                                       1.)) as usize;
                let peak_height;
                {
                    let ref mut peaks = self.peak_heights.borrow_mut();
                    peaks[bar].1 += 0.35;
                    let fallen = peaks[bar].0 as f64 - (0.5 * peaks[bar].1.powi(2));

                    if chunks as f64 > fallen {
                        peaks[bar].0 = chunks as isize;
                        peaks[bar].1 = 0.;
                    }

                    if fallen > 0. {
                        peak_height = fallen;
                    } else {
                        peak_height = 0.;
                    }
                }

                let x = bar as f64 * (self.bar_piece_width + self.bar_piece_horizontal_spacing);
                for i in 0..chunks {
                    context.rectangle(x,
                                      maxbarheight +
                                      i as f64 *
                                      (self.bar_piece_height + self.bar_piece_vertical_spacing),
                                      self.bar_piece_width,
                                      self.bar_piece_height);
                }

                context.rectangle(x,
                                  maxbarheight +
                                  peak_height *
                                  (self.bar_piece_height + self.bar_piece_vertical_spacing),
                                  self.bar_piece_width,
                                  self.bar_piece_height);
                context.fill();
            }
        });

        draw_half();
        context.restore();
        if self.double_sided {
            context.save();
            context.translate(self.left_padding, self.top_padding);
            draw_half();
            context.restore();
        }
    }

    fn draw_area(&self) -> (f64, f64) {
        let vert_mult;
        if self.double_sided {
            vert_mult = 2.;
        } else {
            vert_mult = 1.;
        }
        let hor_mult;
        if self.split_audio_channels {
            hor_mult = 2.;
        } else {
            hor_mult = 1.;
        }
        (hor_mult *
         (self.bar_piece_width * self.num_bars as f64 +
          self.bar_piece_horizontal_spacing * (self.num_bars as f64 - 1.)) +
         self.right_padding + self.left_padding,
         vert_mult *
         (self.max_bar_pieces_vertical as f64 * self.bar_piece_height +
          self.bar_piece_vertical_spacing * (self.max_bar_pieces_vertical as f64 - 1.)) +
         self.top_padding + self.bottom_padding)
    }
}

fn scale_by_fft_max(items: &mut [f64]) {
    for i in 0..256 {
        let scaled = items[i] / FFT_MAX[i];
        if scaled > 1. {
            items[i] = 1.;
        } else {
            items[i] = scaled;
        }
    }
}

// Copied from impulse - how the hell does this work? idk.
const FFT_MAX: [f64; 256] =
    [12317168., 7693595., 5863615., 4082974., 5836037., 4550263., 3377914., 3085778., 3636534.,
     3751823., 2660548., 3313252., 2698853., 2186441., 1697466., 1960070., 1286950., 1252382.,
     1313726., 1140443., 1345589., 1269153., 897605., 900408., 892528., 587972., 662925., 668177.,
     686784., 656330., 1580286., 785491., 761213., 730185., 851753., 927848., 891221., 634291.,
     833909., 646617., 804409., 1015627., 671714., 813811., 689614., 727079., 853936., 819333.,
     679111., 730295., 836287., 1602396., 990827., 773609., 733606., 638993., 604530., 573002.,
     634570., 1015040., 679452., 672091., 880370., 1140558., 1593324., 686787., 781368., 605261.,
     1190262., 525205., 393080., 409546., 436431., 723744., 765299., 393927., 322105., 478074.,
     458596., 512763., 381303., 671156., 1177206., 476813., 366285., 436008., 361763., 252316.,
     204433., 291331., 296950., 329226., 319209., 258334., 388701., 543025., 396709., 296099.,
     190213., 167976., 138928., 116720., 163538., 331761., 133932., 187456., 530630., 131474.,
     84888., 82081., 122379., 82914., 75510., 62669., 73492., 68775., 57121., 94098., 68262.,
     68307., 48801., 46864., 61480., 46607., 45974., 45819., 45306., 45110., 45175., 44969.,
     44615., 44440., 44066., 43600., 57117., 43332., 59980., 55319., 54385., 81768., 51165.,
     54785., 73248., 52494., 57252., 61869., 65900., 75893., 65152., 108009., 421578., 152611.,
     135307., 254745., 132834., 169101., 137571., 141159., 142151., 211389., 267869., 367730.,
     256726., 185238., 251197., 204304., 284443., 258223., 158730., 228565., 375950., 294535.,
     288708., 351054., 694353., 477275., 270576., 426544., 362456., 441219., 313264., 300050.,
     421051., 414769., 244296., 292822., 262203., 418025., 579471., 418584., 419449., 405345.,
     739170., 488163., 376361., 339649., 313814., 430849., 275287., 382918., 297214., 286238.,
     367684., 303578., 516246., 654782., 353370., 417745., 392892., 418934., 475608., 284765.,
     260639., 288961., 301438., 301305., 329190., 252484., 272364., 261562., 208419., 203045.,
     229716., 191240., 328251., 267655., 322116., 509542., 498288., 341654., 346341., 451042.,
     452194., 467716., 447635., 644331., 1231811., 1181923., 1043922., 681166., 1078456.,
     1088757., 1221378., 1358397., 1817252., 1255182., 1410357., 2264454., 1880361., 1630934.,
     1147988., 1919954., 1624734., 1373554., 1865118., 2431931.];

