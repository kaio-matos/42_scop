fn main() {
    let args = std::env::args_os();

    let str = args.skip(1).nth(0).unwrap();
    let path = std::path::Path::new(&str);
    let img = image::open(path).unwrap();
    println!("{} {}", img.width(), img.height());
    let vc = img.into_rgba8();
    for e in vc.into_vec() {
        print!("{} ", e);
    }
}
