use image::{ImageBuffer, Rgba};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};
use std::io::Error;
use windows::core::{HSTRING, PCWSTR};
use windows::Win32::Graphics::Gdi::{
    CreateDCW, SetDIBitsToDevice, BITMAPINFO, BITMAPINFOHEADER, DIB_RGB_COLORS,
};
use windows::Win32::Storage::Xps::{EndDoc, EndPage, StartDocW, StartPage, DOCINFOW};

fn main() -> Result<(), Error> {
    // 이미지 생성
    let mut img = ImageBuffer::new(900, 600);
    img.fill(255);

    // 텍스트 추가
    let font = Font::try_from_bytes(include_bytes!("C:\\Windows\\Fonts\\K_malgunbd.ttf")).unwrap();
    let scale = Scale::uniform(32.0);
    let text = "안녕하세요, Rust로 프린트합니다!";

    // let text_size = text_size(scale, &font, text);
    draw_text_mut(&mut img, Rgba([0, 0, 0, 255]), 50, 50, scale, &font, text);

    // 이미지 파일 추가 (예: logo.png)
    let logo = image::open("logo.png").unwrap().to_rgba8();
    image::imageops::overlay(&mut img, &logo, 50, 100);

    // 프린터 설정
    unsafe {
        let printer_name = HSTRING::from("Receipt");
        let hdc = CreateDCW(
            PCWSTR::null(),
            PCWSTR(printer_name.as_ptr()),
            PCWSTR::null(),
            None,
        );

        let doc_name = HSTRING::from("Rust 프린트 작업");
        let mut doc_info = DOCINFOW {
            cbSize: std::mem::size_of::<DOCINFOW>() as i32,
            lpszDocName: PCWSTR(doc_name.as_ptr()),
            lpszOutput: PCWSTR::null(),
            lpszDatatype: PCWSTR::null(),
            fwType: 0,
        };

        StartDocW(hdc, &mut doc_info);
        StartPage(hdc);

        // 이미지를 프린터로 전송
        let bmi = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: img.width() as i32,
                biHeight: -(img.height() as i32),
                biPlanes: 1,
                biBitCount: 32,
                biCompression: 0,
                biSizeImage: 0,
                biXPelsPerMeter: 0,
                biYPelsPerMeter: 0,
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
    }

    Ok(())
}
