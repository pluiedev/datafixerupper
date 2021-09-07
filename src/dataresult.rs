use crate::Lifecycle;

pub struct DataResult<R> {
    result: Result<R, PartialResult<R>>,
    lifecycle: Lifecycle,
}

impl<R> DataResult<R> {
    pub fn new(result: Result<R, PartialResult<R>>, lifecycle: Lifecycle) -> Self {
        Self { result, lifecycle }
    }

    pub fn success(result: R, lifecycle: Lifecycle) -> Self {
        Self {
            result: Ok(result),
            lifecycle
        }
    }

    pub fn error(message: String, lifecycle: Lifecycle) -> Self {
        Self {
            result: Err(PartialResult::new(vec![message], None)),
            lifecycle
        }
    }

    pub fn map<T, F>(self, function: F) -> DataResult<T>
    where
        F: Fn(R) -> T,
    {
        let mapped = match self.result {
            Ok(r) => function(r),
            Err(e) => PartialResult::new(e.messages, e.partial_result.map(function)),
        };
        Self::new(mapped, self.lifecycle)
    }

    pub fn flat_map<R2, F>(self, function: F) -> DataResult<R2>
    where
        F: Fn(R) -> DataResult<R2>,
    {
        match self.result {
            Ok(l) => {
                let second = function(l);
                Self::new(second.result, self.lifecycle + second.lifecycle)
            }
            Err(r) => {
                r.partial_result
                    .map(|v| {
                        let second = function(v);
                        // just please
                        // send help
                        let partial = match second.result {
                            Ok(l2) => PartialResult::new(r.messages, Some(l2)),
                            Err(r2) => {
                                r.messages.extend(&r2.messages);
                                PartialResult::new(r.messages, r2.partial_result)
                            }
                        };
                        Self::new(Err(partial), self.lifecycle + second.lifecycle)
                    })
                    .unwrap_or_else(|| {
                        Self::new(Err(PartialResult::new(r.messages, None)), self.lifecycle)
                    })
            }
        }
    }

    pub fn set_lifecycle(self, lifecycle: Lifecycle) -> Self {
        self.lifecycle = lifecycle;
        self
    }

    pub fn ok(&self) -> Option<R> {
        self.result.ok()
    }

    pub fn err(&self) -> Option<PartialResult<R>> {
        self.result.err()
    }
}

pub struct PartialResult<R> {
    messages: Vec<String>,
    partial_result: Option<R>,
}

impl<R> PartialResult<R> {
    pub fn new(messages: Vec<String>, partial_result: Option<R>) -> Self {
        Self {
            messages,
            partial_result,
        }
    }

    pub fn flat_map<R2, F>(self, function: F) -> PartialResult<R2>
    where
        F: Fn(R) -> PartialResult<R2>,
    {
        if let Some(res) = self.partial_result {
            let result = function(res);
            self.messages.push(result.message);
            PartialResult {
                messages: self.messages,
                partial_result: result.partial_result,
            }
        } else {
            PartialResult {
                messages: self.messages,
                partial_result: None,
            }
        }
    }

    pub const fn message(&self) -> &str {
        self.message
    }
}
