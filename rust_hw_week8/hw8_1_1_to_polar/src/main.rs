#[derive(Debug, Clone, PartialEq)]
struct Point{
    x: f64,
    y: f64,
}

#[derive(Debug, Clone, PartialEq)]
struct PolarPoint{
    r: f64,
    t: f64,
}

fn point2polar(pt_list: &[Point]) -> Vec<PolarPoint>{
    let mut return_vec = Vec::new();
    let mut cloned_vec = Vec::new();
    for i in pt_list{
        let temp = i.clone();
        cloned_vec.push(temp);
    }
    for i in 0..cloned_vec.len(){
        let distance = ((cloned_vec[i].x).powf(2.) + (cloned_vec[i].y).powf(2.)).sqrt();
        let theta = (cloned_vec[i].y/cloned_vec[i].x).atan();
        let current_polar_point = PolarPoint{
            r: distance,
            t: theta,
        };
        return_vec.push(current_polar_point);
    }
    return_vec
}

fn main() {
    let testlist = [Point{x:2.72, y:3.82,}, Point{x:5., y:5.,}];
    println!("{:?}", point2polar(&testlist));
}
