// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2024, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Format, Workbook, XlsxError};

// Create rust_xlsxwriter file to compare against Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();
    let format = Format::new().set_background_color("#FFFF00");

    // Add the data, unformatted.
    worksheet.write(2, 1, 123)?;
    worksheet.write(4, 2, 123)?;
    worksheet.write(7, 1, 123)?;
    worksheet.write(8, 3, 123)?;

    // Add the formatting in ranges.
    worksheet.set_cell_range_format(2, 1, 2, 1, &format)?;
    worksheet.set_cell_range_format(4, 1, 4, 3, &format)?;
    worksheet.set_cell_range_format(7, 1, 8, 3, &format)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_overlay02() {
    let test_runner = common::TestRunner::new()
        .set_name("overlay02")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
