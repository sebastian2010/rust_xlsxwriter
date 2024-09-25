// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2024, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Note, Workbook, XlsxError};

// Create rust_xlsxwriter file to compare against Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet_with_constant_memory();
    worksheet.set_default_note_author("John");

    worksheet.write(0, 0, "Foo")?;

    let note = Note::new("Some text").add_author_prefix(false);
    worksheet.insert_note(1, 1, &note)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_optimize13() {
    let test_runner = common::TestRunner::new()
        .set_name("optimize13")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}