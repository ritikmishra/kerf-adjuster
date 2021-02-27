use wasm_bindgen::prelude::*;

use console_error_panic_hook;
use dxf::Drawing;
use std::io::BufReader;
pub mod contour;
use contour::{Contour, ContourVecToDxf};
pub mod errors;

/// Merges the `head` contour with one of the `tail` contours if possible
/// 
/// If the head contour X can be combined with one of the tail contours Y
///     Then it returns a vector containing the combination of X and Y, as well as the remaining uncombined tail contours
/// 
/// If the head contour X cannot be combined with any tail contour
///     Then it returns None
fn collapse_contours_once(head: &Contour, tail: &[Contour]) -> Option<Vec<Contour>> {
    for (i, contour) in tail.iter().enumerate() {
        if let Ok(combined) = head
            .clone()
            .combine_attempt(contour.clone())
        {
            let mut ret = tail.to_vec();
            ret.remove(i);
            ret.push(combined);

            return Some(ret);
        }
    }
    return None;
}


/// Maximizes the number of closed contours in a contour list by combining them
/// 
/// e.g If we have a vector of 4 open contours, where each one is the side of a rectangle, 
/// it will return a vector of 1 closed contour where the  
fn collapse_contours(mut contours: Vec<Contour>) -> Vec<Contour> {
    let mut final_contours = Vec::new();

    loop {
        // Go through the contours
        match contours.as_slice() {
            // Stop if there are no contours left to combine
            [] => break, 

            // Separate the first contour from the remaining ones
            [head, tail @ ..] => {
                match collapse_contours_once(head, tail) {
                    // If we couldn't combine it, `head` must be a complete contour
                    None => {
                        final_contours.push(head.clone());
                        contours = tail.to_vec(); // remove head from contours list
                    },
                    Some(new_contours) => contours = new_contours
                }
            }
        }
    }

    return final_contours;
}

fn drawing_to_contours(drawing: Drawing) -> Vec<Contour> {
    // Convert each DXF entity (arc, circle, text, etc) into a "Contour" which can be more easily manipulated by us
    let contours = drawing
        .entities
        .clone()
        .into_iter()
        .map(Contour::create_from_entity)
        .collect::<Vec<_>>();

    // Partition the contours by whether or not they are open (i.e can be joined to another contour)
    let (contours, mut finished_contours) = contours
        .into_iter()
        .partition::<Vec<_>, _>(|c| c.is_open());

    finished_contours.extend(collapse_contours(contours));

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
        .map(|c| c.offset_contour(offset_amount).unwrap_or(c))
        .collect::<Vec<_>>()
        .to_dxf();

    // return the new dxf
    let mut ret = Vec::new();
    new_drawing.save(&mut ret).unwrap();
    return ret;
}
