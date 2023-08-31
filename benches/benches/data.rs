use std::fs;
use std::io::Read;

pub struct Data
{
    pub name: String,
    pub source: Vec<u8>,
}

/// данные на разных языках для тестов
pub fn validation_data_files() -> Vec<Data>
{
    let dir = fs::read_dir("./data").unwrap();

    let mut data = vec![];

    for entry in dir {
        let entry = entry.unwrap();

        let path = entry.path();
        let path = path.to_str().unwrap();

        data.push(Data {
            name: get_name(path).to_owned(),
            source: read(&path, 2),
        });
    }

    data.sort_by(|a, b| a.name.cmp(&b.name));

    data
}

/// прочитать файл n раз
fn read(source: &str, times: usize) -> Vec<u8>
{
    let mut file = fs::File::open(source).unwrap();
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer).unwrap();

    let mut buffer_10 = Vec::new();

    for _ in 0 ..= times {
        buffer_10.extend_from_slice(&buffer);
    }

    buffer_10
}

/// вырезать из полного пути к файлу его название, без формата
fn get_name(filename: &str) -> &str
{
    let (_, name) = filename.trim_end_matches(".txt").rsplit_once('/').unwrap();

    name
}
