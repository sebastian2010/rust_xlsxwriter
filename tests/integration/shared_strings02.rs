// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2024, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Workbook, XlsxError};

// Test to demonstrate escaping XML escapes in Excel shared strings.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    worksheet.write_string(0, 0, "_")?;
    worksheet.write_string(1, 0, "_x")?;
    worksheet.write_string(2, 0, "_x0")?;
    worksheet.write_string(3, 0, "_x00")?;
    worksheet.write_string(4, 0, "_x000")?;
    worksheet.write_string(5, 0, "_x0000")?;
    worksheet.write_string(6, 0, "_x0000_")?;
    worksheet.write_string(7, 0, "_x005F_")?;
    worksheet.write_string(8, 0, "_x000G_")?;
    worksheet.write_string(9, 0, "_X0000_")?;
    worksheet.write_string(10, 0, "_x000a_")?;
    worksheet.write_string(11, 0, "_x000A_")?;
    worksheet.write_string(12, 0, "_x0000__x0000_")?;
    worksheet.write_string(13, 0, "__x0000__")?;
    worksheet.write_string(14, 0, "_x597D_")?;
    worksheet.write_string(15, 0, "_x597d_")?;
    worksheet.write_string(16, 0, "_x597G_")?;
    worksheet.write_string(17, 0, "_x_x_x")?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_shared_strings02() {
    let test_runner = common::TestRunner::new()
        .set_name("shared_strings02")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
