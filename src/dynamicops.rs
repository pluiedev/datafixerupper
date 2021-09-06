use crate::mapencoder::RecordBuilder;

pub trait DynamicOps<T> {
    // TODO
    fn compress_maps(&self) -> bool;

    fn map_builder<R>(&self) -> R
    where
        R: RecordBuilder<T>;
}