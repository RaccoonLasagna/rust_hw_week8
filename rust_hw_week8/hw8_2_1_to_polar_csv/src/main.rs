use std::fs::File;
use std::io::Write;

fn read_from_file(path: &str) -> std::io::Result<()>{
    let mut out_list = Vec::new();
    let reader = csv::Reader::from_path(path);
    match reader {
        Ok(mut read) => {
            let headers = read.headers();
            match headers {
                Ok(header) => {
                    let header_0: f64 = header[0].trim().parse().unwrap();
                    let header_1: f64 = header[1].trim().parse().unwrap();
                    out_list.push((header_0, header_1));
                    for record in read.records() {
                        if let Ok(rec) = record {
                            let v1: f64 = rec[0].trim().parse().unwrap();
                            let v2: f64 = rec[1].trim().parse().unwrap();
                            out_list.push((v1, v2));
                        }
                    }
                    println!("{:?}", out_list);
                    let mut file = File::create("output.csv")?;
                    for i in out_list{
                        let distance = ((i.0).powf(2.) + (i.1).powf(2.)).sqrt();
                        let theta = (i.1/i.0).atan();
                        write!(file, "{}, {}\n",distance, theta)?;
                    }
                    Ok(())
                }
                Err(_) => panic!("oh no"),
            }
        }
        Err(_) => panic!("oh no"),
    }
}

fn main() {
    read_from_file("input.csv");
}
