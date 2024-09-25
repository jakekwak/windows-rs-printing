use image::{ImageBuffer, Rgba, RgbaImage};
use imageproc::drawing::{draw_line_segment_mut, draw_text_mut};
use rusttype::{Font, Scale};
use std::io::{self, Error, Write};
use std::path::Path;
use windows::core::{HSTRING, PCWSTR};
use windows::Win32::Graphics::Gdi::{
    CreateDCW, DeleteDC, SetDIBitsToDevice, BITMAPINFO, BITMAPINFOHEADER, DIB_RGB_COLORS,
};
use windows::Win32::Storage::Xps::{EndDoc, EndPage, StartDocW, StartPage, DOCINFOW};

const DPI: f32 = 203.0;
const RECEIPT_WIDTH_INCHES: f32 = 3.125;
const RECEIPT_WIDTH_PIXELS: u32 = (RECEIPT_WIDTH_INCHES * DPI) as u32;
const RIGHT_MARGIN: i32 = 10; // 오른쪽 여백 증가
const LEFT_MARGIN: i32 = 10; // 왼쪽 여백 추가

struct PrintConfig {
    image_scale: f32,
    print_scale: f32,
}

fn main() -> Result<(), Error> {
    let config = PrintConfig {
        image_scale: 1.0,
        print_scale: 1.0,
    };

    let width = (RECEIPT_WIDTH_PIXELS as f32 * config.image_scale) as u32;
    let height = 1000;
    let mut img: RgbaImage = ImageBuffer::new(width, height);
    img.fill(255);

    let font = Font::try_from_bytes(include_bytes!("C:\\Windows\\Fonts\\malgun.ttf")).unwrap();
    let font_bold =
        Font::try_from_bytes(include_bytes!("C:\\Windows\\Fonts\\malgunbd.ttf")).unwrap();

    let mut y_offset = 10;
    let line_height = 15;

    let add_text = |img: &mut RgbaImage,
                    text: &str,
                    size: f32,
                    is_bold: bool,
                    y: &mut i32,
                    align: TextAlign,
                    indent: i32| {
        let scale = Scale::uniform(size * DPI / 54.0 * config.image_scale);
        let font_to_use = if is_bold { &font_bold } else { &font };
        let text_width = font_to_use
            .layout(text, scale, rusttype::point(0.0, 0.0))
            .map(|g| g.position().x + g.unpositioned().h_metrics().advance_width)
            .last()
            .unwrap_or(0.0);
        println!(
            "width: {} text_width: {} scale: {}",
            width,
            text_width,
            size * DPI / 54.0
        );
        let x = match align {
            TextAlign::Left => LEFT_MARGIN + indent,
            TextAlign::Center => ((width as f32 - text_width) / 2.0) as i32,
            TextAlign::Right => width as i32 - text_width as i32 - RIGHT_MARGIN - indent,
        };
        draw_text_mut(img, Rgba([0, 0, 0, 255]), x, *y, scale, font_to_use, text);
        draw_text_mut(
            img,
            Rgba([0, 0, 0, 255]),
            x + 1,
            *y,
            scale,
            font_to_use,
            text,
        );
        *y += (line_height as f32 * DPI / 72.0 * config.image_scale) as i32;
    };

    let add_line = |img: &mut RgbaImage, y: i32| {
        draw_line_segment_mut(
            img,
            (LEFT_MARGIN as f32, y as f32),
            ((width - RIGHT_MARGIN as u32) as f32, y as f32),
            Rgba([0, 0, 0, 255]),
        );
    };

    #[derive(Copy, Clone)]
    enum TextAlign {
        Left,
        Center,
        Right,
    }

    y_offset += 100;

    // Add receipt content
    add_text(
        &mut img,
        "Kitchen #1",
        14.0,
        true,
        &mut y_offset,
        TextAlign::Center,
        0,
    );
    y_offset += 5;
    add_line(&mut img, y_offset);
    y_offset += 5;
    add_text(
        &mut img,
        "TABLE M1",
        12.0,
        true,
        &mut y_offset,
        TextAlign::Center,
        0,
    );
    add_text(
        &mut img,
        "ORDER #1-1",
        12.0,
        true,
        &mut y_offset,
        TextAlign::Center,
        0,
    );
    y_offset += 5;
    add_line(&mut img, y_offset);
    y_offset += 5;

    add_text(
        &mut img,
        "Invoice #1",
        8.0,
        false,
        &mut y_offset,
        TextAlign::Left,
        0,
    );
    y_offset -= (line_height as f32 * DPI / 72.0 * config.image_scale) as i32;
    add_text(
        &mut img,
        "Mon, 9/23/2024 6:37 PM",
        8.0,
        false,
        &mut y_offset,
        TextAlign::Right,
        0,
    );
    y_offset += 5;
    add_line(&mut img, y_offset);
    y_offset += 5;

    add_text(
        &mut img,
        "1 Avocado Eggrolls",
        10.0,
        true,
        &mut y_offset,
        TextAlign::Left,
        0,
    );
    add_text(
        &mut img,
        "아보카도 에그롤",
        10.0,
        true,
        &mut y_offset,
        TextAlign::Left,
        10,
    );
    add_text(
        &mut img,
        "1 Make It Gluten Free",
        10.0,
        true,
        &mut y_offset,
        TextAlign::Left,
        10,
    );
    add_text(
        &mut img,
        "1 Diet Coke",
        10.0,
        true,
        &mut y_offset,
        TextAlign::Left,
        0,
    );
    add_text(
        &mut img,
        "1 Yes",
        10.0,
        true,
        &mut y_offset,
        TextAlign::Left,
        10,
    );
    add_text(
        &mut img,
        "1 French Fries",
        10.0,
        true,
        &mut y_offset,
        TextAlign::Left,
        0,
    );
    add_text(
        &mut img,
        "감자 튀김",
        10.0,
        true,
        &mut y_offset,
        TextAlign::Left,
        10,
    );
    add_text(
        &mut img,
        "1 Petite Filet",
        10.0,
        true,
        &mut y_offset,
        TextAlign::Left,
        0,
    );
    add_text(
        &mut img,
        "뼈때 필레",
        10.0,
        true,
        &mut y_offset,
        TextAlign::Left,
        10,
    );
    add_text(
        &mut img,
        "1 Medium Rare",
        10.0,
        true,
        &mut y_offset,
        TextAlign::Left,
        10,
    );
    add_text(
        &mut img,
        "1 Make It Gluten Free",
        10.0,
        true,
        &mut y_offset,
        TextAlign::Left,
        10,
    );
    y_offset += 5;
    add_line(&mut img, y_offset);
    y_offset += 40;
    add_text(
        &mut img,
        "Printed 6:37 PM",
        8.0,
        false,
        &mut y_offset,
        TextAlign::Center,
        0,
    );

    // Save image to file for debugging
    let output_path = Path::new("receipt_debug.png");
    img.save(output_path).expect("Failed to save debug image");
    println!("Debug image saved to {:?}", output_path);

    // Ask user if they want to print
    print!("Do you want to print the receipt? (y/n): ");
    io::stdout().flush().unwrap();
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    if user_input.trim().to_lowercase() == "y" {
        print_receipt(&img, &config)?;
    } else {
        println!("Printing cancelled.");
    }

    Ok(())
}

fn print_receipt(img: &RgbaImage, config: &PrintConfig) -> Result<(), Error> {
    unsafe {
        let printer_name = HSTRING::from("Receipt");
        let hdc = CreateDCW(
            PCWSTR::null(),
            PCWSTR(printer_name.as_ptr()),
            PCWSTR::null(),
            None,
        );

        let doc_name = HSTRING::from("주문 영수증");
        let mut doc_info = DOCINFOW {
            cbSize: std::mem::size_of::<DOCINFOW>() as i32,
            lpszDocName: PCWSTR(doc_name.as_ptr()),
            lpszOutput: PCWSTR::null(),
            lpszDatatype: PCWSTR::null(),
            fwType: 0,
        };

        StartDocW(hdc, &mut doc_info);
        StartPage(hdc);

        let scaled_width = (img.width() as f32 * config.print_scale) as i32;
        let scaled_height = (img.height() as f32 * config.print_scale) as i32;

        let bmi = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: img.width() as i32,
                biHeight: -(img.height() as i32),
                biPlanes: 1,
                biBitCount: 32,
                biCompression: 0,
                biSizeImage: 0,
                biXPelsPerMeter: (DPI * 100.0 / 2.54) as i32,
                biYPelsPerMeter: (DPI * 100.0 / 2.54) as i32,
                biClrUsed: 0,
                biClrImportant: 0,
            },
            bmiColors: [windows::Win32::Graphics::Gdi::RGBQUAD::default(); 1],
        };

        SetDIBitsToDevice(
            hdc,
            0,
            0,
            scaled_width as u32,
            scaled_height as u32,
            0,
            0,
            0,
            img.height(),
            img.as_raw().as_ptr() as _,
            &bmi,
            DIB_RGB_COLORS,
        );

        EndPage(hdc);
        EndDoc(hdc);

        let _ = DeleteDC(hdc);
    }

    println!("Receipt printed.");
    Ok(())
}
