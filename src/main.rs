// [dependencies]
// umya-spreadsheet = "0.7.2"

use umya_spreadsheet::*;

fn main() {
    // READER
    let path = std::path::Path::new("./test.xlsx");
    let mut book = reader::xlsx::read(path).unwrap();

    // read value in cell A1
    let a_one_value = book.get_sheet_by_name("Testsheet").unwrap().get_value("A1");
    println!("A1 = {}", a_one_value);
}
