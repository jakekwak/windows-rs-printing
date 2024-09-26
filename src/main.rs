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
const CANVAS_WIDTH: u32 = (RECEIPT_WIDTH_INCHES * DPI) as u32;
const PRINTABLE_WIDTH: u32 = CANVAS_WIDTH - 105; // 실제 프린트 가능 영역
const RIGHT_MARGIN: i32 = 10;
const LEFT_MARGIN: i32 = 10;

struct PrintConfig {
    image_scale: f32,
    print_scale: f32,
}

#[derive(Copy, Clone)]
enum TextAlign {
    Left,
    Center,
    Right,
}

fn main() -> Result<(), Error> {
    let config = PrintConfig {
        image_scale: 1.0,
        print_scale: PRINTABLE_WIDTH as f32 / CANVAS_WIDTH as f32,
    };

    let width = CANVAS_WIDTH;
    let height = 1000;
    let mut img: RgbaImage = ImageBuffer::new(width, height);
    img.fill(255);

    let font = Font::try_from_bytes(include_bytes!("C:\\Windows\\Fonts\\arial.ttf")).unwrap();
    // ariblk.ttf  arialbd.ttf arial.ttf
    let font_bold =
        Font::try_from_bytes(include_bytes!("C:\\Windows\\Fonts\\arialbd.ttf")).unwrap();
    // let font = Font::try_from_bytes(include_bytes!("C:\\Windows\\Fonts\\malgun.ttf")).unwrap();
    // let font_bold =
    //     Font::try_from_bytes(include_bytes!("C:\\Windows\\Fonts\\malgunbd.ttf")).unwrap();

    let mut y_offset = 10;

    // Add receipt content
    add_text(
        &mut img,
        "M1",
        36.0,
        true,
        &mut y_offset,
        TextAlign::Center,
        0,
        &config,
        &font,
        &font_bold,
    );
    y_offset += 20; // Add a little extra space after the large title

    add_text(
        &mut img,
        "SERVER REQUEST",
        16.0,
        true,
        &mut y_offset,
        TextAlign::Center,
        0,
        &config,
        &font,
        &font_bold,
    );
    y_offset += 10; // Add a little extra space after the subtitle

    add_text(
        &mut img,
        "CHANGE GRILL",
        14.0,
        true,
        &mut y_offset,
        TextAlign::Center,
        0,
        &config,
        &font,
        &font_bold,
    );
    add_text(
        &mut img,
        "2 ICE WATER",
        14.0,
        true,
        &mut y_offset,
        TextAlign::Center,
        0,
        &config,
        &font,
        &font_bold,
    );
    add_text(
        &mut img,
        "2 DISHES",
        14.0,
        true,
        &mut y_offset,
        TextAlign::Center,
        0,
        &config,
        &font,
        &font_bold,
    );
    add_text(
        &mut img,
        "2 WET TOWELS",
        14.0,
        true,
        &mut y_offset,
        TextAlign::Center,
        0,
        &config,
        &font,
        &font_bold,
    );

    y_offset += 20; // Add some space before the "Thank You" message
    add_text(
        &mut img,
        "Thank You",
        12.0,
        true,
        &mut y_offset,
        TextAlign::Center,
        0,
        &config,
        &font,
        &font_bold,
    );

    // Add line
    add_line(&mut img, y_offset);

    // Save image to file for debugging
    save_debug_image(&img, "receipt_debug.png")?;

    // Ask user if they want to print
    if ask_to_print() {
        print_receipt(&img, &config)?;
    } else {
        println!("Printing cancelled.");
    }

    Ok(())
}

fn add_text(
    img: &mut RgbaImage,
    text: &str,
    size: f32,
    is_bold: bool,
    y: &mut i32,
    align: TextAlign,
    indent: i32,
    config: &PrintConfig,
    font: &Font,
    font_bold: &Font,
) {
    let scale = Scale::uniform(size * DPI / 54.0 * config.image_scale);
    let font_to_use = if is_bold { font_bold } else { font };
    let text_width = font_to_use
        .layout(text, scale, rusttype::point(0.0, 0.0))
        .map(|g| g.position().x + g.unpositioned().h_metrics().advance_width)
        .last()
        .unwrap_or(0.0);
    let x = match align {
        TextAlign::Left => LEFT_MARGIN + indent,
        TextAlign::Center => ((PRINTABLE_WIDTH as f32 - text_width) / 2.0) as i32,
        TextAlign::Right => PRINTABLE_WIDTH as i32 - text_width as i32 - RIGHT_MARGIN - indent,
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

    // Calculate the new y_offset based on the font size
    let line_height = (size * DPI / 54.0 * config.image_scale) as i32;
    *y += line_height;
}

fn add_line(img: &mut RgbaImage, y: i32) {
    draw_line_segment_mut(
        img,
        (LEFT_MARGIN as f32, y as f32),
        ((PRINTABLE_WIDTH - RIGHT_MARGIN as u32) as f32, y as f32),
        Rgba([0, 0, 0, 255]),
    );
}

fn save_debug_image(img: &RgbaImage, filename: &str) -> Result<(), Error> {
    let output_path = Path::new(filename);
    img.save(output_path).expect("Failed to save debug image");
    println!("Debug image saved to {:?}", output_path);
    Ok(())
}

fn ask_to_print() -> bool {
    print!("Do you want to print the receipt? (y/n): ");
    io::stdout().flush().unwrap();
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
    user_input.trim().to_lowercase() == "y"
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
