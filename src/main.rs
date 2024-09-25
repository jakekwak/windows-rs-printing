use image::{ImageBuffer, Rgba, RgbaImage};
use imageproc::drawing::{draw_line_segment_mut, draw_text_mut};
use rusttype::{Font, Scale};
use std::io::Error;
use std::path::Path;
use windows::core::{HSTRING, PCWSTR};
use windows::Win32::Graphics::Gdi::{
    CreateDCW, DeleteDC, SetDIBitsToDevice, BITMAPINFO, BITMAPINFOHEADER, DIB_RGB_COLORS,
};
use windows::Win32::Storage::Xps::{EndDoc, EndPage, StartDocW, StartPage, DOCINFOW};

const DPI: f32 = 203.0;
const RECEIPT_WIDTH_INCHES: f32 = 3.125;
const RECEIPT_WIDTH_PIXELS: u32 = (RECEIPT_WIDTH_INCHES * DPI) as u32;

fn main() -> Result<(), Error> {
    let width = RECEIPT_WIDTH_PIXELS;
    let height = 1000;
    let mut img: RgbaImage = ImageBuffer::new(width, height);
    img.fill(255);

    // Load Malgun Gothic fonts
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
                    center: bool,
                    indent: i32| {
        let scale = Scale::uniform(size * DPI / 54.0);
        let font_to_use = if is_bold { &font_bold } else { &font };
        let text_width = font_to_use
            .layout(text, scale, rusttype::point(0.0, 0.0))
            .map(|g| g.position().x + g.unpositioned().h_metrics().advance_width)
            .last()
            .unwrap_or(0.0);
        let x = if center {
            ((width as f32 - text_width) / 2.0) as i32
        } else {
            indent
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
        *y += (line_height as f32 * DPI / 72.0) as i32;
    };

    let add_line = |img: &mut RgbaImage, y: i32| {
        draw_line_segment_mut(
            img,
            (10.0, y as f32),
            ((width - 10) as f32, y as f32),
            Rgba([0, 0, 0, 255]),
        );
    };

    // Add receipt content
    add_text(&mut img, "Kitchen #1", 14.0, true, &mut y_offset, true, 0);
    y_offset += 5;
    add_line(&mut img, y_offset);
    y_offset += 5;
    add_text(&mut img, "TABLE M1", 12.0, true, &mut y_offset, true, 0);
    add_text(&mut img, "ORDER #1-1", 12.0, true, &mut y_offset, true, 0);
    y_offset += 5;
    add_line(&mut img, y_offset);
    y_offset += 5;
    add_text(
        &mut img,
        "Invoice #1    Mon, 9/23/2024 6:37 PM",
        10.0,
        false,
        &mut y_offset,
        false,
        10,
    );
    y_offset += 5;
    add_line(&mut img, y_offset);
    y_offset += 5;

    add_text(
        &mut img,
        "1 Avocado Eggrolls",
        10.0,
        false,
        &mut y_offset,
        false,
        10,
    );
    add_text(
        &mut img,
        "아보카도 에그롤",
        10.0,
        false,
        &mut y_offset,
        false,
        20,
    );
    add_text(
        &mut img,
        "1 Make It Gluten Free",
        10.0,
        false,
        &mut y_offset,
        false,
        20,
    );
    add_text(
        &mut img,
        "1 Diet Coke",
        10.0,
        false,
        &mut y_offset,
        false,
        10,
    );
    add_text(&mut img, "1 Yes", 10.0, false, &mut y_offset, false, 20);
    add_text(
        &mut img,
        "1 French Fries",
        10.0,
        false,
        &mut y_offset,
        false,
        10,
    );
    add_text(&mut img, "감자 튀김", 10.0, false, &mut y_offset, false, 20);
    add_text(
        &mut img,
        "1 Petite Filet",
        10.0,
        false,
        &mut y_offset,
        false,
        10,
    );
    add_text(&mut img, "뼈때 필레", 10.0, false, &mut y_offset, false, 20);
    add_text(
        &mut img,
        "1 Medium Rare",
        10.0,
        false,
        &mut y_offset,
        false,
        20,
    );
    add_text(
        &mut img,
        "1 Make It Gluten Free",
        10.0,
        false,
        &mut y_offset,
        false,
        20,
    );

    y_offset += 10;
    add_text(
        &mut img,
        "Printed 6:37 PM",
        10.0,
        false,
        &mut y_offset,
        true,
        0,
    );

    // Save image to file for debugging
    let output_path = Path::new("receipt_debug.png");
    img.save(output_path).expect("Failed to save debug image");
    println!("Debug image saved to {:?}", output_path);

    // Ask user if they want to print
    println!("Do you want to print the receipt? (y/n)");
    let mut user_input = String::new();
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    if user_input.trim().to_lowercase() == "y" {
        // Printer setup
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
                img.width(),
                img.height(),
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
    } else {
        println!("Printing cancelled.");
    }

    Ok(())
}
