fn greet_world() {
    println!("Hello, world!");
    let southern_germany = "GrüßGott!";
    let japan = "ハロー・ワールド";
    let regions = [southern_germany, japan];
    for region in regions {
        println!("{}", &region);
    }
}

fn main() {
    greet_world();
}