//! Delimiter pairs for inline and display math.

use serde::{Deserialize, Serialize};

/// Left and right delimiters for a math expression.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Delimiter {
    /// Opening delimiter.
    pub left: String,
    /// Closing delimiter.
    pub right: String,
}

impl Delimiter {
    /// Create a delimiter that uses the same string on both sides.
    pub fn symmetric(value: impl Into<String>) -> Self {
        let value = value.into();
        Self {
            left: value.clone(),
            right: value,
        }
    }

    /// First byte of the opening delimiter.
    pub fn first_byte(&self) -> u8 {
        self.left.as_bytes()[0]
    }

    /// Whether `candidate` begins with this delimiter's opening side.
    pub fn opens_with(&self, candidate: &[u8]) -> bool {
        if self.left.len() > candidate.len() {
            return false;
        }
        self.left
            .as_bytes()
            .iter()
            .zip(candidate)
            .all(|(a, b)| a == b)
    }

    /// Wrap an expression with this delimiter pair.
    pub fn wrap(&self, expression: &str) -> String {
        let mut output =
            String::with_capacity(self.left.len() + expression.len() + self.right.len());
        output.push_str(&self.left);
        output.push_str(expression);
        output.push_str(&self.right);
        output
    }
}
