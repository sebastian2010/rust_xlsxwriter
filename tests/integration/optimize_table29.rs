// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2024, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Image, Table, Workbook, XlsxError};

// Create rust_xlsxwriter file to compare against Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet_with_low_memory();

    worksheet.set_column_range_width(2, 5, 10.288)?;

    let table = Table::new();
    worksheet.add_table(2, 2, 12, 5, &table)?;

    let image = Image::new("tests/input/images/red.png")?.set_alt_text("red.png");

    worksheet.insert_image(0, 0, &image)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_optimize_table29() {
    let test_runner = common::TestRunner::new()
        .set_name("table29")
        .set_function(create_new_xlsx_file)
        .unique("optimize")
        .ignore_worksheet_spans()
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}