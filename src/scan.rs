//! Scan markdown chapter content and locate math segments.

use std::collections::VecDeque;

use crate::delimiter::Delimiter;

/// Boundary event emitted while scanning a chapter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScanEvent {
    /// Start of a plain-text or math segment.
    SegmentStart(usize),
    /// End of a plain-text segment.
    TextEnd(usize),
    /// End of an inline math segment (exclusive of delimiters).
    InlineEnd(usize),
    /// End of a display math segment (exclusive of delimiters).
    DisplayEnd(usize),
}

/// Stateful scanner for math delimiters inside markdown.
#[derive(Debug)]
pub struct Scanner<'source> {
    source: &'source str,
    cursor: usize,
    pending: VecDeque<ScanEvent>,
    display_delimiter: &'source Delimiter,
    inline_delimiter: &'source Delimiter,
}

impl<'source> Scanner<'source> {
    /// Create a scanner for `source` using the configured delimiters.
    pub fn new(
        source: &'source str,
        display_delimiter: &'source Delimiter,
        inline_delimiter: &'source Delimiter,
    ) -> Self {
        Self {
            source,
            cursor: 0,
            pending: VecDeque::new(),
            display_delimiter,
            inline_delimiter,
        }
    }
}

impl Iterator for Scanner<'_> {
    type Item = ScanEvent;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(event) = self.pending.pop_front() {
                return Some(event);
            }
            self.advance_byte().ok()?;
        }
    }
}

impl Scanner<'_> {
    fn bytes(&self) -> &[u8] {
        self.source.as_bytes()
    }

    fn advance_byte(&mut self) -> Result<(), ()> {
        let byte = self.current_byte()?;
        self.cursor += 1;

        match byte {
            b if b == self.display_delimiter.first_byte()
                && self
                    .display_delimiter
                    .opens_with(&self.bytes()[self.cursor - 1..]) =>
            {
                self.cursor -= 1;
                self.consume_math(false)?;
            }
            b if b == self.inline_delimiter.first_byte()
                && self
                    .inline_delimiter
                    .opens_with(&self.bytes()[self.cursor - 1..]) =>
            {
                self.cursor -= 1;
                self.consume_math(true)?;
            }
            b'\\' => {
                self.cursor += 1;
            }
            b'`' => self.skip_fenced_code()?,
            _ => {}
        }

        Ok(())
    }

    fn current_byte(&self) -> Result<u8, ()> {
        self.bytes().get(self.cursor).copied().ok_or(())
    }

    fn skip_fenced_code(&mut self) -> Result<(), ()> {
        let mut fence_len = 1;
        while self.current_byte()? == b'`' {
            self.cursor += 1;
            fence_len += 1;
        }

        loop {
            let offset = self.find_fence_close(fence_len)?;
            self.cursor += offset + fence_len;

            if self.current_byte()? == b'`' {
                self.cursor += 1;
                while self.current_byte()? == b'`' {
                    self.cursor += 1;
                }
            } else {
                break;
            }
        }

        Ok(())
    }

    fn find_fence_close(&self, fence_len: usize) -> Result<usize, ()> {
        let bytes = self.bytes();
        let mut index = self.cursor;
        while index + fence_len <= bytes.len() {
            if bytes[index..index + fence_len]
                .iter()
                .all(|byte| *byte == b'`')
            {
                return Ok(index - self.cursor);
            }
            index += 1;
        }
        Err(())
    }

    fn consume_math(&mut self, inline: bool) -> Result<(), ()> {
        if self.cursor > 0 {
            self.pending.push_back(ScanEvent::TextEnd(self.cursor));
        }

        let delimiter = if inline {
            self.inline_delimiter
        } else {
            self.display_delimiter
        };

        self.cursor += delimiter.left.len();
        self.pending.push_back(ScanEvent::SegmentStart(self.cursor));

        loop {
            self.cursor += self.source[self.cursor..]
                .find(&delimiter.right)
                .ok_or(())?;

            if !self.is_closing_delimiter_escaped() {
                let end = if inline {
                    ScanEvent::InlineEnd(self.cursor)
                } else {
                    ScanEvent::DisplayEnd(self.cursor)
                };
                self.pending.push_back(end);
                self.cursor += delimiter.right.len();
                self.pending.push_back(ScanEvent::SegmentStart(self.cursor));
                break;
            }

            self.cursor += delimiter.right.len();
        }

        Ok(())
    }

    fn is_closing_delimiter_escaped(&self) -> bool {
        let mut escaped = false;
        let mut index = self.cursor;
        loop {
            index -= 1;
            match self.bytes().get(index) {
                Some(b'\\') => escaped = !escaped,
                _ => break,
            }
        }
        escaped
    }
}
