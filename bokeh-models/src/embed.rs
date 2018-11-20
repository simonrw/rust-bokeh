use crate::document::Document;
use crate::errors::Result;

pub fn file_html(_document: &Document, _title: &str) -> Result<String> {
    Ok("".to_string())
}
