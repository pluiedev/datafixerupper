use crate::dataresult::DataResult;
use crate::dynamicops::DynamicOps;
use crate::Keys;
use crate::Lifecycle;

pub trait Encoder<A> {
    fn encode<T>(&self, input: A, ops: impl DynamicOps<T>, prefix: T) -> DataResult<T>;

    fn encode_start<T>(&self, ops: impl DynamicOps<T>, input: A) -> DataResult<T> {
        self.encode(input, ops, ops.empty())
    }

    fn field_of(&self, name: &str) -> FieldEncoder<A, Self>
    where
        Self: Sized,
    {
        FieldEncoder {
            name,
            encoder: self,
        };
    }

    fn comap<B, F>(&self, function: F) -> Comap<Self, F>
    where
        F: Fn(B) -> A,
        Self: Sized,
    {
        Comap {
            encoder: self,
            function,
        }
    }

    fn flat_comap<B, F>(&self, function: F) -> FlatComap<B>
    where
        F: Fn(B) -> DataResult<A>,
    {
        FlatComap {}
    }

    fn with_lifecycle(&self, lifecycle: Lifecycle) -> WithLifecycle<A> {
        WithLifecycle {
            encoder: self,
            lifecycle,
        }
    }
}

fn empty<A>() -> Empty<A> {
    Empty {}
}

fn error<A>(error: impl AsRef<str>) -> Error<A> {
    Error {
        error: error.into(),
    }
}

struct Comap<E, F> {
    encoder: E,
    function: F,
}

impl<E, F, A, B> Encoder<B> for Comap<E, F>
where
    E: Encoder<A>,
    F: Fn(B) -> A,
{
    fn encode<T>(&self, input: B, ops: impl DynamicOps<T>, prefix: T) -> DataResult<T> {
        self.encoder.encode(self.function(input), ops, prefix)
    }
}

struct FlatComap<E, F> {
    encoder: E,
    function: F,
}

impl<E, F, A, B> Encoder<B> for FlatComap<E, F>
where
    E: Encoder<A>,
    F: Fn(B) -> DataResult<A>,
{
    fn encode<T>(&self, input: B, ops: impl DynamicOps<T>, prefix: T) -> DataResult<T> {
        let result = (self.function)(input);
        result.flat_map(|a| self.encoder.encode(a, ops, prefix))
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

struct Empty;

impl<A> Encoder<A> for Empty {
    fn encode<T>(&self, input: A, ops: impl DynamicOps<T>, prefix: T) -> DataResult<T> {
        prefix
    }
}

impl Keys for Empty {
    fn keys<T, I>(&self, ops: impl DynamicOps<T>) -> I
    where
        I: Iterator<Item = T>,
    {
        std::iter::empty()
    }
}

struct Error {
    error: String,
}

impl<A> Encoder<A> for Error {
    fn encode<T>(&self, input: A, ops: impl DynamicOps<T>, prefix: T) -> DataResult<T> {
        // TODO: can we not...
        DataResult::error(format!("{} {}", self.error, input), Lifecycle::Experimental)
    }
}

struct FieldEncoder<'a, A, E> {
    name: &'a str,
    encoder: E,
}
