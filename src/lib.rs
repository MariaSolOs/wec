#![warn(missing_docs)]

//! A collection of useful Rust macros for creating `vec!`s.

/// Boxes each element in the vector.
///
/// # Examples
/// ```
/// use wec::bec;
///
/// let v = bec![1, 2, 3];
/// assert_eq!(v, vec![Box::new(1), Box::new(2), Box::new(3)]);
/// ```
#[macro_export]
macro_rules! bec {
    () => { ::std::vec![] };

    ( $($elem:expr),+ $(,)? ) => {
        ::std::vec![ $( ::std::boxed::Box::new($elem) ),+ ]
    };
}

/// Calls `.into()` on each element in the vector.
///
/// # Examples
/// ```
/// use wec::vinto;
///
/// let v: Vec<String> = vinto!["foo", String::from("bar")];
/// assert_eq!(v, vec![String::from("foo"), String::from("bar")]);
/// ```
#[macro_export]
macro_rules! vinto {
    () => { ::std::vec![] };

    ( $($elem:expr),+ $(,)? ) => { ::std::vec![ $( ::std::convert::Into::into($elem) ),+ ] };
}
