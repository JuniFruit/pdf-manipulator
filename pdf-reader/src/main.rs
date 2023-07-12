use std::env::args;
use std::time::SystemTime;
mod algs;
mod manipulations;
use lopdf::content::{Content, Operation};
use lopdf::{Document, Object, Stream};

use crate::manipulations::reading::page_to_text;
use crate::manipulations::{add_text_annot, extract_and_save_pages, save_pages_with_term};

fn main() {
    let path = args().nth(1).unwrap_or("./files/cv.pdf".to_owned());
    println!("read: {}", path);
    let now = SystemTime::now();

    let mut doc = Document::load(path).unwrap();

    let p = doc.get_pages();

    let f_page_id = p.get(&1).unwrap();

    let page = doc.get_page_content(*f_page_id).unwrap();
    // let res = doc.get_page_resources(*f_page_id);

    // extract_and_save_pages(&doc, &[1], "./").unwrap();

    // println!("{:?}", doc.get_object((11, 0)));
    // println!("{:?}", doc.get_object((4, 0)));
    println!("{:?}", doc.extract_text(&[1]));
    save_pages_with_term(&doc, "API").unwrap();

    // let stream = doc.get_object(p_obj_id).unwrap();

    // println!("{:?}", stream);

    if let Ok(elapsed) = now.elapsed() {
        println!(
            "Time: {}s",
            elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9
        );
    }
}
