mod ex1;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let res = ex1::increases_window(&args[1]);
    println!("Result is {}", res)
}
