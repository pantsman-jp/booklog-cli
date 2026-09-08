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
    let mut args = std::env::args();
    let filepath = match args.nth(1) {
        Some(path) => path,
        None => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Specify the path to the CSV file.",
            )
            .into());
        }
    };
    let command = match args.nth(0) {
        Some(cmd) => cmd,
        None => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Specify the appropriate command.",
            )
            .into());
        }
    };
    let mut reader = csv::Reader::from_path(filepath)?;
    let mut books: Vec<Book> = Vec::new();
    for result in reader.records() {
        let record = result?;
        books.push(convert(&record));
        // println!("{:?}", convert(&record));
    }
    match command.as_str() {
        "list" => {
            for book in books.iter().enumerate() {
                println!("{}. {}", book.0 + 1, book.1.title);
            }
        }
        "search" => {
            let word = match args.nth(0) {
                Some(word) => word,
                None => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "Specify the appropriate word.",
                    )
                    .into());
                }
            };
            for book in books.iter().enumerate() {
                if book.1.title.contains(&word) {
                    println!("{}. {}", book.0 + 1, book.1.title);
                }
            }
        }
        _ => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Unknown command: {}", command),
            )
            .into());
        }
    }
    Ok(())
}
