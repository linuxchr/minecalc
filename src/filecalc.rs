use super::calc::boxes_from_items;
use ascii_table::AsciiTable;
use std::fmt::Display;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

struct Row {
    name: String,
    total: f64,
    missing: f64,
    available: f64,
}

struct RowWithBoxes {
    name: String,
    total: f64,
    totalboxes: f64,
    missing: f64,
    missingboxes: f64,
    available: f64,
    availableboxes: f64,
}

pub fn parse_file(
    input_path: String,
    output_path: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let (rows, name) = format_content(fs::read_to_string(input_path)?)?;
    let rows_with_boxes: Vec<RowWithBoxes> = rows_to_rows_with_boxes(rows);
    write_to_file(output_path, rows_with_boxes, name)?;
    Ok(())
}

fn format_content(file: String) -> Result<(Vec<Row>, String), Box<dyn std::error::Error>> {
    let mut rows: Vec<Row> = vec![];
    let mut name: String = String::from("");
    for row in file.lines() {
        let content: Vec<&str> = row.split("|").collect();
        if content.len() == 6 {
            if content[2].to_ascii_lowercase().trim() == String::from("total") {
                continue;
            }
            let name = content[1].trim().to_string();
            let total = content[2].trim().parse::<f64>()?;
            let missing = content[3].trim().parse::<f64>()?;
            let available = content[4].trim().parse::<f64>()?;
            rows.push(Row {
                name,
                total,
                missing,
                available,
            });
        } else if content.len() == 3 {
            name = content[1].trim().to_string();
        }
    }
    return Ok((rows, name));
}

fn rows_to_rows_with_boxes(rows: Vec<Row>) -> Vec<RowWithBoxes> {
    let mut rows_with_boxes: Vec<RowWithBoxes> = vec![];
    for row in rows {
        let name = row.name.to_string();
        let total = row.total;
        let missing = row.missing;
        let available = row.available;
        let totalboxes: f64 = boxes_from_items(total, &name);
        let missingboxes: f64 = boxes_from_items(missing, &name);
        let availableboxes: f64 = boxes_from_items(available, &name);
        rows_with_boxes.push(RowWithBoxes {
            name,
            total,
            totalboxes,
            missing,
            missingboxes,
            available,
            availableboxes,
        });
    }
    rows_with_boxes
}

fn write_to_file(
    path: String,
    rows: Vec<RowWithBoxes>,
    title: String,
) -> Result<(), std::io::Error> {
    let mut file = File::create(path)?;
    let mut data: Vec<Vec<&dyn Display>> = vec![];
    for row in rows.iter() {
        data.push(vec![
            &row.name,
            &row.total,
            &row.totalboxes,
            &row.missing,
            &row.missingboxes,
            &row.available,
            &row.availableboxes,
        ]);
    }
    let mut table = AsciiTable::default();
    table.set_max_width(500);
    table.column(0).set_header("Name");
    table.column(1).set_header("Total");
    table.column(2).set_header("Total boxes");
    table.column(3).set_header("Missing");
    table.column(4).set_header("Missing boxes");
    table.column(5).set_header("Available");
    table.column(6).set_header("Available boxes");
    let mut output = format!("{title}\n{}", table.format(data));
    file.write_all(output.as_mut().as_bytes())?;
    Ok(())
}
