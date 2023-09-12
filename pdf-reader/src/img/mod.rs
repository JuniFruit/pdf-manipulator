use crate::manipulations::get_from_dict;
use lopdf::{Dictionary, Document, Object};
use pdf_extract::OutputError;

#[derive(Debug, Clone)]
pub struct PdfImage {
    name: String,
    dict: Dictionary,
    data: Vec<u8>,
}

impl PdfImage {
    pub fn new(name: &str, dict: &Dictionary, data: Vec<u8>) -> Self {
        Self {
            name: name.to_string(),
            dict: dict.clone(),
            data,
        }
    }
}

pub fn get_img_from_page(doc_file: &Document, page: &u32) -> Result<Vec<PdfImage>, OutputError> {
    let p_id = doc_file.get_pages().get(page).unwrap().clone();
    let p_dict = doc_file.get_object(p_id)?.as_dict().unwrap();
    let resources = get_from_dict(doc_file, p_dict, b"Resources").unwrap();
    let x_objects = resources.get(b"XObject").unwrap().as_dict().unwrap();
    let img_objects: Vec<&Object> = x_objects
        .as_hashmap()
        .iter()
        .map(|img| doc_file.get_object(img.1.as_reference().unwrap()).unwrap())
        .collect();

    let mut raw_imgs = vec![];

    for img in &img_objects {
        let data = img.as_stream().unwrap();
        let name = data.dict.get(b"Name").unwrap().as_name_str().unwrap();
        let dict = &data.dict;
        raw_imgs.push(PdfImage::new(
            &format!("p_{}_{}", page, name),
            dict,
            data.content.clone(),
        ))
    }
    // println!("{:?}", (&raw_imgs[0].data.clone()));

    // println!("{:?}", raw_imgs);

    Ok(raw_imgs)
}
