use std::collections::HashMap;

use crate::{Keys, dynamicops::DynamicOps};

trait Compressable: Keys {
    fn compressor<T>(ops: impl DynamicOps<T>) -> KeyCompressor<T>;
}

pub struct KeyCompressor<T, O> {
    decompress: HashMap<usize, T>,
    compress: HashMap<T, usize>,
    compress_string: HashMap<String, usize>,

    size: usize,
    ops: O,
}

impl<T, O> KeyCompressor<T, O>
where
    O: DynamicOps<T>,
{
    pub fn new(ops: O, key_stream: impl Iterator<Item = T>) -> Self {
        let compress = HashMap::new();
        let decompress = HashMap::new();
        let compress_string = HashMap::new();

        // not transferrable to Rust
        //compressString.defaultReturnValue(-1)

        for key in key_stream {
            if compress.contains_key(&key) {
                return;
            }
            let next = compress.keys().len();
            compress.entry(&key).insert(next);

            if let Some(k) = ops.string_value(&key).ok() {
                compress_string.entry(k).insert(next);
            }
            decompress.put(next, key);
        }

        let size = compress.keys().len();

        Self {
            compress,
            decompress,
            compress_string,
            size,
            ops,
        }
    }

    fn decompress(&self, key: usize) -> Option<&T> { self.decompress.get(&key) }

    fn compress_string(&self, key: &str) -> usize {
        self.compress_string.get(key).or_else(||
            self.compress(&self.ops.create_string(key))
        )
    }

    fn compress(&self, key: &T) -> usize { self.compress.get(&key) }

    fn size(&self) -> usize { self.size }
}
