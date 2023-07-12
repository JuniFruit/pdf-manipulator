use lopdf::Document;

use crate::algs::find_occurences;

pub fn page_to_text(doc_file: &Document, page: &u32) -> String {
    let text = doc_file
        .extract_text(&[*page])
        .unwrap_or("".to_ascii_lowercase());

    text
}

pub fn find_pages_with_term(doc_file: &Document, term: &str) -> Vec<(u32, (u32, u16))> {
    let pages = doc_file.get_pages();
    let mut res = vec![];
    for (p, id) in pages {
        println!("Looking for {} in page {}", term, p);
        let text = page_to_text(&doc_file, &p);
        // println!("{:?}", text);
        if text.find(term).is_some() {
            res.push((p, id))
        }
    }
    res
}
