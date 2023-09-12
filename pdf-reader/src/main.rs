use std::fs;
use std::path::Path;
use std::time::SystemTime;
mod algs;
mod img;
mod manipulations;
use crate::{img::get_img_from_page, manipulations::save_pages_with_term};
use lopdf::{Dictionary, Document, Object, Stream};

fn main() {
    let p = Path::new("pdf-reader/files/declaration.pdf");
    println!("read: {:?}", p);
    let now = SystemTime::now();
    let mut doc = Document::load(p).unwrap();

    let p = doc.get_pages();

    let f_page_id = p.get(&1).unwrap();

    let page = doc.get_page_content(*f_page_id).unwrap();
    // let img_obj = &doc.get_object((23, 0));
    get_img_from_page(&doc, &1);
    // println!("{:?}", img_obj);

    // save_pages_with_term(&doc, "lingvist").unwrap();

    if let Ok(elapsed) = now.elapsed() {
        println!(
            "Time: {}s",
            elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9
        );
    }
}
