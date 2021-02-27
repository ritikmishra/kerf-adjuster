use wasm_bindgen::prelude::*;

use console_error_panic_hook;
use dxf::Drawing;
use std::io::BufReader;
use std::{
    collections::HashMap,
};
pub mod contour;
use contour::{Contour, ContourVecToDxf};
pub mod errors;

fn drawing_to_contours(drawing: Drawing) -> HashMap<usize, Contour> {
    // Convert each DXF entity (arc, circle, text, etc) into a "Contour" which can be more easily manipulated by us
    let contours = drawing
        .entities
        .clone()
        .into_iter()
        .map(Contour::create_from_entity)
        .collect::<Vec<_>>();

    // Partition the contours by whether or not they are open (i.e can be joined to another contour)
    let (mut contours, mut finished_contours) = contours
        .into_iter()
        .enumerate()
        .partition::<HashMap<usize, _>, _>(|(_, c)| c.is_open());

    // Group the contours with each other.end_points.is_some()
    // TODO: handle unclosed contours
    while contours.len() > 0 {
        'each_a: for (i, current_contour) in contours.iter() {
            loop {
                for (j, other_contour) in contours.iter() {
                    let (i, j) = (*i, *j);
                    if i != j {
                        if let Ok(combined) = current_contour
                            .clone()
                            .combine_attempt(other_contour.clone())
                        {
                            // We will combine other_contour into this_contour
                            // Remove other_contour from contours map
                            contours.remove(&j);

                            // Check if the combined result is open or closed, and
                            // put it in its place accordingly
                            if combined.end_points.is_some() {
                                contours.insert(i, combined);
                            } else {
                                contours.remove(&i);
                                finished_contours.insert(i, combined);
                            }

                            // Restart the contour merging process from the beginning
                            // helps with ensuring consistent behavior
                            break 'each_a;
                        }
                    }
                }
            }
        }
    }

    return finished_contours;
}

#[wasm_bindgen]
pub fn offset_drawing(drawing_bytes: &[u8], offset_amount: f64) -> Vec<u8> {
    let mut bufreader = BufReader::new(drawing_bytes);
    let drawing = Drawing::load(&mut bufreader).unwrap();
    let drawing_contours = drawing_to_contours(drawing);

    // offset the contours
    let new_drawing = drawing_contours
        .into_iter()
        .map(|(_, c)| c.offset_contour(offset_amount).unwrap_or(c))
        .collect::<Vec<_>>()
        .to_dxf();

    // return the new dxf
    let mut ret = Vec::new();
    new_drawing.save(&mut ret).unwrap();
    return ret;
}
