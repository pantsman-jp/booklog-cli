use std::error::Error;

// タイトル,貸出日,巻情報,著者,出版社,年月情報,資料ID,URL
#[derive(Debug)]
struct Book {
    title: String,
    checkout_date: String,
    vol_info: String,
    author: String,
    publisher: String,
    date_info: String,
    id: String,
    url: String,
}

fn convert(record: &csv::StringRecord) -> Book {
    Book {
        title: record[0].to_string(),
        checkout_date: record[1].to_string(),
        vol_info: record[2].to_string(),
        author: record[3].to_string(),
        publisher: record[4].to_string(),
        date_info: record[5].to_string(),
        id: record[6].to_string(),
        url: record[7].to_string(),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path("loan_history.csv")?;
    for result in reader.records() {
        let record = result?;
        println!("{:?}", convert(&record));
    }
    Ok(())
}
