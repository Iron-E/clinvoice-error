/// Creates a `pub type` named `Result`. See examples below.
///
/// # Examples
///
/// For when you have a type named `Error`:
///
/// ```rust
/// use clinvoice_error::AliasResult;
/// # use pretty_assertions::assert_eq;
///
/// {
///   #[derive(Debug, PartialEq)]
///   struct Error;
///
///   AliasResult!();
///   assert_eq!(Result::Ok(()), std::result::Result::<(), Error>::Ok(()));
/// }
///
/// {
///   #[derive(Debug, PartialEq)]
///   struct SomeError;
///
///   AliasResult!(SomeError);
///   assert_eq!(Result::Ok(()), std::result::Result::<(), SomeError>::Ok(()));
/// }
/// ```
#[macro_export]
macro_rules! AliasResult {
	() => {
		clinvoice_error::AliasResult!(Error);
	};

	($error:ident) => {
		/// An alias of [`::core::result::Result<T, _>`].
		pub type Result<T> = ::core::result::Result<T, $error>;
	};
}
