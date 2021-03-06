#![warn(
	clippy::pedantic,
	clippy::nursery,
	clippy::suspicious,
	clippy::str_to_string,
	clippy::string_to_string,
	missing_copy_implementations,
	missing_docs
)]
#![deny(clippy::all)]
#![allow(clippy::module_name_repetitions, clippy::no_effect_underscore_binding)]
#![cfg_attr(
	docsrs,
	feature(doc_auto_cfg, doc_cfg),
	deny(rustdoc::broken_intra_doc_links)
)]
#![cfg_attr(not(test), warn(clippy::panic_in_result_fn))]
//! A lazy markdown utility crate for Discord.

mod adaptors;

use std::fmt::Display;

pub use self::adaptors::*;

/// An interface for helping with markdown. Each adaptor is lazy and not used until the [`Display`]
/// method is used.
pub trait Markdown: Display {
    /// Applies bold markdown to the text.
	fn bold(self) -> Bold<Self>
	where
		Self: Sized,
	{
		Bold::new(self)
	}

    /// Applies italic markdown to the text.
	fn italic(self) -> Italic<Self>
	where
		Self: Sized,
	{
		Italic::new(self)
	}

    /// Applies underscoring markdown to the text.
	fn underscore(self) -> Underscore<Self>
	where
		Self: Sized,
	{
		Underscore::new(self)
	}

    /// Applies strikethrough to the text.
	fn strikethrough(self) -> Strikethrough<Self>
	where
		Self: Sized,
	{
		Strikethrough::new(self)
	}

    /// Quotes the text.
	fn quote(self) -> Quote<Self>
	where
		Self: Sized,
	{
		Quote::new(self)
	}

    /// Block-quotes the text.
	fn block_quote(self) -> BlockQuote<Self>
	where
		Self: Sized,
	{
		BlockQuote::new(self)
	}

    /// Marks the text as a spoiler.
	fn spoiler(self) -> Spoiler<Self>
	where
		Self: Sized,
	{
		Spoiler::new(self)
	}

    /// Creates a codeblock with no language of the text.
	fn codeblock(self) -> Codeblock<Self>
	where
		Self: Sized,
	{
		Codeblock::new(self)
	}

    /// Creates a codeblock with the provided language of the text.
	fn codeblock_with<Lang: Display>(self, lang: Lang) -> CodeblockWith<Self, Lang>
	where
		Self: Sized,
	{
		CodeblockWith::new(self, lang)
	}

    /// Creates an inline codeblock of the text.
	fn inline_codeblock(self) -> InlineCodeblock<Self>
	where
		Self: Sized,
	{
		InlineCodeblock::new(self)
	}
}

impl<T: Display> Markdown for T {}

#[cfg(test)]
mod tests {
	use super::Markdown;

	#[test]
	fn bold() {
		assert_eq!("hey there".bold().to_string(), "**hey there**");
	}

	#[test]
	fn italic() {
		assert_eq!("hey there".italic().to_string(), "_hey there_");
	}

	#[test]
	fn underscore() {
		assert_eq!("hey there".underscore().to_string(), "__hey there__");
	}

	#[test]
	fn strikethrough() {
		assert_eq!("hey there".strikethrough().to_string(), "~~hey there~~");
	}

	#[test]
	fn quote() {
		assert_eq!("hey there".quote().to_string(), "> hey there");
	}

	#[test]
	fn block_quote() {
		assert_eq!("hey there".block_quote().to_string(), ">>> hey there");
	}

	#[test]
	fn spoiler() {
		assert_eq!("hey there".spoiler().to_string(), "||hey there||");
	}

	#[test]
	fn codeblocks() {
		assert_eq!("hey there".codeblock().to_string(), "```\nhey there```");

		assert_eq!("hey there".codeblock_with("rs").to_string(), "```rs\nhey there```");
	}

	#[test]
	fn inline_codeblock() {
		assert_eq!("hey there".inline_codeblock().to_string(), "`hey there`");
	}
}