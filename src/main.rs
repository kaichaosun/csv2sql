use csv::Reader;
use std::env;
fn main() {
    let file_path = format!("./{}", env::var("CSV_PATH").unwrap());
    let csv_content = read_csv(&file_path).unwrap();
    println!("csv: {:?}", csv_content);

    let generated_sql = generate_sql(&csv_content);
    println!("sql: {}", generated_sql);
}

// Define your own header here
#[derive(Debug)]
struct Header(String, String, String, String);
// Define your own data type here
#[derive(Debug)]
struct Data(String, String, u32, String);

fn read_csv(file_path: &str) -> Result<(Header, Vec<Data>), &'static str> {
    let mut reader = Reader::from_path(file_path).map_err(|e| {
        println!("Error from reading csv: {}", e);
        "Reading csv failed."
    })?;

    let header = Header(
        "title".to_string(),
        "publish_date".to_string(),
        "read_num".to_string(),
        "complete_rate".to_string(),
    );

    let mut all_data: Vec<Data> = Vec::new();

    for item in reader.records() {
        let record = item.map_err(|_| "Get each record error")?;
        let data = Data(
            String::from(&record[0]),
            String::from(&record[1]),
            record[2].parse().unwrap(),
            String::from(&record[7]),
        );
        all_data.push(data);
    };

    Ok((header, all_data))
}

fn generate_sql(csv_data: &(Header, Vec<Data>)) -> String {
    let (header, data) = csv_data;
    let base_sql = format!(
        "INSERT INTO wechat_tech_articles ( {}, {}, {}, {} ) VALUES ",
        header.0,
        header.1,
        header.2,
        header.3,
    );

    let mut values_sql = "".to_string();
    for item in data.iter() {
        let value_part = format!(
            "( '{}', '{}', {}, '{}' ),",
            item.0,
            item.1,
            item.2,
            item.3,
        );
        values_sql.push_str(&value_part);
    }

    let s = format!("{}{}", base_sql, values_sql);
    let len = s.len();
    s[0..len-1].to_string()
}
