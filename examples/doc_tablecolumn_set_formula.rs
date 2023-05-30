// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

//! Example of adding a formula to a column in a worksheet table.

use rust_xlsxwriter::{Table, TableColumn, Workbook, XlsxError};

fn main() -> Result<(), XlsxError> {
    // Create a new Excel file object.
    let mut workbook = Workbook::new();

    // Add a worksheet to the workbook.
    let worksheet = workbook.add_worksheet();

    // Some sample data for the table.
    let items = ["Apples", "Pears", "Bananas", "Oranges"];
    let data = [
        [10000, 5000, 8000, 6000],
        [2000, 3000, 4000, 5000],
        [6000, 6000, 6500, 6000],
        [500, 300, 200, 700],
    ];

    // Write the table data.
    worksheet.write_column(3, 1, items)?;
    worksheet.write_row_matrix(3, 2, data)?;

    // Set the columns widths for clarity.
    for col_num in 1..=6u16 {
        worksheet.set_column_width(col_num, 12)?;
    }

    // Create a new table and configure the columns.
    let mut table = Table::new();

    // Add a structured reference formula to the last column and set the header
    // caption.
    let columns = vec![
        TableColumn::new().set_header("Product"),
        TableColumn::new().set_header("Quarter 1"),
        TableColumn::new().set_header("Quarter 2"),
        TableColumn::new().set_header("Quarter 3"),
        TableColumn::new().set_header("Quarter 4"),
        TableColumn::new()
            .set_header("Totals")
            .set_formula("SUM(Table1[@[Quarter 1]:[Quarter 4]])"),
    ];
    table.set_columns(&columns);

    // Add the table to the worksheet.
    worksheet.add_table(3, 1, 6, 6, &table)?;

    // Save the file to disk.
    workbook.save("tables.xlsx")?;

    Ok(())
}