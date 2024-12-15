use image::{GenericImageView, ImageReader};

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() < 3 {
        eprintln!("command not valid");
        eprintln!("USAGE: {} <path-to-image> <name-of-output-file>", args[0]);
        return;
    }

    let img_path = args[1].clone();
    let op_img_name = args[2].clone();

    let img = ImageReader::open(img_path).unwrap().decode().unwrap();

    let matrix = img.pixels().map(|p| p.2).collect::<Vec<_>>();

    //new_img.save(op_img_name).expect("can't. sorry.");
}
