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

/// Strikethrough text formatter.
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

/// Quote formatter.
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

/// Block quote formatter
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

/// Spoiler formatter.
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

/// Multi-line code block formatter.
#[derive(Debug, Clone)]
#[repr(transparent)]
#[must_use = "markdown formatters are lazy and must be used"]
pub struct Codeblock<T>(pub(super) T);

impl<T> Codeblock<T> {
	pub(super) const fn new(value: T) -> Self {
		Self(value)
	}
}

impl<T: Display> Display for Codeblock<T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.write_str("```\n")?;
		Display::fmt(&self.0, f)?;
		f.write_str("```")
	}
}

/// Multi-line code block formatter with language.
#[derive(Debug, Clone)]
#[must_use = "markdown formatters are lazy and must be used"]
pub struct CodeblockWith<T, Lang>(pub(super) T, pub(super) Lang);

impl<T, Lang> CodeblockWith<T, Lang> {
	pub (super) const fn new(value: T, lang: Lang) -> Self {
		Self(value, lang)
	}
}

impl<T: Display, Lang: Display> Display for CodeblockWith<T, Lang> {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.write_str("```")?;
		Display::fmt(&self.1, f)?;
		f.write_char('\n')?;
		Display::fmt(&self.0, f)?;
		f.write_str("```")
	}
}

/// Inline code block formatter
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
