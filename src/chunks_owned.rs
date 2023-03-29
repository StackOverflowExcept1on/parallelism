use vecshard::{ShardExt, VecShard};

pub struct ChunksOwned<T> {
    v: Option<VecShard<T>>,
    chunk_size: usize,
}

impl<T> ChunksOwned<T> {
    #[inline]
    fn new(vec: VecShard<T>, size: usize) -> Self {
        Self { v: Some(vec), chunk_size: size }
    }
}

impl<T> Iterator for ChunksOwned<T> {
    type Item = VecShard<T>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let v = self.v.take()?;
        if v.len() == 0 {
            None
        } else {
            let chunksz = std::cmp::min(v.len(), self.chunk_size);
            let (fst, snd) = v.split_inplace_at(chunksz);
            self.v = Some(snd);
            Some(fst)
        }
    }
}

pub trait ChunksOwnedExt<T> {
    fn chunks_owned(self, chunk_size: usize) -> ChunksOwned<T>;
}

impl<T> ChunksOwnedExt<T> for VecShard<T> {
    fn chunks_owned(self, chunk_size: usize) -> ChunksOwned<T> {
        assert!(chunk_size != 0, "chunk size must be non-zero");
        ChunksOwned::new(self, chunk_size)
    }
}
