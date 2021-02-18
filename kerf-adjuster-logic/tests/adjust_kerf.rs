use kerfadjusterlogic::contour::ContourVecToDxf;
use kerfadjusterlogic::contour::find_endpoints_of_entity;
use kerfadjusterlogic::contour::Contour;
use dxf::Drawing;
use std::collections::HashMap;


#[test]
fn main() {
    // Load DXF
    let drawing =
        Drawing::load_file("../example_dxf.DXF").unwrap();

    println!(
        "There are {} entities in the drawing",
        drawing.entities.len()
    );

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
    let mut i = 0;
    while contours.len() > 0 {
        println!("#####\nIter {}:", i);
        i += 1;
        contours.iter().for_each(|(i, contour)| {
            print!("{}: ", i);
            if let Some(ref endpoints) = contour.end_points {
                print!(
                    "(({:0.4}, {:0.4}), ({:0.4}, {:0.4})): ",
                    endpoints.0.x, endpoints.0.y, endpoints.1.x, endpoints.1.y
                );
            } else {
                print!("closed: ")
            }

            print_endpoints(contour)
        });
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

    println!("there are {} closed contours", finished_contours.len());
    for (_, contour) in finished_contours.iter() {
        print_endpoints(contour);
        println!("\n#######\n");
    }

    // test offsetting contours
    let new_drawing = finished_contours
        .into_iter()
        .map(|(_, c)| c.offset_contour(0.3).unwrap_or(c))
        .collect::<Vec<_>>()
        .to_dxf();
    new_drawing.save_file("./tester.dxf");
}

fn print_endpoints(contour: &Contour) {
    for entity in contour.entities.iter() {
        match find_endpoints_of_entity(entity) {
            Some((start, end)) => {
                let entity_specific_debug = format!("{:?}", entity.specific);
                let entity_name = entity_specific_debug
                    .split("(")
                    .next()
                    .unwrap_or("unknown entity name");
                print!(
                    "({:?} ({:0.4}, {:0.4}), ({:0.4}, {:0.4}))",
                    entity_name, start.x, start.y, end.x, end.y
                );
            }
            None => print!(" None "),
        }
        print!(" -> ");
    }
    print!("\n");
}
