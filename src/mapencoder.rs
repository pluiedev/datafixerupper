use crate::encoder::Encoder;
use crate::{dataresult::DataResult, dynamicops::DynamicOps};
use crate::{Keys, Lifecycle};

trait MapEncoder<A>: Keys + Compressable {
    fn encode<T, R>(&self, input: A, ops: impl DynamicOps<T>, prefix: R) -> R
    where
        R: RecordBuilder<T>;

    fn compressed_builder<T, R>(&self, ops: impl DynamicOps<T>) -> R
    where
        R: RecordBuilder<T>,
    {
        if ops.compress_maps() {
            make_compressed_builder(ops, self.compressor(ops))
        } else {
            ops.map_builder()
        }
    }

    fn comap<B, F>(&self, function: F) -> Comap<Self, F>
    where
        F: Fn(B) -> A,
    {
        Comap {
            encoder: self,
            function,
        }
    }

    fn flat_comap<B, F>(&self, function: F) -> FlatComap<Self, F>
    where
        F: Fn(B) -> DataResult<A>,
    {
        FlatComap {
            encoder: self,
            function,
        }
    }

    /*
    <T> KeyCompressor<T> compressor(final DynamicOps<T> ops);


    default Encoder<A> encoder() {
        return new Encoder<A>() {
            @Override
            public <T> DataResult<T> encode(final A input, final DynamicOps<T> ops, final T prefix) {
                return MapEncoder.this.encode(input, ops, compressedBuilder(ops)).build(prefix);
            }

            @Override
            public String toString() {
                return MapEncoder.this.toString();
            }
        };
    }
=

    abstract class Implementation<A> extends CompressorHolder implements MapEncoder<A> {
    }

    static <T> RecordBuilder<T> makeCompressedBuilder(final DynamicOps<T> ops, final KeyCompressor<T> compressor) {
        class CompressedRecordBuilder extends RecordBuilder.AbstractUniversalBuilder<T, List<T>> {
            private CompressedRecordBuilder() {
                super(ops);
            }

            @Override
            protected List<T> initBuilder() {
                final List<T> list = new ArrayList<>(compressor.size());
                for (int i = 0; i < compressor.size(); i++) {
                    list.add(null);
                }
                return list;
            }

            @Override
            protected List<T> append(final T key, final T value, final List<T> builder) {
                builder.set(compressor.compress(key), value);
                return builder;
            }

            @Override
            protected DataResult<T> build(final List<T> builder, final T prefix) {
                return ops().mergeToList(prefix, builder);
            }
        }

        return new CompressedRecordBuilder();
    }

    */
}



struct Comap<E, F> {
    encoder: E,
    function: F,
}

impl<E, F, A, B> MapEncoder<B> for Comap<E, F>
where
    E: MapEncoder<A>,
    F: Fn(B) -> A,
{
    fn encode<T, R>(&self, input: A, ops: impl DynamicOps<T>, prefix: R) -> R
    where
        R: RecordBuilder<T>,
    {
        self.encoder.encode(self.function(input), ops, prefix)
    }
}

impl<E, F> Keys for Comap<E, F> {
    fn keys<T, I>(&self, ops: impl DynamicOps<T>) -> I
    where
        I: Iterator<Item = T>,
    {
        self.encoder.keys()
    }
}

struct FlatComap<E, F> {
    encoder: E,
    function: F,
}

impl<E, F, A, B> MapEncoder<B> for FlatComap<E, F>
where
    E: MapEncoder<A>,
    F: Fn(B) -> DataResult<A>,
{
    fn encode<T, R>(&self, input: A, ops: impl DynamicOps<T>, prefix: R) -> R
    where
        R: RecordBuilder<T>,
    {
        let result = (self.function)(input);
        let builder = prefix.with_errors_from(result);
        result
            .map(|r| self.encoder.encode(r, ops, prefix))
            .result()
            .or_else(builder)
    }
}

impl<E, F> Keys for FlatComap<E, F> {
    fn keys<T, I>(&self, ops: impl DynamicOps<T>) -> I
    where
        I: Iterator<Item = T>,
    {
        self.encoder.keys()
    }
}

struct WithLifecycle<E> {
    encoder: E,
    lifecycle: Lifecycle,
}

impl<E, A> Encoder<A> for WithLifecycle<E>
where
    E: Encoder<A>,
{
    fn encode<T>(&self, input: A, ops: impl DynamicOps<T>, prefix: T) -> DataResult<T> {
        self.encoder
            .encode(input, ops, prefix)
            .set_lifecycle(self.lifecycle)
    }
}

impl<E> Keys for WithLifecycle<E> {
    fn keys<T, I>(&self, ops: impl DynamicOps<T>) -> I
    where
        I: Iterator<Item = T> {
        self.encoder.keys()
    }
}

pub trait RecordBuilder<T> {
    // TODO
    fn with_errors_from(self, result: DataResult<_>) -> Self;
}
