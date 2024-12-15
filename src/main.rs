use image::Rgba;
use image::{GenericImageView, ImageBuffer, ImageReader};

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

    let (img_x, img_y) = img.dimensions();

    let mut matrix: Vec<Rgba<u8>> = img.pixels().map(|p| p.2).collect();
    //dbg!(matrix);

    pixelate(&mut matrix);

    let fin_img = ImageBuffer::from_fn(img_x, img_y, |x, y| {
        matrix[(y * img_x + x) as usize] // Access the corresponding pixel
    });

    fin_img.save(op_img_name).expect("Error saving the image");
}

fn pixelate(matrix: &mut Vec<Rgba<u8>>) {
    let mut i = 0;
    let pix = 10;

    while i < matrix.len() {
        // dbg!(i);
        let mut j = 0;
        let mut clrs = vec![];

        while j < pix {
            if i + j >= matrix.len() {
                break;
            }
            clrs.push(matrix[i + j]);
            j += 1;
        }

        let avg_red = (clrs.iter().map(|c| c.0[0] as u32).sum::<u32>() / clrs.len() as u32) as u8;
        let avg_green = (clrs.iter().map(|c| c.0[1] as u32).sum::<u32>() / clrs.len() as u32) as u8;
        let avg_blue = (clrs.iter().map(|c| c.0[2] as u32).sum::<u32>() / clrs.len() as u32) as u8;

        let clr = Rgba([avg_red, avg_green, avg_blue, 255]);

        for k in i..i + pix {
            matrix[k] = clr;
        }

        i = j + i;
    }
}
