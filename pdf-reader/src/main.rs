use std::env::args;
use std::time::SystemTime;
mod algs;
mod manipulations;
use lopdf::content::{Content, Operation};
use lopdf::encryption::decrypt_object;
use lopdf::{Dictionary, Document, Object, Stream};

use crate::manipulations::reading::page_to_text;
use crate::manipulations::{add_text_annot, extract_and_save_pages, save_pages_with_term};

fn main() {
    let path = args().nth(1).unwrap_or("./files/big.pdf".to_owned());
    println!("read: {}", path);
    let now = SystemTime::now();
    let bytes = std::fs::read(&path).unwrap();
    println!("{:?}", pdf_extract::extract_text_from_mem(&bytes).unwrap());
    let mut doc = Document::load(path).unwrap();

    let p = doc.get_pages();

    let f_page_id = p.get(&1).unwrap();

    let page = doc.get_page_content(*f_page_id).unwrap();
    // let res = doc.get_page_resources(*f_page_id);
    // println!("{:?}", Content::decode(&page));0
    // extract_and_save_pages(&doc, &[1], "./").unwrap();
    output(&doc, f_page_id);
    // let mut fonts = doc.get_page_fonts(*f_page_id);
    // let key = b"ToUnicode".to_vec();
    // let to_unicode_id = fonts.first_entry().unwrap().get().as_hashmap().get(&key);
    // println!("{:?}", doc.extract_text(&[1]));
    // println!("{:?}", fonts);
    // println!(
    //     "{:?}",
    //     doc.get_object(to_unicode_id.unwrap().as_reference().unwrap())
    // );
    // // save_pages_with_term(&doc, "API").unwrap();
    // let obj = doc.get_object((317, 0)).unwrap();
    // let stream = obj.as_stream();
    // println!(
    //     "{:#?}",
    //     String::from_utf8(stream.as_ref().unwrap().decompressed_content().unwrap())
    // );

    if let Ok(elapsed) = now.elapsed() {
        println!(
            "Time: {}s",
            elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9
        );
    }
}

fn output(doc_file: &Document, page_id: &(u32, u16)) {
    let mut writable = String::new();
    let page_dict = doc_file.get_object(*page_id).unwrap().as_dict().unwrap();
    let resources = get_inherited(&doc_file, page_dict, b"Resources")
        .unwrap_or(Object::Dictionary(Dictionary::new()));
    println!("{:?}", resources);

    process_stream(
        doc_file,
        &mut writable,
        &resources,
        doc_file.get_page_content(*page_id).unwrap_or(vec![]),
    )
}

fn process_stream(doc_file: &Document, writable: &mut String, resources: &Object, data: Vec<u8>) {
    let content = Content::decode(&data).unwrap();

    for operation in content.operations {
        match operation.operator.as_ref() {
            // "BT" => {
            //     tlm = Transform2D::identity();
            //     gs.ts.tm = tlm;
            // }
            // "ET" => {
            //     tlm = Transform2D::identity();
            //     gs.ts.tm = tlm;
            // }
            // "cm" => {
            //     assert!(operation.operands.len() == 6);
            //     let m = Transform2D::row_major(
            //         as_num(&operation.operands[0]),
            //         as_num(&operation.operands[1]),
            //         as_num(&operation.operands[2]),
            //         as_num(&operation.operands[3]),
            //         as_num(&operation.operands[4]),
            //         as_num(&operation.operands[5]),
            //     );
            //     gs.ctm = gs.ctm.pre_transform(&m);
            //     println!("matrix {:?}", gs.ctm);
            // }
            // "CS" => {
            //     let name = operation.operands[0].as_name().unwrap();
            //     gs.stroke_colorspace = make_colorspace(doc, name, resources);
            // }
            // "cs" => {
            //     let name = operation.operands[0].as_name().unwrap();
            //     gs.fill_colorspace = make_colorspace(doc, name, resources);
            // }
            // "SC" | "SCN" => {
            //     gs.stroke_color = match gs.stroke_colorspace {
            //         ColorSpace::Pattern => {
            //             println!("unhandled pattern color");
            //             Vec::new()
            //         }
            //         _ => operation.operands.iter().map(|x| as_num(x)).collect(),
            //     };
            // }
            // "sc" | "scn" => {
            //     gs.fill_color = match gs.fill_colorspace {
            //         ColorSpace::Pattern => {
            //             println!("unhandled pattern color");
            //             Vec::new()
            //         }
            //         _ => operation.operands.iter().map(|x| as_num(x)).collect(),
            //     };
            // }
            // "G" | "g" | "RG" | "rg" | "K" | "k" => {
            //     println!("unhandled color operation {:?}", operation);
            // }
            "TJ" => match operation.operands[0] {
                Object::Array(ref array) => {
                    for e in array {
                        match e {
                            &Object::String(ref s, _) => {
                                // show_text(&mut gs, s, &tlm, &flip_ctm, output)?;
                                println!("{:?}", s);
                            }
                            &Object::Integer(i) => {
                                // let ts = &mut gs.ts;
                                let w0 = 0.;
                                let tj = i as f64;
                                let ty = 0.;
                                // let tx = ts.horizontal_scaling * ((w0 - tj / 1000.) * ts.font_size);
                                // ts.tm = ts
                                //     .tm
                                // .pre_transform(&Transform2D::create_translation(tx, ty));
                                println!("adjust text by: {}", i);
                            }
                            // &Object::Real(i) => {
                            //     let ts = &mut gs.ts;
                            //     let w0 = 0.;
                            //     let tj = i as f64;
                            //     let ty = 0.;
                            //     let tx = ts.horizontal_scaling * ((w0 - tj / 1000.) * ts.font_size);
                            //     ts.tm = ts
                            //         .tm
                            //         .pre_transform(&Transform2D::create_translation(tx, ty));
                            //     println!("adjust text by: {} {:?}", i, ts.tm);
                            // }
                            _ => {
                                println!("kind of {:?}", e);
                            }
                        }
                    }
                }
                _ => {}
            },
            "Tj" => match operation.operands[0] {
                Object::String(ref s, _) => {
                    // show_text(&mut gs, s, &tlm, &flip_ctm, output)?;
                }
                _ => {
                    panic!("unexpected Tj operand {:?}", operation)
                }
            },
            // "Tc" => {
            //     gs.ts.character_spacing = as_num(&operation.operands[0]);
            // }
            // "Tw" => {
            //     gs.ts.word_spacing = as_num(&operation.operands[0]);
            // }
            // "Tz" => {
            //     gs.ts.horizontal_scaling = as_num(&operation.operands[0]) / 100.;
            // }
            // "TL" => {
            //     gs.ts.leading = as_num(&operation.operands[0]);
            // }
            // "Tf" => {
            //     let fonts: &Dictionary = get(&doc, resources, b"Font");
            //     let name = operation.operands[0].as_name().unwrap();
            //     let font = font_table
            //         .entry(name.to_owned())
            //         .or_insert_with(|| make_font(doc, get::<&Dictionary>(doc, fonts, name)))
            //         .clone();
            //     {
            //         /*let file = font.get_descriptor().and_then(|desc| desc.get_file());
            //         if let Some(file) = file {
            //             let file_contents = filter_data(file.as_stream().unwrap());
            //             let mut cursor = Cursor::new(&file_contents[..]);
            //             //let f = Font::read(&mut cursor);
            //             //println!("font file: {:?}", f);
            //         }*/
            //     }
            //     gs.ts.font = Some(font);

            //     gs.ts.font_size = as_num(&operation.operands[1]);
            // }
            // "Ts" => {
            //     gs.ts.rise = as_num(&operation.operands[0]);
            // }
            // "Tm" => {
            //     assert!(operation.operands.len() == 6);
            //     tlm = Transform2D::row_major(
            //         as_num(&operation.operands[0]),
            //         as_num(&operation.operands[1]),
            //         as_num(&operation.operands[2]),
            //         as_num(&operation.operands[3]),
            //         as_num(&operation.operands[4]),
            //         as_num(&operation.operands[5]),
            //     );
            //     gs.ts.tm = tlm;
            //     println!("Tm: matrix {:?}", gs.ts.tm);
            //     output.end_line()?;
            // }
            // "Td" => {
            //     /* Move to the start of the next line, offset from the start of the current line by (tx , ty ).
            //       tx and ty are numbers expressed in unscaled text space units.
            //       More precisely, this operator performs the following assignments:
            //     */
            //     assert!(operation.operands.len() == 2);
            //     let tx = as_num(&operation.operands[0]);
            //     let ty = as_num(&operation.operands[1]);
            //     println!("translation: {} {}", tx, ty);

            //     tlm = tlm.pre_transform(&Transform2D::create_translation(tx, ty));
            //     gs.ts.tm = tlm;
            //     println!("Td matrix {:?}", gs.ts.tm);
            //     output.end_line()?;
            // }

            // "TD" => {
            //     /* Move to the start of the next line, offset from the start of the current line by (tx , ty ).
            //       As a side effect, this operator sets the leading parameter in the text state.
            //     */
            //     assert!(operation.operands.len() == 2);
            //     let tx = as_num(&operation.operands[0]);
            //     let ty = as_num(&operation.operands[1]);
            //     println!("translation: {} {}", tx, ty);
            //     gs.ts.leading = -ty;

            //     tlm = tlm.pre_transform(&Transform2D::create_translation(tx, ty));
            //     gs.ts.tm = tlm;
            //     println!("TD matrix {:?}", gs.ts.tm);
            //     output.end_line()?;
            // }

            // "T*" => {
            //     let tx = 0.0;
            //     let ty = -gs.ts.leading;

            //     tlm = tlm.pre_transform(&Transform2D::create_translation(tx, ty));
            //     gs.ts.tm = tlm;
            //     println!("T* matrix {:?}", gs.ts.tm);
            //     output.end_line()?;
            // }
            // "q" => {
            //     gs_stack.push(gs.clone());
            // }
            // "Q" => {
            //     let s = gs_stack.pop();
            //     if let Some(s) = s {
            //         gs = s;
            //     } else {
            //         println!("No state to pop");
            //     }
            // }
            // "gs" => {
            //     let ext_gstate: &Dictionary = get(doc, resources, b"ExtGState");
            //     let name = operation.operands[0].as_name().unwrap();
            //     let state: &Dictionary = get(doc, ext_gstate, name);
            //     apply_state(&mut gs, state);
            // }
            // "i" => {
            //     println!(
            //         "unhandled graphics state flattness operator {:?}",
            //         operation
            //     );
            // }
            // "w" => {
            //     gs.line_width = as_num(&operation.operands[0]);
            // }
            // "J" | "j" | "M" | "d" | "ri" => {
            //     println!("unknown graphics state operator {:?}", operation);
            // }
            // "m" => path.ops.push(PathOp::MoveTo(
            //     as_num(&operation.operands[0]),
            //     as_num(&operation.operands[1]),
            // )),
            // "l" => path.ops.push(PathOp::LineTo(
            //     as_num(&operation.operands[0]),
            //     as_num(&operation.operands[1]),
            // )),
            // "c" => path.ops.push(PathOp::CurveTo(
            //     as_num(&operation.operands[0]),
            //     as_num(&operation.operands[1]),
            //     as_num(&operation.operands[2]),
            //     as_num(&operation.operands[3]),
            //     as_num(&operation.operands[4]),
            //     as_num(&operation.operands[5]),
            // )),
            // "v" => {
            //     let (x, y) = path.current_point();
            //     path.ops.push(PathOp::CurveTo(
            //         x,
            //         y,
            //         as_num(&operation.operands[0]),
            //         as_num(&operation.operands[1]),
            //         as_num(&operation.operands[2]),
            //         as_num(&operation.operands[3]),
            //     ))
            // }
            // "y" => path.ops.push(PathOp::CurveTo(
            //     as_num(&operation.operands[0]),
            //     as_num(&operation.operands[1]),
            //     as_num(&operation.operands[2]),
            //     as_num(&operation.operands[3]),
            //     as_num(&operation.operands[2]),
            //     as_num(&operation.operands[3]),
            // )),
            // "h" => path.ops.push(PathOp::Close),
            // "re" => path.ops.push(PathOp::Rect(
            //     as_num(&operation.operands[0]),
            //     as_num(&operation.operands[1]),
            //     as_num(&operation.operands[2]),
            //     as_num(&operation.operands[3]),
            // )),
            // "s" | "f*" | "B" | "B*" | "b" => {
            //     println!("unhandled path op {:?}", operation);
            // }
            // "S" => {
            //     output.stroke(&gs.ctm, &gs.stroke_colorspace, &gs.stroke_color, &path)?;
            //     path.ops.clear();
            // }
            // "F" | "f" => {
            //     output.fill(&gs.ctm, &gs.fill_colorspace, &gs.fill_color, &path)?;
            //     path.ops.clear();
            // }
            // "W" | "w*" => {
            //     println!("unhandled clipping operation {:?}", operation);
            // }
            // "n" => {
            //     println!("discard {:?}", path);
            //     path.ops.clear();
            // }
            // "BMC" | "BDC" => {
            //     mc_stack.push(operation);
            // }
            // "EMC" => {
            //     mc_stack.pop();
            // }
            // "Do" => {
            //     // `Do` process an entire subdocument, so we do a recursive call to `process_stream`
            //     // with the subdocument content and resources
            //     let xobject: &Dictionary = get(&doc, resources, b"XObject");
            //     let name = operation.operands[0].as_name().unwrap();
            //     let xf: &Stream = get(&doc, xobject, name);
            //     let resources = maybe_get_obj(&doc, &xf.dict, b"Resources")
            //         .and_then(|n| n.as_dict().ok())
            //         .unwrap_or(resources);
            //     let contents = get_contents(xf);
            //     self.process_stream(&doc, contents, resources, &media_box, output, page_num)?;
            // }
            _ => {
                println!(
                    "unknown operation {:?}, decoded: {:?}",
                    operation,
                    operation
                        .operands
                        .iter()
                        .map(|item| item.as_array())
                        .collect::<Vec<_>>()
                );
            }
        }
    }
}

fn get_inherited<'a>(doc_file: &Document, dict: &'a Dictionary, key: &[u8]) -> Option<Object> {
    let obj = get(dict, key);
    if obj.is_some() {
        obj
    } else {
        let parent = dict
            .get(b"Parent")
            .and_then(|parent| parent.as_reference())
            .and_then(|id| doc_file.get_dictionary(id))
            .unwrap();
        get_inherited(doc_file, parent, key)
    }
}

fn get(dict: &Dictionary, key: &[u8]) -> Option<Object> {
    let obj = dict.get(key).ok();
    if obj.is_some() {
        Some(obj.unwrap().clone())
    } else {
        None
    }
}

fn render_text() {}
