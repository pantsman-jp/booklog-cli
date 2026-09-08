use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path("loan_history.csv")?;
    for result in reader.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}
