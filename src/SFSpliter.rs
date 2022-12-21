use std::{fs::File, io::Read};

pub fn SourceFileSpliter(assembly_file_path: &str) -> Vec<(usize, String)> {
    let mut sf = File::open(assembly_file_path).unwrap();
    let mut sf_data = String::new();
    sf.read_to_string(&mut sf_data).unwrap();

    let sf_data = sf_data.trim().lines().enumerate().map(
        |(line_num, line)|
        (line_num, line.trim())
    ).collect::<Vec<_>>();

    let mut no_comments_data = vec![];
    for (line_num, line) in sf_data {
        if line == "" {
            continue;
        }
        no_comments_data.push((line_num, String::from(line.split(";").collect::<Vec<_>>()[0].trim())));
    }

    no_comments_data
}