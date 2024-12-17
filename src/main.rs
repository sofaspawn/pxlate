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
    pxlated.save("readme_expo/mahoraga_pxlated.png").unwrap();

    let fin_img = ImageBuffer::from_fn(img_x, img_y, |x, y| {
        matrix[(y * img_x + x) as usize] // Access the corresponding pixel
    });

    fin_img.save(op_img_name).expect("Error saving the image");
}

fn color_diff(c1: Rgba<u8>, c2: Rgba<u8>) -> i32 {
    let r_diff = (c1[0] as i32 - c2[0] as i32).pow(2);
    let g_diff = (c1[1] as i32 - c2[1] as i32).pow(2);
    let b_diff = (c1[2] as i32 - c2[2] as i32).pow(2);

    ((r_diff + g_diff + b_diff) as f64).sqrt() as i32
}

fn pxlate(img: DynamicImage) -> DynamicImage {
    let dwnscl = downscale(img);
    let palette: Vec<Rgba<u8>> = vec![
        // retro
        //Rgba([0, 0, 0, 255]),
        //Rgba([255, 255, 255, 255]),
        //Rgba([136, 0, 0, 255]),
        //Rgba([170, 255, 238, 255]),
        //Rgba([204, 68, 204, 255]),
        //Rgba([0, 204, 85, 255]),
        //Rgba([0, 0, 170, 255]),
        //Rgba([238, 238, 119, 255]),
        //Rgba([221, 136, 85, 255]),
        //Rgba([102, 68, 0, 255]),
        //Rgba([255, 119, 119, 255]),
        //Rgba([51, 51, 51, 255]),
        //Rgba([119, 119, 119, 255]),
        //Rgba([170, 255, 102, 255]),
        //Rgba([0, 136, 255, 255]),
        //Rgba([187, 187, 187, 255]),
        // onedark
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
        // dracula
        //Rgba([40, 42, 54, 255]),
        //Rgba([248, 248, 242, 255]),
        //Rgba([255, 85, 85, 255]),
        //Rgba([80, 250, 123, 255]),
        //Rgba([241, 250, 140, 255]),
        //Rgba([189, 147, 249, 255]),
        //Rgba([255, 121, 198, 255]),
        //Rgba([139, 233, 253, 255]),
        //Rgba([255, 184, 108, 255]),
        //Rgba([68, 71, 90, 255]),
        //Rgba([98, 114, 164, 255]),
        //Rgba([255, 110, 110, 255]),
        //Rgba([95, 255, 135, 255]),
        //Rgba([58, 60, 78, 255]),
        //Rgba([241, 250, 140, 255]),
        //Rgba([68, 71, 90, 255]),
        // monochrome
        //Rgba([0, 0, 0, 255]),
        //Rgba([255, 255, 255, 255]),
        //Rgba([85, 85, 85, 255]),
        //Rgba([170, 170, 170, 255]),
        //Rgba([212, 212, 212, 255]),
        //Rgba([128, 128, 128, 255]),
        //Rgba([192, 192, 192, 255]),
        //Rgba([224, 224, 224, 255]),
        //Rgba([160, 160, 160, 255]),
        //Rgba([32, 32, 32, 255]),
        //Rgba([96, 96, 96, 255]),
        //Rgba([144, 144, 144, 255]),
        //Rgba([208, 208, 208, 255]),
        //Rgba([16, 16, 16, 255]),
        //Rgba([240, 240, 240, 255]),
        //Rgba([64, 64, 64, 255]),
        // monokai
        //Rgba([39, 40, 34, 255]),
        //Rgba([248, 248, 242, 255]),
        //Rgba([249, 38, 114, 255]),
        //Rgba([166, 226, 46, 255]),
        //Rgba([230, 219, 116, 255]),
        //Rgba([102, 217, 239, 255]),
        //Rgba([174, 129, 255, 255]),
        //Rgba([161, 239, 228, 255]),
        //Rgba([253, 151, 31, 255]),
        //Rgba([69, 70, 64, 255]),
        //Rgba([117, 113, 94, 255]),
        //Rgba([249, 38, 114, 255]),
        //Rgba([166, 226, 46, 255]),
        //Rgba([56, 56, 48, 255]),
        //Rgba([248, 248, 242, 255]),
        //Rgba([117, 113, 94, 255]),
        // solarized
        //Rgba([0, 43, 54, 255]),
        //Rgba([131, 148, 150, 255]),
        //Rgba([220, 50, 47, 255]),
        //Rgba([133, 153, 0, 255]),
        //Rgba([181, 137, 0, 255]),
        //Rgba([38, 139, 210, 255]),
        //Rgba([211, 54, 130, 255]),
        //Rgba([42, 161, 152, 255]),
        //Rgba([203, 75, 22, 255]),
        //Rgba([7, 54, 66, 255]),
        //Rgba([88, 110, 117, 255]),
        //Rgba([253, 246, 227, 255]),
        //Rgba([238, 232, 213, 255]),
        //Rgba([0, 43, 54, 255]),
        //Rgba([253, 246, 227, 255]),
        //Rgba([101, 123, 131, 255]),
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
    //let (smolwidth, smolheight) = (width / 8, height / 8);
    let (smolwidth, smolheight) = (width / 4, height / 4); // for a more detailed image
                                                           //let (smolwidth, smolheight) = (width / 5, height / 5);

    let dwnsclimg = img.resize_exact(smolwidth, smolheight, image::imageops::FilterType::Nearest);
    return dwnsclimg;
}

fn upscale(img: DynamicImage) -> DynamicImage {
    let (width, height) = img.dimensions();
    //let (bigwidth, bigheight) = (width * 8, height * 8);
    let (bigwidth, bigheight) = (width * 4, height * 4); // for a more detailed image
                                                         //let (bigwidth, bigheight) = (width * 5, height * 5);

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
