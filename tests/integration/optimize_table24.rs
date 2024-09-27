// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2024, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Table, TableStyle, Workbook, XlsxError};

// Create rust_xlsxwriter file to compare against Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet_with_low_memory();

    worksheet.set_column_range_width(2, 5, 10.288)?;

    let table = Table::new().set_style(TableStyle::Medium10);

    worksheet.add_table(2, 2, 12, 5, &table)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_optimize_table24() {
    let test_runner = common::TestRunner::new()
        .set_name("table24")
        .set_function(create_new_xlsx_file)
        .unique("optimize")
        .ignore_worksheet_spans()
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}