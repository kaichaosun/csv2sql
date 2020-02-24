// 文章标题,发布时间,阅读数,赞数,评论数,收藏数,完整阅读率,链接
// Substrate存储数据类型概览,2020-01-20,704,1,0,0,13.0%,https://zhuanlan.zhihu.com/p/103510959
// 抛硬币游戏(二)：编写测试和UI,2019-10-03,240,0,0,0,33.0%,https://zhuanlan.zhihu.com/p/85070687
// Substrate应用 - 抛硬币游戏（一）,2019-09-17,463,2,2,1,31.0%,https://zhuanlan.zhihu.com/p/76815437
// 使用Substrate搭建你的第一条区块链,2019-07-10,619,2,0,4,34.0%,https://zhuanlan.zhihu.com/p/67580341
// Substrate开发资源,2019-05-28,708,2,0,6,25.0%,https://zhuanlan.zhihu.com/p/67203196


use csv::Reader;
fn main() {
    let file_path = "./zhihu_articles.csv";
    let csv_content = read_csv(file_path).unwrap();
    println!("csv: {:?}", csv_content);

    let generated_sql = generate_sql(&csv_content);
    println!("sql: {}", generated_sql);
}

// Define your own header here
#[derive(Debug)]
struct Header(String, String, String, String, String, String, String, String);
// Define your own data type here
#[derive(Debug)]
struct Data(String, String, u32, u32, u32, u32, String, String);

fn read_csv(file_path: &str) -> Result<(Header, Vec<Data>), &'static str> {
    let mut reader = Reader::from_path(file_path).map_err(|e| {
        println!("Error from reading csv: {}", e);
        "Reading csv failed."
    })?;

    let header = Header(
        "title".to_string(),
        "publish_date".to_string(),
        "read_num".to_string(),
        "like_num".to_string(),
        "comment_num".to_string(),
        "collection_num".to_string(),
        "complete_read_rate".to_string(),
        "link".to_string(),
    );

    let mut all_data: Vec<Data> = Vec::new();

    for item in reader.records() {
        let record = item.map_err(|_| "Get each record error")?;
        let data = Data(
            String::from(&record[0]),
            String::from(&record[1]),
            record[2].parse().unwrap(),
            record[3].parse().unwrap(),
            record[4].parse().unwrap(),
            record[5].parse().unwrap(),
            String::from(&record[6]),
            String::from(&record[7]),
        );
        all_data.push(data);
    };

    Ok((header, all_data))
}

fn generate_sql(csv_data: &(Header, Vec<Data>)) -> String {
    let (header, data) = csv_data;
    let base_sql = format!(
        "INSERT INTO zhihu_articles_performance ( {}, {}, {}, {}, {}, {}, {}, {} ) VALUES ",
        header.0,
        header.1,
        header.2,
        header.3,
        header.4,
        header.5,
        header.6,
        header.7,
    );

    let mut values_sql = "".to_string();
    for item in data.iter() {
        let value_part = format!(
            "( '{}', '{}', {}, {}, {}, {}, '{}', '{}' ),",
            item.0,
            item.1,
            item.2,
            item.3,
            item.4,
            item.5,
            item.6,
            item.7,
        );
        values_sql.push_str(&value_part);
    }

    let s = format!("{}{}", base_sql, values_sql);
    let len = s.len();
    s[0..len-1].to_string()
}