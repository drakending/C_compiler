use std::cmp;
use crate::diagnostics::Diagnostic;
use crate::text::SourceText;

pub struct DiagnosticsPrinter<'a>{
    text: &'a SourceText,
    diagnostic: &'a [Diagnostic],
}

const PREFIX_LENGTH:usize = 8;

impl <'a> DiagnosticsPrinter<'a>{
    pub fn new(text:&'a SourceText,diagnostic:&'a [Diagnostic]) -> Self {
        Self { text, diagnostic }
    }

    pub fn stringify_diagnostic(&self, diagnostic: &Diagnostic) -> String {
        let line_index = self.text.line_index(diagnostic.span.start);
        let line = self.text.get_line(line_index);
        let line_start = self.text.line_start(line_index);
        let column = diagnostic.span.start-line_start ;
        let prefix_start = cmp::max(0,column as isize - PREFIX_LENGTH as isize) as usize;
        let prefix_end = column;
        let suffix_start = cmp::min(column+diagnostic.span.length(),line.len())+ 1;
        let suffix_end = cmp::min(suffix_start + PREFIX_LENGTH , line.len());
        let prefix = &line[prefix_start..prefix_end];
        let span = &line[prefix_end..suffix_start];
        let suffix = &line[suffix_start..suffix_end];
        let indent = cmp::min(PREFIX_LENGTH,column);
        let arrow_pointers = format!("{:indent$}^{}","","^".repeat(diagnostic.span.length() -1),indent = indent);
        let arrow_line = format!("{:indent$}|","",indent = indent);
        let error_message = format!("{:indent$}+--{}","",diagnostic.message,indent = indent);
        format!(
            "{prefix}{span}{suffix}\n{arrow_pointers}\n{arrow_line}\n{error_message}"
        )
    }
    pub fn print(&self) {
        for diagnostic in self.diagnostic {
            println!("{}", self.stringify_diagnostic(diagnostic));
        }
    }
}