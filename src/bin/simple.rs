use parallelism::*;
use std::time::Instant;

#[inline]
fn execute<R, F: FnOnce() -> R>(label: &str, func: F) -> R {
    let start_time = Instant::now();
    let ret = func();
    let end_time = start_time.elapsed();
    println!("{label}:");
    println!("    {end_time:.2?}");
    return ret;
}

fn main() {
    println!("running functions with CPUs = {count}", count = num_cpus());

    let input = (0..=10_000_000).map(|x| x as f64).collect::<Vec<_>>();
    let func = |x: f64| x.sqrt().cos().sin().asin().acos().powf(f64::EPSILON);

    //don't include Vec::clone() into benchmarks

    let input1 = input.clone();
    let output1 = execute("compute1 (single thread)", || compute1(input1, func));

    let input2 = input.clone();
    let output2 = execute("compute2 (std::thread with MaybeUninit)", || compute2(input2, func, 2_500_000));

    let input3 = input.clone();
    let output3 = execute("compute3 (rayon parallel iter)", || compute3(input3, func, 2_500_000));

    assert_eq!(output1, output2);
    assert_eq!(output2, output3);
}
