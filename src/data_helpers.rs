// shrink vector down to wanted size n by averaging parts
// before shrink
// [0.,1.,2.,3.,4.,5.,6.,7.,8.,9.,10.]
// after shrink to 4
// [1, 4, 7, 9.5]
// use to make dft'd data viewable in n bars
pub fn average_shrink(items: &mut Vec<f64>, n: usize) {
    assert!(n != 0);

    fn average(items: &[f64]) -> f64 {
        items.iter().fold(0., |acc, &x| acc + x) / items.len() as f64
    }

    let (a, b);
    if items.len() % n != 0 {
        b = items.len() / n + 1;
        a = items.len() / b;
    } else {
        b = items.len() / n;
        a = n;
    }
    for i in 0..a {
        items[i] = average(&items[i * b..(i + 1) * b]);
    }
    if a * b != items.len() {
        items[a] = average(&items[a * b..items.len()]);
    }
    items.split_off(n);
}

#[test]
fn test_average_shrink_1() {
    let mut data = vec![0., 1., 2., 3., 4., 5., 6., 7., 8.];
    average_shrink(&mut data, 3);
    assert_eq!(data.as_slice(), &[1., 4., 7.]);
}

#[test]
fn test_average_shrink_2() {
    let mut data = vec![0., 1., 2., 3., 4., 5., 6., 7., 8., 9., 10.];
    average_shrink(&mut data, 3);
    assert_eq!(data.as_slice(), &[1.5, 5.5, 9.]);
}

#[test]
fn test_average_shrink_3() {
    let mut data = vec![0., 1., 2., 3., 4., 5., 6., 7., 8., 9., 10.];
    average_shrink(&mut data, 2);
    assert_eq!(data.as_slice(), &[2.5, 8.]);   
}

#[test]
fn test_average_shrink_4() {
    let mut data = vec![0., 1., 2., 3., 4., 5., 6., 7.];
    average_shrink(&mut data, 2);
    assert_eq!(data.as_slice(), &[1.5, 5.5]);   
}
