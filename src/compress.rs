trait Compressable: Keys {
    fn compressor<T>(ops: impl DynamicOps<T>) -> KeyCompressor<T>;
}

struct KeyCompressor<T, O> {
    decompress: HashMap<i32, T>,
    compress: HashMap<T, i32>,
    compress_string: HashMap<String, i32>,

    size: i32,
    ops: O,
}

impl KeyCompressor