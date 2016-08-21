// shrink vector down to wanted size n by averaging parts
// before shrink
// [0.,1.,2.,3.,4.,5.,6.,7.,8.,9.,10.]
// after shrink to 4
// [1, 4, 7, 9.5]
// use to make dft'd data viewable in n bars
pub fn shrink_by_averaging(items: &mut Vec<f64>, n: usize) {
    assert!(n != 0);
    assert!(items.len() > n);

    fn average(items: &[f64]) -> f64 {
        items.iter().fold(0., |acc, &x| acc + x) / items.len() as f64
    }

    let q = items.len() as f64 / n as f64;
    for i in 0..n {
        let start = (q * i as f64).round() as usize;
        let end = (q * (i + 1) as f64).round() as usize;
        items[i] = average(&items[start..end]);
    }
    items.split_off(n);
}

pub fn expand_by_clone(items: &mut Vec<f64>, n: usize) -> Vec<f64> {
    assert!(n != 0);
    assert!(items.len() < n);

    let clone_times = n / items.len();
    let mut extra_clone_times = n % items.len();
    let extra_clone_gap;
    if extra_clone_times == 0 {
        extra_clone_gap = 0;
    } else {
        extra_clone_gap = items.len() / extra_clone_times;
    }
    let mut newvec = Vec::with_capacity(n);
    let mut curidx = 0;
    for item in items.into_iter().map(|f| *f) {
        for _ in 0..clone_times {
            newvec.push(item);
        }
        if extra_clone_times > 0 && curidx % extra_clone_gap == 0 {
            newvec.push(item);
            extra_clone_times -= 1;
        }
        curidx += 1;
    }
    newvec
}

pub fn scale_to_maximum(items: &mut Vec<f64>, maximum: f64) {
    let minimum = items.iter().cloned().fold(0. / 0., f64::min);
    if maximum != 0. && maximum != ::std::f64::NEG_INFINITY {
        for item in items.iter_mut() {
            //*item = item.log(maximum);
            //*item = (*item - minimum) / (maximum - minimum);
            *item = *item / maximum;
        }
    }
}

pub fn map_multiply(items: &mut Vec<f64>, n: f64) {
    for item in items.iter_mut() {
        *item *= n;
    }
}

#[test]
fn test_shrink_by_averaging_1() {
    let mut data = vec![0., 1., 2., 3., 4., 5., 6., 7., 8.];
    shrink_by_averaging(&mut data, 3);
    assert_eq!(data.as_slice(), &[1., 4., 7.]);
}

#[test]
fn test_shrink_by_averaging_2() {
    let mut data = vec![0., 1., 2., 3., 4., 5., 6., 7., 8., 9., 10.];
    shrink_by_averaging(&mut data, 3);
    assert_eq!(data.as_slice(), &[1.5, 5., 8.5]);
}

#[test]
fn test_shrink_by_averaging_3() {
    let mut data = vec![0., 1., 2., 3., 4., 5., 6., 7., 8., 9., 10.];
    shrink_by_averaging(&mut data, 2);
    assert_eq!(data.as_slice(), &[2.5, 8.]);
}

#[test]
fn test_shrink_by_averaging_4() {
    let mut data = vec![0., 1., 2., 3., 4., 5., 6., 7.];
    shrink_by_averaging(&mut data, 2);
    assert_eq!(data.as_slice(), &[1.5, 5.5]);
}

#[test]
fn test_shrink_by_averaging_5() {
    let mut data = (0..256).map(|f| f as f64).collect();
    shrink_by_averaging(&mut data, 30);
    assert_eq!(data[data.len() - 1], 251.);
}

#[test]
fn test_expand_by_clone_1() {
    let mut data = vec![0., 1., 2.];
    data = expand_by_clone(&mut data, 5);
    assert_eq!(data.as_slice(), &[0., 0., 1., 1., 2.]);
}

#[test]
fn test_expand_by_clone_2() {
    let mut data = vec![0., 1., 2.];
    data = expand_by_clone(&mut data, 6);
    assert_eq!(data.as_slice(), &[0., 0., 1., 1., 2., 2.]);
}

pub fn scale(items: &mut Vec<f64>) {
    // https://www.reddit.com/r/rust/comments/3fg0xr/how_do_i_find_the_max_value_in_a_vecf64/
    let maximum = items.iter().cloned().fold(0. / 0., f64::max);
    scale_to_maximum(items, maximum);
}

#[test]
fn test_scale_1() {
    let mut data = vec![0., 1., 2.];
    scale(&mut data);
    assert_eq!(data.as_slice(), &[0., 0.5, 1.]);
}

#[test]
fn test_scale_2() {
    let mut data = vec![2., 2., 2.];
    scale(&mut data);
    assert_eq!(data.as_slice(), &[1., 1., 1.]);
}

#[test]
fn test_scale_3() {
    let mut data = vec![0., 0., 0.];
    scale(&mut data);
    assert_eq!(data.as_slice(), &[0., 0., 0.]);
}

#[test]
fn test_map_multiply() {
    let mut data = vec![1., 2., 3.];
    map_multiply(&mut data, 2.);
    assert_eq!(data.as_slice(), &[2., 4., 6.]);
}
