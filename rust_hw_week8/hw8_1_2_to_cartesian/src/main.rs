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

fn point2cartesian(pt_list: &[PolarPoint]) -> Vec<Point>{
    let mut return_vec = Vec::new();
    let mut cloned_vec = Vec::new();
    for i in pt_list{
        let temp = i.clone();
        cloned_vec.push(temp);
    }
    for i in 0..cloned_vec.len(){
        let xcoord = cloned_vec[i].r*(cloned_vec[i].t).cos();
        let ycoord = cloned_vec[i].r*(cloned_vec[i].t).sin();
        let current_cartesian_point = Point{
            x: xcoord,
            y: ycoord,
        };
        return_vec.push(current_cartesian_point);
    }
    return_vec
}

fn main() {
    let piover4 = std::f64::consts::PI/4.;
    let ninesqrttwo = 9.*(2_f64).sqrt();
    let testlist = [PolarPoint { r: ninesqrttwo, t: piover4 }, PolarPoint { r: 7.0710678118654755, t: 0.7853981633974483 }];
    println!("{:?}", point2cartesian(&testlist));
}
