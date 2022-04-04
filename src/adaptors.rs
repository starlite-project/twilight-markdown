use std::fmt::{Display, Formatter, Result as FmtResult, Write};

/// Bold text formatter.
#[derive(Debug, Clone)]
#[repr(transparent)]
#[must_use = "markdown formatters are lazy and must be used"]
pub struct Bold<T>(pub(super) T);

impl<T> Bold<T> {
	pub(super) const fn new(value: T) -> Self {
		Self(value)
	}
}

impl<T: Display> Display for Bold<T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.write_str("**")?;
		Display::fmt(&self.0, f)?;
		f.write_str("**")
	}
}

/// Italic text formatter.
#[derive(Debug, Clone)]
#[repr(transparent)]
#[must_use = "markdown formatters are lazy and must be used"]
pub struct Italic<T>(pub(super) T);

impl<T> Italic<T> {
	pub(super) const fn new(value: T) -> Self {
		Self(value)
	}
}

impl<T: Display> Display for Italic<T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.write_char('_')?;
		Display::fmt(&self.0, f)?;
		f.write_char('_')
	}
}

/// Underscore text formatter.
#[derive(Debug, Clone)]
#[repr(transparent)]
#[must_use = "markdown formatters are lazy and must be used"]
pub struct Underscore<T>(pub(super) T);

impl<T> Underscore<T> {
	pub(super) const fn new(value: T) -> Self {
		Self(value)
	}
}

impl<T: Display> Display for Underscore<T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.write_str("__")?;
		Display::fmt(&self.0, f)?;
		f.write_str("__")
	}
}

#[derive(Debug, Clone)]
#[repr(transparent)]
#[must_use = "markdown formatters are lazy and must be used"]
pub struct Strikethrough<T>(pub(super) T);

impl<T> Strikethrough<T> {
	pub(super) const fn new(value: T) -> Self {
		Self(value)
	}
}

impl<T: Display> Display for Strikethrough<T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.write_str("~~")?;
		Display::fmt(&self.0, f)?;
		f.write_str("~~")
	}
}

#[derive(Debug, Clone)]
#[repr(transparent)]
#[must_use = "markdown formatters are lazy and must be used"]
pub struct Quote<T>(pub(super) T);

impl<T> Quote<T> {
	pub(super) const fn new(value: T) -> Self {
		Self(value)
	}
}

impl<T: Display> Display for Quote<T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.write_str("> ")?;
		Display::fmt(&self.0, f)
	}
}

#[derive(Debug, Clone)]
#[repr(transparent)]
#[must_use = "markdown formatters are lazy and must be used"]
pub struct BlockQuote<T>(pub(super) T);

impl<T> BlockQuote<T> {
	pub(super) const fn new(value: T) -> Self {
		Self(value)
	}
}

impl<T: Display> Display for BlockQuote<T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.write_str(">>> ")?;
		Display::fmt(&self.0, f)
	}
}

#[derive(Debug, Clone)]
#[repr(transparent)]
#[must_use = "markdown formatters are lazy and must be used"]
pub struct Spoiler<T>(pub(super) T);

impl<T> Spoiler<T> {
	pub(super) const fn new(value: T) -> Self {
		Self(value)
	}
}

impl<T: Display> Display for Spoiler<T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.write_str("||")?;
		Display::fmt(&self.0, f)?;
		f.write_str("||")
	}
}

#[derive(Debug, Clone)]
#[must_use = "markdown formatters are lazy and must be used"]
pub struct Codeblock<T>(pub(super) T, Option<String>);

impl<T> Codeblock<T> {
	pub(super) const fn new(value: T) -> Self {
		Self(value, None)
	}

	pub(super) const fn with_lang(value: T, lang: String) -> Self {
		Self(value, Some(lang))
	}
}

impl<T: Display> Display for Codeblock<T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		let lang = self.1.as_deref().unwrap_or_default();

		f.write_str("```")?;
		Display::fmt(lang, f)?;
		f.write_char('\n')?;
		Display::fmt(&self.0, f)?;
		f.write_str("```")
	}
}

#[derive(Debug, Clone)]
#[repr(transparent)]
#[must_use = "markdown formatters are lazy and must be used"]
pub struct InlineCodeblock<T>(pub(super) T);

impl<T> InlineCodeblock<T> {
	pub(super) const fn new(value: T) -> Self {
		Self(value)
	}
}

impl<T: Display> Display for InlineCodeblock<T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.write_char('`')?;
		Display::fmt(&self.0, f)?;
		f.write_char('`')
	}
}
