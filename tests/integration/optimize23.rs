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
    let bold = Format::new().set_bold();
    let italic = Format::new().set_italic();
    let both = Format::new().set_bold().set_italic();

    // Set the order of the formats.
    workbook.register_format(&bold);
    workbook.register_format(&both);

    // Constant memory worksheet.
    let worksheet = workbook.add_worksheet_with_constant_memory();

    worksheet.set_row_format(1, &bold)?;
    worksheet.set_column_format(2, &italic)?;

    worksheet.write_number(0, 0, 123)?;
    worksheet.write_number(1, 1, 123)?;
    worksheet.write_blank(1, 2, &both)?;
    worksheet.write_number(2, 2, 123)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_optimize23() {
    let test_runner = common::TestRunner::new()
        .set_name("optimize23")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}