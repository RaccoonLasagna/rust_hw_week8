use std::fs::File;
use std::io::Write;
use webbrowser;

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
                    let mut file = File::create("output.html")?;
                    file.write_all(
b"<style>
    table, td {
    border: 1px solid #000000;
    border-collapse: collapse;
}
</style>
<table>
  <tr>
    <th>X</th>
    <th>Y</th>
  </tr>",)?;
                    for i in out_list{
                        let xcoord = i.0*(i.1).cos();
                        let ycoord = i.0*(i.1).sin();
                        write!(file,"\n\t<tr>\n\t\t<td>{}</td>\n\t\t<td>{}</td>\n\t</tr>", xcoord, ycoord)?;
                    }
                    write!(file,"\n</table>")?;
                    webbrowser::open("output.html");
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
