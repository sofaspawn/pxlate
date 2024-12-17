use std::cmp::min;

use image::{DynamicImage, Rgba};
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

    let mut pix = 10;

    if args.len() > 3 {
        match args.len() {
            4 => pix = args[3].parse::<u32>().unwrap() as usize,
            _ => pix = pix,
        }
    }

    let img = ImageReader::open(img_path).unwrap().decode().unwrap();

    let (img_x, img_y) = img.dimensions();

    let mut matrix: Vec<Rgba<u8>> = img.pixels().map(|p| p.2).collect();
    //dbg!(matrix);

    smudge(&mut matrix, pix);
    let pxlated = pxlate(img);
    pxlated.save("pxlated.png").unwrap();

    let fin_img = ImageBuffer::from_fn(img_x, img_y, |x, y| {
        matrix[(y * img_x + x) as usize] // Access the corresponding pixel
    });

    fin_img.save(op_img_name).expect("Error saving the image");
}

fn color_diff(c1: Rgba<u8>, c2: Rgba<u8>) -> i32 {
    let avg_color_1 = c1.clone().0.iter().map(|c| *c as i32).sum::<i32>() / c1.0.len() as i32;
    let avg_color_2 = c2.clone().0.iter().map(|c| *c as i32).sum::<i32>() / c2.0.len() as i32;
    return (avg_color_2 - avg_color_1).abs();
}

fn pxlate(img: DynamicImage) -> DynamicImage {
    let (width, height) = img.dimensions();
    let dwnscl = downscale(img);
    let palette: Vec<Rgba<u8>> = vec![
        Rgba([0, 0, 0, 255]),
        Rgba([255, 255, 255, 255]),
        Rgba([136, 0, 0, 255]),
        Rgba([170, 255, 238, 255]),
        Rgba([204, 68, 204, 255]),
        Rgba([0, 204, 85, 255]),
        Rgba([0, 0, 170, 255]),
        Rgba([238, 238, 119, 255]),
        Rgba([221, 136, 85, 255]),
        Rgba([102, 68, 0, 255]),
        Rgba([255, 119, 119, 255]),
        Rgba([51, 51, 51, 255]),
        Rgba([119, 119, 119, 255]),
        Rgba([170, 255, 102, 255]),
        Rgba([0, 136, 255, 255]),
        Rgba([187, 187, 187, 255]),
        Rgba([40, 44, 52, 255]),
        Rgba([171, 178, 191, 255]),
        Rgba([224, 108, 117, 255]),
        Rgba([152, 195, 121, 255]),
        Rgba([229, 192, 123, 255]),
        Rgba([97, 175, 239, 255]),
        Rgba([198, 120, 221, 255]),
        Rgba([86, 182, 194, 255]),
        Rgba([190, 80, 70, 255]),
        Rgba([92, 99, 112, 255]),
        Rgba([130, 137, 151, 255]),
        Rgba([209, 154, 102, 255]),
        Rgba([195, 232, 141, 255]),
        Rgba([56, 62, 71, 255]),
        Rgba([239, 241, 245, 255]),
        Rgba([75, 82, 94, 255]),
    ];

    let mut matrix: Vec<Rgba<u8>> = dwnscl.pixels().map(|p| p.2).collect();

    for (i, pxl) in matrix.clone().iter().enumerate() {
        let (mut iclr, mut pxldiff) = (Rgba([0, 0, 0, 255]), 255);
        for clr in &palette {
            let diff = color_diff(*pxl, *clr);
            if diff < pxldiff {
                iclr = *clr;
            }
            pxldiff = min(pxldiff, diff);
        }
        matrix[i] = iclr;
    }

    let (smol_width, smol_height) = dwnscl.dimensions();

    let fin_img = ImageBuffer::from_fn(smol_width, smol_height, |x, y| {
        matrix[(y * smol_width + x) as usize] // Access the corresponding pixel
    });

    let upsclimg = upscale(DynamicImage::ImageRgba8(fin_img));

    return upsclimg;
}

fn downscale(img: DynamicImage) -> DynamicImage {
    let (width, height) = img.dimensions();
    let (smolwidth, smolheight) = (width / 5, height / 5);

    let dwnsclimg = img.resize_exact(smolwidth, smolheight, image::imageops::FilterType::Nearest);
    return dwnsclimg;
}

fn upscale(img: DynamicImage) -> DynamicImage {
    let (width, height) = img.dimensions();
    let (bigwidth, bigheight) = (width * 5, height * 5);

    let upsclimg = img.resize_exact(bigwidth, bigheight, image::imageops::FilterType::Nearest);
    return upsclimg;
}

fn smudge(matrix: &mut Vec<Rgba<u8>>, pix: usize) {
    let mut i = 0;
    //let pix = 10;

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

        for k in i..(i + pix).min(matrix.len()) {
            matrix[k] = clr;
        }

        i = j + i;
    }
}
