use crate::img::PdfImage;

use super::get_from_dict;

use lopdf::{Document, Object};
use pdf_extract::{output_page, OutputError, PlainTextOutput};

pub fn page_to_text(doc_file: &Document, page: &u32) -> Result<String, OutputError> {
    let mut s = String::new();
    let mut output = PlainTextOutput::new(&mut s);
    output_page(doc_file, &mut output, page)?;
    Ok(s.clone())
}

pub fn find_pages_with_term(doc_file: &Document, term: &str) -> Vec<(u32, (u32, u16))> {
    let pages = doc_file.get_pages();
    let mut res = vec![];
    for (p, id) in pages {
        println!("Looking for {} in page {}", term, p);
        let text = page_to_text(&doc_file, &p).unwrap_or("".to_string());
        println!("{:?}", text);
        if text.find(term).is_some() {
            res.push((p, id))
        }
    }
    res
}
