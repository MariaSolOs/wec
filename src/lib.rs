#[macro_export]
macro_rules! bec {
    ( $($elem:expr),* ) => {
        ::std::vec![ $( ::std::boxed::Box::new($elem) ),* ]
    };
}

#[macro_export]
macro_rules! vinto {
    ( $($elem:expr),* ) => { ::std::vec![ $( ::std::convert::Into::into($elem) ),* ] };
}
