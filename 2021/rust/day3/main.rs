mod ex1;

fn main() {
    let (hor,depth) = ex1::get_final_position("input").unwrap();
    let res = hor * depth;
    println!("Pos: {}\nDepth: {}", res, depth);
    println!("Result: {}", res);
}
