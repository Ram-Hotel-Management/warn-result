#![cfg_attr(feature = "nightly", feature(try_trait_v2))]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Warning or Ok (WOK)
pub enum WOK<T, W> {
    Ok(T),
    Warning(W),
}

impl<T, W> WOK<T, W> {
    pub fn ok(val: T) -> Self {
        Self::Ok(val)
    }
    pub fn warning(warn: W) -> Self {
        Self::Warning(warn)
    }

    pub fn unwrap(self) -> T {
        match self {
            Self::Ok(t) => t,
            Self::Warning(_) => panic!("called `unwrap()` on a `Warning` value"),
        }
    }

    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> WOK<U, W> {
        match self {
            Self::Ok(t) => WOK::Ok(f(t)),
            Self::Warning(w) => WOK::Warning(w),
        }
    }

    pub fn map_warning<U, F: FnOnce(W) -> U>(self, f: F) -> WOK<T, U> {
        match self {
            Self::Ok(t) => WOK::Ok(t),
            Self::Warning(w) => WOK::Warning(f(w)),
        }
    }

    pub fn is_ok(&self) -> bool {
        matches!(self, Self::Ok(_))
    }

    pub fn is_warning(&self) -> bool {
        matches!(self, Self::Warning(_))
    }

    pub fn unwrap_or(self, default: T) -> T {
        match self {
            Self::Ok(t) => t,
            _ => default,
        }
    }

    pub fn unwrap_or_else<F: FnOnce() -> T>(self, f: F) -> T {
        match self {
            Self::Ok(t) => t,
            _ => f(),
        }
    }
}

impl<T, W> WOK<T, W>
where
    T: Default,
{
    pub fn unwrap_or_default(self) -> T {
        match self {
            WOK::Ok(t) => t,
            WOK::Warning(_) => T::default(),
        }
    }
}

impl<T, W> From<Result<T, W>> for WOK<T, W> {
    fn from(res: Result<T, W>) -> Self {
        match res {
            Ok(t) => WOK::Ok(t),
            Err(w) => WOK::Warning(w),
        }
    }
}

impl<T, W, E> From<WarnResult<T, W, E>> for WOK<T, W>
where
    E: Into<W>,
{
    fn from(val: WarnResult<T, W, E>) -> Self {
        match val {
            WarnResult::Ok(t) => WOK::Ok(t),
            WarnResult::Warning(w) => WOK::Warning(w),
            WarnResult::Err(e) => WOK::Warning(e.into()),
        }
    }
}

impl<T, W, E> From<WOK<T, W>> for WarnResult<T, W, E> {
    fn from(val: WOK<T, W>) -> Self {
        match val {
            WOK::Ok(t) => WarnResult::Ok(t),
            WOK::Warning(w) => WarnResult::Warning(w),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WarnResult<T, W, E> {
    Ok(T),
    Warning(W),
    Err(E),
}

impl<T, W, E> WarnResult<T, W, E> {
    pub fn ok(val: T) -> Self {
        Self::Ok(val)
    }
    pub fn warning(warn: W) -> Self {
        Self::Warning(warn)
    }
    pub fn err(err: E) -> Self {
        Self::Err(err)
    }

    pub fn is_ok(&self) -> bool {
        matches!(self, Self::Ok(_))
    }
    pub fn is_warning(&self) -> bool {
        matches!(self, Self::Warning(_))
    }
    pub fn is_err(&self) -> bool {
        matches!(self, Self::Err(_))
    }

    pub fn unwrap(self) -> T {
        match self {
            Self::Ok(t) => t,
            Self::Warning(_) => panic!("called `unwrap()` on a `Warning` value"),
            Self::Err(_) => panic!("called `unwrap()` on an `Err` value"),
        }
    }

    pub fn unwrap_or(self, default: T) -> T {
        match self {
            Self::Ok(t) => t,
            _ => default,
        }
    }

    pub fn unwrap_or_else<F: FnOnce() -> T>(self, f: F) -> T {
        match self {
            Self::Ok(t) => t,
            _ => f(),
        }
    }

    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> WarnResult<U, W, E> {
        match self {
            Self::Ok(t) => WarnResult::Ok(f(t)),
            Self::Warning(w) => WarnResult::Warning(w),
            Self::Err(e) => WarnResult::Err(e),
        }
    }

    pub fn map_warning<U, F: FnOnce(W) -> U>(self, f: F) -> WarnResult<T, U, E> {
        match self {
            Self::Ok(t) => WarnResult::Ok(t),
            Self::Warning(w) => WarnResult::Warning(f(w)),
            Self::Err(e) => WarnResult::Err(e),
        }
    }

    pub fn map_err<U, F: FnOnce(E) -> U>(self, f: F) -> WarnResult<T, W, U> {
        match self {
            Self::Ok(t) => WarnResult::Ok(t),
            Self::Warning(w) => WarnResult::Warning(w),
            Self::Err(e) => WarnResult::Err(f(e)),
        }
    }

    pub fn ok_or(self, warn_default: T, err_default: T) -> T {
        match self {
            Self::Ok(t) => t,
            Self::Warning(_) => warn_default,
            Self::Err(_) => err_default,
        }
    }

    pub fn ok_or_else<F1: FnOnce() -> T, F2: FnOnce() -> T>(
        self,
        warn_default: F1,
        err_default: F2,
    ) -> T {
        match self {
            Self::Ok(t) => t,
            Self::Warning(_) => warn_default(),
            Self::Err(_) => err_default(),
        }
    }
}

// Conversion to/from standard Result for interop
impl<T, W, E> From<Result<T, E>> for WarnResult<T, W, E> {
    fn from(res: Result<T, E>) -> Self {
        match res {
            Ok(t) => WarnResult::Ok(t),
            Err(e) => WarnResult::Err(e),
        }
    }
}

impl<T, W, E> From<WarnResult<T, W, E>> for Result<T, E>
where
    W: Into<E>,
{
    fn from(val: WarnResult<T, W, E>) -> Self {
        match val {
            WarnResult::Ok(t) => Ok(t),
            WarnResult::Warning(w) => Err(w.into()),
            WarnResult::Err(e) => Err(e),
        }
    }
}

impl<T, W, E> WarnResult<T, W, E>
where
    T: Default,
{
    pub fn unwrap_or_default(self) -> T {
        match self {
            WarnResult::Ok(t) => t,
            WarnResult::Warning(_) => T::default(),
            WarnResult::Err(_) => T::default(),
        }
    }
}

// Optionally, add a macro to mimic `?` for warnings
#[macro_export]
/// A macro to simplify working with `WarnResult` types, allowing for early returns on warnings.
/// as well as errs
macro_rules! warn_try_sc_warning {
    ($expr:expr) => {
        match $expr {
            $crate::WarnResult::Ok(val) => val,
            $crate::WarnResult::Warning(warn) => return $crate::WarnResult::Warning(warn),
            $crate::WarnResult::Err(err) => return $crate::WarnResult::Err(err),
        }
    };
}

/// Optionally, add a macro to mimic `?` for errors
/// This macro allows for early returns on errors, while still allowing warnings to be handled separately.
#[macro_export]
macro_rules! warn_try_sc_error {
    ($expr:expr) => {
        match $expr {
            $crate::WarnResult::Ok(val) => $crate::WOK::Ok(val),
            $crate::WarnResult::Warning(warn) => $crate::WOK::Warning(warn),
            $crate::WarnResult::Err(err) => return $crate::WarnResult::Err(err),
        }
    };
}

/// same as 'warn_try_sc_error'
/// but return Result<_,_>
#[macro_export]
macro_rules! warn_try_sc_error_result {
    ($expr:expr) => {
        match $expr {
            $crate::WarnResult::Ok(val) => $crate::WOK::Ok(val),
            $crate::WarnResult::Warning(warn) => $crate::WOK::Warning(warn),
            $crate::WarnResult::Err(err) => return Err(err),
        }
    };
}

// Nightly compatibility using the Try trait
#[cfg(feature = "nightly")]
impl<T, W, E> std::ops::Try for WarnResult<T, W, E> {
    type Output = T;
    type Residual = WarnResult<std::convert::Infallible, W, E>;

    fn from_output(output: T) -> Self {
        WarnResult::Ok(output)
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, T> {
        match self {
            WarnResult::Ok(t) => std::ops::ControlFlow::Continue(t),
            WarnResult::Warning(w) => std::ops::ControlFlow::Break(WarnResult::Warning(w)),
            WarnResult::Err(e) => std::ops::ControlFlow::Break(WarnResult::Err(e)),
        }
    }
}
