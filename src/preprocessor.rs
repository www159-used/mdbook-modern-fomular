//! mdBook preprocessor trait implementation.

use mdbook_preprocessor::book::Book;
use mdbook_preprocessor::errors::Result;
use mdbook_preprocessor::{Preprocessor, PreprocessorContext};

use crate::chapter::RenderSetup;
use crate::config;
use crate::constants::PREPROCESSOR_NAME;

pub struct FomularPreprocessor;

impl Preprocessor for FomularPreprocessor {
    fn name(&self) -> &str {
        PREPROCESSOR_NAME
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book> {
        let config = config::load(&ctx.config);
        let setup = RenderSetup::from_config(&config, &ctx.root);

        book.for_each_chapter_mut(|ch| {
            ch.content = setup.render(&ch.content);
        });

        Ok(book)
    }
}
