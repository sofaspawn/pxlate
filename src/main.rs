use std::cmp::min;

use image::{DynamicImage, Rgba};
use image::{GenericImageView, ImageBuffer, ImageReader};

use std::time;


fn help(args: &Vec<String>){
    let help = format!("
!!!INVALID CALL!!!
USAGE: {binary_name} <command> <path-to-input-image.png/jpg/jpeg> <name-of-output-image.png> [pixel_size/scaling_factor]
[] -> optional
List of valid commands:
1. smudge -> basic smudge the entire image to make it a little blurry
2. pixelate -> convert an image into a cool pixelated representation of itself

Optional:
1. pixel_size -> number of pixels to average out horizontally
2. scaling_factor -> the amount by which to downscale and upscale the image (affects the level of detail)

example: {binary_name} pixelate input.png output.png 4
        ", binary_name=args[0]);
    eprintln!("{}", help);
}

fn main() {
    let start = time::Instant::now();
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() < 4 {
        help(&args);
        return;
    }

    let command = args[1].clone();
    let img_path = args[2].clone();
    let op_img_name = args[3].clone();

    let mut pix = 10;

    if args.len() > 4 {
        pix = match args.len() {
            5 => args[4].parse::<u32>().unwrap() as usize,
            _ => pix,
        }
    }

    println!("Reading Image...");
    let img = ImageReader::open(&img_path).unwrap().decode().unwrap();

    println!("Performing Operation...");

    let output = match command.as_str(){
        "smudge" => {smudge(img.clone(), pix)},
        "pixelate" => {pxlate(img.clone(), pix)},
        _ => {img.clone()}
    };

    output.save(&op_img_name).expect("file not saved properly");

    let end = time::Instant::now();
    println!("✨ Operation Successful ✨");
    println!("{img_path} -> {op_img_name}");
    println!("Time Taken: {:?} seconds", (end - start).as_secs_f64());
}

fn color_diff(c1: Rgba<u8>, c2: Rgba<u8>) -> i32 {
    let r_diff = (c1[0] as i32 - c2[0] as i32).pow(2);
    let g_diff = (c1[1] as i32 - c2[1] as i32).pow(2);
    let b_diff = (c1[2] as i32 - c2[2] as i32).pow(2);

    ((r_diff + g_diff + b_diff) as f64).sqrt() as i32
}

fn pxlate(img: DynamicImage, sfactor:usize) -> DynamicImage {
    let dwnscl = downscale(img, sfactor);
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

    let upsclimg = upscale(DynamicImage::ImageRgba8(fin_img), sfactor);

    return upsclimg;
}

fn downscale(img: DynamicImage, sfactor: usize) -> DynamicImage {
    let (width, height) = img.dimensions();
    //let (smolwidth, smolheight) = (width / 4, height / 4); // for a more detailed image
    let (smolwidth, smolheight) = (width / sfactor as u32, height / sfactor as u32);

    let dwnsclimg = img.resize_exact(smolwidth, smolheight, image::imageops::FilterType::Nearest);
    return dwnsclimg;
}

fn upscale(img: DynamicImage, sfactor: usize) -> DynamicImage {
    let (width, height) = img.dimensions();
    //let (bigwidth, bigheight) = (width * 8, height * 8);
    //let (bigwidth, bigheight) = (width * 4, height * 4); // for a more detailed image
    let (bigwidth, bigheight) = (width * sfactor as u32, height * sfactor as u32);

    let upsclimg = img.resize_exact(bigwidth, bigheight, image::imageops::FilterType::Nearest);
    return upsclimg;
}

fn smudge(img: DynamicImage, pix: usize) -> DynamicImage {
    let (img_x, img_y) = img.dimensions();
    let mut matrix: Vec<Rgba<u8>> = img.pixels().map(|p| p.2).collect();
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
    let fin_img = ImageBuffer::from_fn(img_x, img_y, |x, y| {
        matrix[(y * img_x + x) as usize] // Access the corresponding pixel
    });

    return DynamicImage::ImageRgba8(fin_img);
}
