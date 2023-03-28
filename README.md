### parallelism

[![Build Status](https://github.com/StackOverflowExcept1on/parallelism/actions/workflows/ci.yml/badge.svg)](https://github.com/StackOverflowExcept1on/parallelism/actions/workflows/ci.yml)

```
Vec<T> = (0..=10_000_000).map(|x| x as f64)
// *simulate some expensive work with trigonometric functions*
Fn(T) -> R = |x: f64| x.sqrt().cos().sin().asin().acos().powf(f64::EPSILON);
```

```
running functions with CPUs = 6
compute1 (single thread):
    790.77ms
compute2 (std::thread with MaybeUninit):
    173.54ms
compute3 (rayon parallel iter):
    166.17ms
```
