use std::num::NonZeroUsize;
use std::thread;

use rayon::prelude::*;

#[inline]
pub fn num_cpus() -> usize {
    thread::available_parallelism()
        .unwrap_or(NonZeroUsize::MIN) //NonZeroUsize::MIN = 1, i.e. fallback to single thread
        .get()
}

#[inline]
pub fn compute1<T, R, F: Fn(T) -> R>(input: Vec<T>, func: F) -> Vec<R> {
    input.into_iter().map(|x| func(x)).collect()
}

#[inline]
pub fn compute2<
    T: Clone + Send + Sync + 'static,
    F: Fn(T) -> R + Clone + Send + Sync + 'static,
    R: Clone + Send + Sync + 'static,
>(
    input: Vec<T>,
    func: F,
    threshold: usize,
) -> Vec<R> {
    if input.len() <= threshold {
        compute1(input, func)
    } else {
        let mut result: Vec<R> = Vec::with_capacity(input.len());

        let chunk_size = input.len() / num_cpus();
        thread::scope(|scope| {
            //scoped thread would modify stack of this thread without Mutex
            for (original, changed) in input
                .chunks(chunk_size)
                .zip(result.spare_capacity_mut().chunks_mut(chunk_size))
            {
                let f = func.clone();
                scope.spawn(move || {
                    for (t, r) in original.iter().cloned().zip(changed.iter_mut()) {
                        r.write(f(t));
                    }
                });
            }
        });

        //SAFETY: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.spare_capacity_mut
        unsafe { result.set_len(result.capacity()) };
        result
    }
}

#[inline]
pub fn compute3<
    T: Clone + Send + Sync + 'static,
    F: Fn(T) -> R + Clone + Send + Sync + 'static,
    R: Clone + Send + Sync + 'static,
>(
    input: Vec<T>,
    func: F,
    threshold: usize,
) -> Vec<R> {
    if input.len() <= threshold {
        compute1(input, func)
    } else {
        input.into_par_iter().map(|x| func(x)).collect()
    }
}
