use self::reading::find_pages_with_term;
use crate::img::PdfImage;
use lopdf::{dictionary, Dictionary, Document, Object};
pub mod reading;

/// Save provided pages as a separate pdf file
pub fn extract_and_save_pages(
    doc_file: &Document,
    pages: &[u32],
    path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Clone document
    let mut doc = doc_file.clone();
    let doc_pages = doc.get_pages();
    let mut pages_to_del: Vec<u32> = vec![];

    for i in 0..doc_pages.len() {
        let curr_p = (i + 1) as u32;
        if !pages.contains(&curr_p) {
            pages_to_del.push(curr_p)
        } else {
            continue;
        }
    }
    doc.delete_pages(&pages_to_del);

    doc.save(format!("{}/{}_saved.pdf", path, pages.len()))?;
    Ok(())
}

/// Adds text annotation to specified rect location
pub fn add_text_annot(
    doc_file: &mut Document,
    rect: Option<Vec<u32>>,
    text: &str,
    page_id: &(u32, u16),
) -> Result<(), Box<dyn std::error::Error>> {
    let rect = if rect.is_some() {
        rect.unwrap()
    } else {
        vec![100, 500, 400, 500]
    };
    let annots_id = doc_file.add_object(dictionary! {
        "Type" => "Annot",
        "Subtype" => "Text",
        "Rect" =>  {let res: Vec<Object> = rect.into_iter().map(|item| item.into()).collect();
        res},
        "Contents" => text,
        "Border" => vec![0.into(), 0.into(), 1.into()]
    });
    let page_obj = doc_file.get_object_mut(*page_id)?;
    let page_dict = page_obj.as_dict_mut()?;
    let annots_entry = (
        Vec::from("Annots".as_bytes()),
        Object::Array(vec![Object::Reference(annots_id)]),
    );
    page_dict.as_hashmap_mut().extend([annots_entry]);
    Ok(())
}

/// Extracts pages with given search query and saves them as a separate pdf file
pub fn save_pages_with_term(doc_file: &Document, term: &str) -> Result<(), Box<&'static str>> {
    let pages: Vec<u32> = find_pages_with_term(doc_file, term)
        .iter()
        .map(|page| page.0)
        .collect();
    println!("Found pages: {:?}", pages);
    if pages.len() == 0 {
        return Err(Box::new("No pages found"));
    }
    extract_and_save_pages(doc_file, &pages, "./").or(Err(Box::new("Error saving pages")))?;
    Ok(())
}

pub fn get_from_dict<'a>(
    doc_file: &'a Document,
    dict: &Dictionary,
    key: &[u8],
) -> Option<&'a Dictionary> {
    let res = dict
        .get(key)
        .and_then(|parent| parent.as_reference())
        .and_then(|id| doc_file.get_dictionary(id))
        .ok();
    res
}
