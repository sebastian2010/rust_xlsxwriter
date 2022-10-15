// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0 Copyright 2022, John McNamara,
// jmcnamara@cpan.org

use rust_xlsxwriter::{Format, Workbook, XlsxBorder, XlsxError};

mod common;

// Test case to test simple formatting.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new(filename);

    let worksheet = workbook.add_worksheet();

    let top_left_bottom = Format::new()
        .set_border_left(XlsxBorder::Thin)
        .set_border_top(XlsxBorder::Thin)
        .set_border_bottom(XlsxBorder::Thin);

    let top_bottom = Format::new()
        .set_border_top(XlsxBorder::Thin)
        .set_border_bottom(XlsxBorder::Thin);

    let top_left = Format::new()
        .set_border_left(XlsxBorder::Thin)
        .set_border_top(XlsxBorder::Thin);

    worksheet.write_string(1, 1, "test", &top_left_bottom)?;
    worksheet.write_string(1, 3, "test", &top_left)?;
    worksheet.write_string(1, 5, "test", &top_bottom)?;

    workbook.close()?;

    Ok(())
}

#[test]
fn test_format12() {
    let test_runner = common::TestRunner::new("format12").initialize();

    _ = create_new_xlsx_file(test_runner.output_file());

    test_runner.assert_eq();
    test_runner.cleanup();
}