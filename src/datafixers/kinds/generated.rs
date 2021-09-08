#![allow(dead_code)]
#![allow(unused_macros)]
//! This is an auto-generated file.
//! Edits would be invalidated after a refresh.
    
/// Represents a **type constructor** that takes 1 arguments to construct a type.
pub trait Kind1 {
    type This<T1>;
}

#[macro_export]
macro_rules! kind1 {
    ($t:ident) => {
        impl<_T1> $crate::datafixers::kinds::generated::Kind1 for $t<_T1> {
            type This<T1> = $t<T1>;
        }
    };
}
            
/// Represents a **type constructor** that takes 2 arguments to construct a type.
pub trait Kind2 {
    type This<T1, T2>;
}

#[macro_export]
macro_rules! kind2 {
    ($t:ident) => {
        impl<_T1, _T2> $crate::datafixers::kinds::generated::Kind2 for $t<_T1, _T2> {
            type This<T1, T2> = $t<T1, T2>;
        }
    };
}
            
/// Represents a **type constructor** that takes 3 arguments to construct a type.
pub trait Kind3 {
    type This<T1, T2, T3>;
}

#[macro_export]
macro_rules! kind3 {
    ($t:ident) => {
        impl<_T1, _T2, _T3> $crate::datafixers::kinds::generated::Kind3 for $t<_T1, _T2, _T3> {
            type This<T1, T2, T3> = $t<T1, T2, T3>;
        }
    };
}
            
/// Represents a **type constructor** that takes 4 arguments to construct a type.
pub trait Kind4 {
    type This<T1, T2, T3, T4>;
}

#[macro_export]
macro_rules! kind4 {
    ($t:ident) => {
        impl<_T1, _T2, _T3, _T4> $crate::datafixers::kinds::generated::Kind4 for $t<_T1, _T2, _T3, _T4> {
            type This<T1, T2, T3, T4> = $t<T1, T2, T3, T4>;
        }
    };
}
            
/// Represents a **type constructor** that takes 5 arguments to construct a type.
pub trait Kind5 {
    type This<T1, T2, T3, T4, T5>;
}

#[macro_export]
macro_rules! kind5 {
    ($t:ident) => {
        impl<_T1, _T2, _T3, _T4, _T5> $crate::datafixers::kinds::generated::Kind5 for $t<_T1, _T2, _T3, _T4, _T5> {
            type This<T1, T2, T3, T4, T5> = $t<T1, T2, T3, T4, T5>;
        }
    };
}
            
/// Represents a **type constructor** that takes 6 arguments to construct a type.
pub trait Kind6 {
    type This<T1, T2, T3, T4, T5, T6>;
}

#[macro_export]
macro_rules! kind6 {
    ($t:ident) => {
        impl<_T1, _T2, _T3, _T4, _T5, _T6> $crate::datafixers::kinds::generated::Kind6 for $t<_T1, _T2, _T3, _T4, _T5, _T6> {
            type This<T1, T2, T3, T4, T5, T6> = $t<T1, T2, T3, T4, T5, T6>;
        }
    };
}
            
/// Represents a **type constructor** that takes 7 arguments to construct a type.
pub trait Kind7 {
    type This<T1, T2, T3, T4, T5, T6, T7>;
}

#[macro_export]
macro_rules! kind7 {
    ($t:ident) => {
        impl<_T1, _T2, _T3, _T4, _T5, _T6, _T7> $crate::datafixers::kinds::generated::Kind7 for $t<_T1, _T2, _T3, _T4, _T5, _T6, _T7> {
            type This<T1, T2, T3, T4, T5, T6, T7> = $t<T1, T2, T3, T4, T5, T6, T7>;
        }
    };
}
            
/// Represents a **type constructor** that takes 8 arguments to construct a type.
pub trait Kind8 {
    type This<T1, T2, T3, T4, T5, T6, T7, T8>;
}

#[macro_export]
macro_rules! kind8 {
    ($t:ident) => {
        impl<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8> $crate::datafixers::kinds::generated::Kind8 for $t<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8> {
            type This<T1, T2, T3, T4, T5, T6, T7, T8> = $t<T1, T2, T3, T4, T5, T6, T7, T8>;
        }
    };
}
            
/// Represents a **type constructor** that takes 9 arguments to construct a type.
pub trait Kind9 {
    type This<T1, T2, T3, T4, T5, T6, T7, T8, T9>;
}

#[macro_export]
macro_rules! kind9 {
    ($t:ident) => {
        impl<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9> $crate::datafixers::kinds::generated::Kind9 for $t<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9> {
            type This<T1, T2, T3, T4, T5, T6, T7, T8, T9> = $t<T1, T2, T3, T4, T5, T6, T7, T8, T9>;
        }
    };
}
            
/// Represents a **type constructor** that takes 10 arguments to construct a type.
pub trait Kind10 {
    type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>;
}

#[macro_export]
macro_rules! kind10 {
    ($t:ident) => {
        impl<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10> $crate::datafixers::kinds::generated::Kind10 for $t<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10> {
            type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> = $t<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>;
        }
    };
}
            
/// Represents a **type constructor** that takes 11 arguments to construct a type.
pub trait Kind11 {
    type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11>;
}

#[macro_export]
macro_rules! kind11 {
    ($t:ident) => {
        impl<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11> $crate::datafixers::kinds::generated::Kind11 for $t<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11> {
            type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> = $t<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11>;
        }
    };
}
            
/// Represents a **type constructor** that takes 12 arguments to construct a type.
pub trait Kind12 {
    type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12>;
}

#[macro_export]
macro_rules! kind12 {
    ($t:ident) => {
        impl<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12> $crate::datafixers::kinds::generated::Kind12 for $t<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12> {
            type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> = $t<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12>;
        }
    };
}
            
/// Represents a **type constructor** that takes 13 arguments to construct a type.
pub trait Kind13 {
    type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13>;
}

#[macro_export]
macro_rules! kind13 {
    ($t:ident) => {
        impl<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13> $crate::datafixers::kinds::generated::Kind13 for $t<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13> {
            type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> = $t<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13>;
        }
    };
}
            
/// Represents a **type constructor** that takes 14 arguments to construct a type.
pub trait Kind14 {
    type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14>;
}

#[macro_export]
macro_rules! kind14 {
    ($t:ident) => {
        impl<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14> $crate::datafixers::kinds::generated::Kind14 for $t<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14> {
            type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> = $t<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14>;
        }
    };
}
            
/// Represents a **type constructor** that takes 15 arguments to construct a type.
pub trait Kind15 {
    type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15>;
}

#[macro_export]
macro_rules! kind15 {
    ($t:ident) => {
        impl<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15> $crate::datafixers::kinds::generated::Kind15 for $t<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15> {
            type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> = $t<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15>;
        }
    };
}
            
/// Represents a **type constructor** that takes 16 arguments to construct a type.
pub trait Kind16 {
    type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16>;
}

#[macro_export]
macro_rules! kind16 {
    ($t:ident) => {
        impl<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16> $crate::datafixers::kinds::generated::Kind16 for $t<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16> {
            type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> = $t<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16>;
        }
    };
}
            
/// Represents a **type constructor** that takes 17 arguments to construct a type.
pub trait Kind17 {
    type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17>;
}

#[macro_export]
macro_rules! kind17 {
    ($t:ident) => {
        impl<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17> $crate::datafixers::kinds::generated::Kind17 for $t<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17> {
            type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> = $t<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17>;
        }
    };
}
            
/// Represents a **type constructor** that takes 18 arguments to construct a type.
pub trait Kind18 {
    type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18>;
}

#[macro_export]
macro_rules! kind18 {
    ($t:ident) => {
        impl<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18> $crate::datafixers::kinds::generated::Kind18 for $t<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18> {
            type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> = $t<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18>;
        }
    };
}
            
/// Represents a **type constructor** that takes 19 arguments to construct a type.
pub trait Kind19 {
    type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19>;
}

#[macro_export]
macro_rules! kind19 {
    ($t:ident) => {
        impl<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19> $crate::datafixers::kinds::generated::Kind19 for $t<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19> {
            type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> = $t<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19>;
        }
    };
}
            
/// Represents a **type constructor** that takes 20 arguments to construct a type.
pub trait Kind20 {
    type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20>;
}

#[macro_export]
macro_rules! kind20 {
    ($t:ident) => {
        impl<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20> $crate::datafixers::kinds::generated::Kind20 for $t<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20> {
            type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> = $t<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20>;
        }
    };
}
            
/// Represents a **type constructor** that takes 21 arguments to construct a type.
pub trait Kind21 {
    type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21>;
}

#[macro_export]
macro_rules! kind21 {
    ($t:ident) => {
        impl<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20, _T21> $crate::datafixers::kinds::generated::Kind21 for $t<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20, _T21> {
            type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> = $t<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21>;
        }
    };
}
            
/// Represents a **type constructor** that takes 22 arguments to construct a type.
pub trait Kind22 {
    type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22>;
}

#[macro_export]
macro_rules! kind22 {
    ($t:ident) => {
        impl<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20, _T21, _T22> $crate::datafixers::kinds::generated::Kind22 for $t<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20, _T21, _T22> {
            type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> = $t<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22>;
        }
    };
}
            
/// Represents a **type constructor** that takes 23 arguments to construct a type.
pub trait Kind23 {
    type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23>;
}

#[macro_export]
macro_rules! kind23 {
    ($t:ident) => {
        impl<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20, _T21, _T22, _T23> $crate::datafixers::kinds::generated::Kind23 for $t<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20, _T21, _T22, _T23> {
            type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> = $t<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23>;
        }
    };
}
            
/// Represents a **type constructor** that takes 24 arguments to construct a type.
pub trait Kind24 {
    type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24>;
}

#[macro_export]
macro_rules! kind24 {
    ($t:ident) => {
        impl<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20, _T21, _T22, _T23, _T24> $crate::datafixers::kinds::generated::Kind24 for $t<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20, _T21, _T22, _T23, _T24> {
            type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> = $t<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24>;
        }
    };
}
            
/// Represents a **type constructor** that takes 25 arguments to construct a type.
pub trait Kind25 {
    type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25>;
}

#[macro_export]
macro_rules! kind25 {
    ($t:ident) => {
        impl<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20, _T21, _T22, _T23, _T24, _T25> $crate::datafixers::kinds::generated::Kind25 for $t<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20, _T21, _T22, _T23, _T24, _T25> {
            type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> = $t<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25>;
        }
    };
}
            
/// Represents a **type constructor** that takes 26 arguments to construct a type.
pub trait Kind26 {
    type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26>;
}

#[macro_export]
macro_rules! kind26 {
    ($t:ident) => {
        impl<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20, _T21, _T22, _T23, _T24, _T25, _T26> $crate::datafixers::kinds::generated::Kind26 for $t<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20, _T21, _T22, _T23, _T24, _T25, _T26> {
            type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> = $t<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26>;
        }
    };
}
            
/// Represents a **type constructor** that takes 27 arguments to construct a type.
pub trait Kind27 {
    type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27>;
}

#[macro_export]
macro_rules! kind27 {
    ($t:ident) => {
        impl<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20, _T21, _T22, _T23, _T24, _T25, _T26, _T27> $crate::datafixers::kinds::generated::Kind27 for $t<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20, _T21, _T22, _T23, _T24, _T25, _T26, _T27> {
            type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> = $t<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27>;
        }
    };
}
            
/// Represents a **type constructor** that takes 28 arguments to construct a type.
pub trait Kind28 {
    type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28>;
}

#[macro_export]
macro_rules! kind28 {
    ($t:ident) => {
        impl<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20, _T21, _T22, _T23, _T24, _T25, _T26, _T27, _T28> $crate::datafixers::kinds::generated::Kind28 for $t<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20, _T21, _T22, _T23, _T24, _T25, _T26, _T27, _T28> {
            type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> = $t<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28>;
        }
    };
}
            
/// Represents a **type constructor** that takes 29 arguments to construct a type.
pub trait Kind29 {
    type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29>;
}

#[macro_export]
macro_rules! kind29 {
    ($t:ident) => {
        impl<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20, _T21, _T22, _T23, _T24, _T25, _T26, _T27, _T28, _T29> $crate::datafixers::kinds::generated::Kind29 for $t<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20, _T21, _T22, _T23, _T24, _T25, _T26, _T27, _T28, _T29> {
            type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> = $t<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29>;
        }
    };
}
            
/// Represents a **type constructor** that takes 30 arguments to construct a type.
pub trait Kind30 {
    type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30>;
}

#[macro_export]
macro_rules! kind30 {
    ($t:ident) => {
        impl<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20, _T21, _T22, _T23, _T24, _T25, _T26, _T27, _T28, _T29, _T30> $crate::datafixers::kinds::generated::Kind30 for $t<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20, _T21, _T22, _T23, _T24, _T25, _T26, _T27, _T28, _T29, _T30> {
            type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> = $t<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30>;
        }
    };
}
            
/// Represents a **type constructor** that takes 31 arguments to construct a type.
pub trait Kind31 {
    type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31>;
}

#[macro_export]
macro_rules! kind31 {
    ($t:ident) => {
        impl<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20, _T21, _T22, _T23, _T24, _T25, _T26, _T27, _T28, _T29, _T30, _T31> $crate::datafixers::kinds::generated::Kind31 for $t<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20, _T21, _T22, _T23, _T24, _T25, _T26, _T27, _T28, _T29, _T30, _T31> {
            type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> = $t<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31>;
        }
    };
}
            
/// Represents a **type constructor** that takes 32 arguments to construct a type.
pub trait Kind32 {
    type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32>;
}

#[macro_export]
macro_rules! kind32 {
    ($t:ident) => {
        impl<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20, _T21, _T22, _T23, _T24, _T25, _T26, _T27, _T28, _T29, _T30, _T31, _T32> $crate::datafixers::kinds::generated::Kind32 for $t<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20, _T21, _T22, _T23, _T24, _T25, _T26, _T27, _T28, _T29, _T30, _T31, _T32> {
            type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32> = $t<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32>;
        }
    };
}
            
/// Represents a **type constructor** that takes 33 arguments to construct a type.
pub trait Kind33 {
    type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33>;
}

#[macro_export]
macro_rules! kind33 {
    ($t:ident) => {
        impl<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20, _T21, _T22, _T23, _T24, _T25, _T26, _T27, _T28, _T29, _T30, _T31, _T32, _T33> $crate::datafixers::kinds::generated::Kind33 for $t<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20, _T21, _T22, _T23, _T24, _T25, _T26, _T27, _T28, _T29, _T30, _T31, _T32, _T33> {
            type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33> = $t<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33>;
        }
    };
}
            
/// Represents a **type constructor** that takes 34 arguments to construct a type.
pub trait Kind34 {
    type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34>;
}

#[macro_export]
macro_rules! kind34 {
    ($t:ident) => {
        impl<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20, _T21, _T22, _T23, _T24, _T25, _T26, _T27, _T28, _T29, _T30, _T31, _T32, _T33, _T34> $crate::datafixers::kinds::generated::Kind34 for $t<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20, _T21, _T22, _T23, _T24, _T25, _T26, _T27, _T28, _T29, _T30, _T31, _T32, _T33, _T34> {
            type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34> = $t<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34>;
        }
    };
}
            
/// Represents a **type constructor** that takes 35 arguments to construct a type.
pub trait Kind35 {
    type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35>;
}

#[macro_export]
macro_rules! kind35 {
    ($t:ident) => {
        impl<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20, _T21, _T22, _T23, _T24, _T25, _T26, _T27, _T28, _T29, _T30, _T31, _T32, _T33, _T34, _T35> $crate::datafixers::kinds::generated::Kind35 for $t<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20, _T21, _T22, _T23, _T24, _T25, _T26, _T27, _T28, _T29, _T30, _T31, _T32, _T33, _T34, _T35> {
            type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35> = $t<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35>;
        }
    };
}
            
/// Represents a **type constructor** that takes 36 arguments to construct a type.
pub trait Kind36 {
    type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36>;
}

#[macro_export]
macro_rules! kind36 {
    ($t:ident) => {
        impl<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20, _T21, _T22, _T23, _T24, _T25, _T26, _T27, _T28, _T29, _T30, _T31, _T32, _T33, _T34, _T35, _T36> $crate::datafixers::kinds::generated::Kind36 for $t<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20, _T21, _T22, _T23, _T24, _T25, _T26, _T27, _T28, _T29, _T30, _T31, _T32, _T33, _T34, _T35, _T36> {
            type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36> = $t<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36>;
        }
    };
}
            
/// Represents a **type constructor** that takes 37 arguments to construct a type.
pub trait Kind37 {
    type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37>;
}

#[macro_export]
macro_rules! kind37 {
    ($t:ident) => {
        impl<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20, _T21, _T22, _T23, _T24, _T25, _T26, _T27, _T28, _T29, _T30, _T31, _T32, _T33, _T34, _T35, _T36, _T37> $crate::datafixers::kinds::generated::Kind37 for $t<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20, _T21, _T22, _T23, _T24, _T25, _T26, _T27, _T28, _T29, _T30, _T31, _T32, _T33, _T34, _T35, _T36, _T37> {
            type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37> = $t<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37>;
        }
    };
}
            
/// Represents a **type constructor** that takes 38 arguments to construct a type.
pub trait Kind38 {
    type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38>;
}

#[macro_export]
macro_rules! kind38 {
    ($t:ident) => {
        impl<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20, _T21, _T22, _T23, _T24, _T25, _T26, _T27, _T28, _T29, _T30, _T31, _T32, _T33, _T34, _T35, _T36, _T37, _T38> $crate::datafixers::kinds::generated::Kind38 for $t<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20, _T21, _T22, _T23, _T24, _T25, _T26, _T27, _T28, _T29, _T30, _T31, _T32, _T33, _T34, _T35, _T36, _T37, _T38> {
            type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38> = $t<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38>;
        }
    };
}
            
/// Represents a **type constructor** that takes 39 arguments to construct a type.
pub trait Kind39 {
    type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39>;
}

#[macro_export]
macro_rules! kind39 {
    ($t:ident) => {
        impl<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20, _T21, _T22, _T23, _T24, _T25, _T26, _T27, _T28, _T29, _T30, _T31, _T32, _T33, _T34, _T35, _T36, _T37, _T38, _T39> $crate::datafixers::kinds::generated::Kind39 for $t<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20, _T21, _T22, _T23, _T24, _T25, _T26, _T27, _T28, _T29, _T30, _T31, _T32, _T33, _T34, _T35, _T36, _T37, _T38, _T39> {
            type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39> = $t<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39>;
        }
    };
}
            
/// Represents a **type constructor** that takes 40 arguments to construct a type.
pub trait Kind40 {
    type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40>;
}

#[macro_export]
macro_rules! kind40 {
    ($t:ident) => {
        impl<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20, _T21, _T22, _T23, _T24, _T25, _T26, _T27, _T28, _T29, _T30, _T31, _T32, _T33, _T34, _T35, _T36, _T37, _T38, _T39, _T40> $crate::datafixers::kinds::generated::Kind40 for $t<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20, _T21, _T22, _T23, _T24, _T25, _T26, _T27, _T28, _T29, _T30, _T31, _T32, _T33, _T34, _T35, _T36, _T37, _T38, _T39, _T40> {
            type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40> = $t<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40>;
        }
    };
}
            
/// Represents a **type constructor** that takes 41 arguments to construct a type.
pub trait Kind41 {
    type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41>;
}

#[macro_export]
macro_rules! kind41 {
    ($t:ident) => {
        impl<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20, _T21, _T22, _T23, _T24, _T25, _T26, _T27, _T28, _T29, _T30, _T31, _T32, _T33, _T34, _T35, _T36, _T37, _T38, _T39, _T40, _T41> $crate::datafixers::kinds::generated::Kind41 for $t<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20, _T21, _T22, _T23, _T24, _T25, _T26, _T27, _T28, _T29, _T30, _T31, _T32, _T33, _T34, _T35, _T36, _T37, _T38, _T39, _T40, _T41> {
            type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41> = $t<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41>;
        }
    };
}
            
/// Represents a **type constructor** that takes 42 arguments to construct a type.
pub trait Kind42 {
    type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42>;
}

#[macro_export]
macro_rules! kind42 {
    ($t:ident) => {
        impl<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20, _T21, _T22, _T23, _T24, _T25, _T26, _T27, _T28, _T29, _T30, _T31, _T32, _T33, _T34, _T35, _T36, _T37, _T38, _T39, _T40, _T41, _T42> $crate::datafixers::kinds::generated::Kind42 for $t<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20, _T21, _T22, _T23, _T24, _T25, _T26, _T27, _T28, _T29, _T30, _T31, _T32, _T33, _T34, _T35, _T36, _T37, _T38, _T39, _T40, _T41, _T42> {
            type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42> = $t<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42>;
        }
    };
}
            
/// Represents a **type constructor** that takes 43 arguments to construct a type.
pub trait Kind43 {
    type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43>;
}

#[macro_export]
macro_rules! kind43 {
    ($t:ident) => {
        impl<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20, _T21, _T22, _T23, _T24, _T25, _T26, _T27, _T28, _T29, _T30, _T31, _T32, _T33, _T34, _T35, _T36, _T37, _T38, _T39, _T40, _T41, _T42, _T43> $crate::datafixers::kinds::generated::Kind43 for $t<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20, _T21, _T22, _T23, _T24, _T25, _T26, _T27, _T28, _T29, _T30, _T31, _T32, _T33, _T34, _T35, _T36, _T37, _T38, _T39, _T40, _T41, _T42, _T43> {
            type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43> = $t<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43>;
        }
    };
}
            
/// Represents a **type constructor** that takes 44 arguments to construct a type.
pub trait Kind44 {
    type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44>;
}

#[macro_export]
macro_rules! kind44 {
    ($t:ident) => {
        impl<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20, _T21, _T22, _T23, _T24, _T25, _T26, _T27, _T28, _T29, _T30, _T31, _T32, _T33, _T34, _T35, _T36, _T37, _T38, _T39, _T40, _T41, _T42, _T43, _T44> $crate::datafixers::kinds::generated::Kind44 for $t<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20, _T21, _T22, _T23, _T24, _T25, _T26, _T27, _T28, _T29, _T30, _T31, _T32, _T33, _T34, _T35, _T36, _T37, _T38, _T39, _T40, _T41, _T42, _T43, _T44> {
            type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44> = $t<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44>;
        }
    };
}
            
/// Represents a **type constructor** that takes 45 arguments to construct a type.
pub trait Kind45 {
    type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45>;
}

#[macro_export]
macro_rules! kind45 {
    ($t:ident) => {
        impl<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20, _T21, _T22, _T23, _T24, _T25, _T26, _T27, _T28, _T29, _T30, _T31, _T32, _T33, _T34, _T35, _T36, _T37, _T38, _T39, _T40, _T41, _T42, _T43, _T44, _T45> $crate::datafixers::kinds::generated::Kind45 for $t<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20, _T21, _T22, _T23, _T24, _T25, _T26, _T27, _T28, _T29, _T30, _T31, _T32, _T33, _T34, _T35, _T36, _T37, _T38, _T39, _T40, _T41, _T42, _T43, _T44, _T45> {
            type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45> = $t<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45>;
        }
    };
}
            
/// Represents a **type constructor** that takes 46 arguments to construct a type.
pub trait Kind46 {
    type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46>;
}

#[macro_export]
macro_rules! kind46 {
    ($t:ident) => {
        impl<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20, _T21, _T22, _T23, _T24, _T25, _T26, _T27, _T28, _T29, _T30, _T31, _T32, _T33, _T34, _T35, _T36, _T37, _T38, _T39, _T40, _T41, _T42, _T43, _T44, _T45, _T46> $crate::datafixers::kinds::generated::Kind46 for $t<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20, _T21, _T22, _T23, _T24, _T25, _T26, _T27, _T28, _T29, _T30, _T31, _T32, _T33, _T34, _T35, _T36, _T37, _T38, _T39, _T40, _T41, _T42, _T43, _T44, _T45, _T46> {
            type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46> = $t<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46>;
        }
    };
}
            
/// Represents a **type constructor** that takes 47 arguments to construct a type.
pub trait Kind47 {
    type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47>;
}

#[macro_export]
macro_rules! kind47 {
    ($t:ident) => {
        impl<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20, _T21, _T22, _T23, _T24, _T25, _T26, _T27, _T28, _T29, _T30, _T31, _T32, _T33, _T34, _T35, _T36, _T37, _T38, _T39, _T40, _T41, _T42, _T43, _T44, _T45, _T46, _T47> $crate::datafixers::kinds::generated::Kind47 for $t<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20, _T21, _T22, _T23, _T24, _T25, _T26, _T27, _T28, _T29, _T30, _T31, _T32, _T33, _T34, _T35, _T36, _T37, _T38, _T39, _T40, _T41, _T42, _T43, _T44, _T45, _T46, _T47> {
            type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47> = $t<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47>;
        }
    };
}
            
/// Represents a **type constructor** that takes 48 arguments to construct a type.
pub trait Kind48 {
    type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48>;
}

#[macro_export]
macro_rules! kind48 {
    ($t:ident) => {
        impl<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20, _T21, _T22, _T23, _T24, _T25, _T26, _T27, _T28, _T29, _T30, _T31, _T32, _T33, _T34, _T35, _T36, _T37, _T38, _T39, _T40, _T41, _T42, _T43, _T44, _T45, _T46, _T47, _T48> $crate::datafixers::kinds::generated::Kind48 for $t<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20, _T21, _T22, _T23, _T24, _T25, _T26, _T27, _T28, _T29, _T30, _T31, _T32, _T33, _T34, _T35, _T36, _T37, _T38, _T39, _T40, _T41, _T42, _T43, _T44, _T45, _T46, _T47, _T48> {
            type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48> = $t<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48>;
        }
    };
}
            
/// Represents a **type constructor** that takes 49 arguments to construct a type.
pub trait Kind49 {
    type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48, T49>;
}

#[macro_export]
macro_rules! kind49 {
    ($t:ident) => {
        impl<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20, _T21, _T22, _T23, _T24, _T25, _T26, _T27, _T28, _T29, _T30, _T31, _T32, _T33, _T34, _T35, _T36, _T37, _T38, _T39, _T40, _T41, _T42, _T43, _T44, _T45, _T46, _T47, _T48, _T49> $crate::datafixers::kinds::generated::Kind49 for $t<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20, _T21, _T22, _T23, _T24, _T25, _T26, _T27, _T28, _T29, _T30, _T31, _T32, _T33, _T34, _T35, _T36, _T37, _T38, _T39, _T40, _T41, _T42, _T43, _T44, _T45, _T46, _T47, _T48, _T49> {
            type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48, T49> = $t<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48, T49>;
        }
    };
}
            
/// Represents a **type constructor** that takes 50 arguments to construct a type.
pub trait Kind50 {
    type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48, T49, T50>;
}

#[macro_export]
macro_rules! kind50 {
    ($t:ident) => {
        impl<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20, _T21, _T22, _T23, _T24, _T25, _T26, _T27, _T28, _T29, _T30, _T31, _T32, _T33, _T34, _T35, _T36, _T37, _T38, _T39, _T40, _T41, _T42, _T43, _T44, _T45, _T46, _T47, _T48, _T49, _T50> $crate::datafixers::kinds::generated::Kind50 for $t<_T1, _T2, _T3, _T4, _T5, _T6, _T7, _T8, _T9, _T10, _T11, _T12, _T13, _T14, _T15, _T16, _T17, _T18, _T19, _T20, _T21, _T22, _T23, _T24, _T25, _T26, _T27, _T28, _T29, _T30, _T31, _T32, _T33, _T34, _T35, _T36, _T37, _T38, _T39, _T40, _T41, _T42, _T43, _T44, _T45, _T46, _T47, _T48, _T49, _T50> {
            type This<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48, T49, T50> = $t<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48, T49, T50>;
        }
    };
}
            