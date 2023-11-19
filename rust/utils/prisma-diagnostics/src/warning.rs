use crate::{
    pretty_print::{pretty_print, DiagnosticColorer},
    Span,
};
use colored::{ColoredString, Colorize};

/// A non-fatal warning emitted by the schema parser.
/// For fancy printing, please use the `pretty_print_error` function.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SchemaWarning {
    message: String,
    span: Span,
}

impl SchemaWarning {
    /// You should avoid using this constructor directly when possible, and define warnings as public methods of this class.
    /// The constructor is only left public for supporting connector-specific warnings (which should not live in the core).
    pub fn new(message: String, span: Span) -> SchemaWarning {
        SchemaWarning { message, span }
    }

    /// The user-facing warning message.
    pub fn message(&self) -> &str {
        &self.message
    }

    /// The source span the warning applies to.
    pub fn span(&self) -> Span {
        self.span
    }

    pub fn pretty_print(
        &self,
        f: &mut dyn std::io::Write,
        file_name: &str,
        text: &str,
    ) -> std::io::Result<()> {
        pretty_print(
            f,
            file_name,
            text,
            self.span(),
            self.message.as_ref(),
            &SchemaWarningColorer {},
        )
    }
}

struct SchemaWarningColorer {}

impl DiagnosticColorer for SchemaWarningColorer {
    fn title(&self) -> &'static str {
        "warning"
    }

    fn primary_color(&self, token: &'_ str) -> ColoredString {
        token.bright_yellow()
    }
}
