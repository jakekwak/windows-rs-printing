use image::{ImageBuffer, Rgba, RgbaImage};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};
use std::io::Error;
use windows::core::{HSTRING, PCWSTR};
use windows::Win32::Graphics::Gdi::{
    CreateDCW, DeleteDC, SetDIBitsToDevice, BITMAPINFO, BITMAPINFOHEADER, DIB_RGB_COLORS,
};
use windows::Win32::Storage::Xps::{EndDoc, EndPage, StartDocW, StartPage, DOCINFOW};

fn main() -> Result<(), Error> {
    // 이미지 생성 (영수증 크기에 맞게 조정)
    // let mut img: RgbaImage = ImageBuffer::from_pixel(800, 600, Rgba([255, 255, 255, 255]));
    let mut img: RgbaImage = ImageBuffer::new(800, 600);
    img.fill(255);

    // 폰트 로드
    let font = Font::try_from_bytes(include_bytes!("C:\\Windows\\Fonts\\K_malgun.ttf")).unwrap();
    let font_bold =
        Font::try_from_bytes(include_bytes!("C:\\Windows\\Fonts\\K_malgunbd.ttf")).unwrap();

    // 텍스트 추가 함수
    let mut y_offset = 10;
    let line_height = 20;

    let mut add_text = |text: &str, size: f32, is_bold: bool, y: &mut i32| {
        let scale = Scale::uniform(size);
        let font_to_use = if is_bold { &font_bold } else { &font };
        draw_text_mut(
            &mut img,
            Rgba([0, 0, 0, 255]),
            10,
            *y,
            scale,
            font_to_use,
            text,
        );
        *y += line_height;
    };

    // 영수증 내용 추가
    add_text("Kitchen #1", 16.0, true, &mut y_offset);
    y_offset += 20;
    add_text("TABLE M1", 14.0, true, &mut y_offset);
    add_text("ORDER #1-1", 14.0, true, &mut y_offset);
    y_offset += 20;
    add_text(
        "Invoice #1    Mon, 9/23/2024 6:37 PM",
        12.0,
        false,
        &mut y_offset,
    );
    y_offset += 20;

    add_text("1 Avocado Eggrolls", 12.0, false, &mut y_offset);
    add_text("아보카도 에그롤", 12.0, false, &mut y_offset);
    add_text("1 Make It Gluten Free", 12.0, false, &mut y_offset);
    add_text("1 Diet Coke", 12.0, false, &mut y_offset);
    add_text("1 Yes", 12.0, false, &mut y_offset);
    add_text("1 French Fries", 12.0, false, &mut y_offset);
    add_text("감자 튀김", 12.0, false, &mut y_offset);
    add_text("1 Petite Filet", 12.0, false, &mut y_offset);
    add_text("뼈때 필레", 12.0, false, &mut y_offset);
    add_text("1 Medium Rare", 12.0, false, &mut y_offset);
    add_text("1 Make It Gluten Free", 12.0, false, &mut y_offset);

    y_offset += 40;
    add_text("Printed 6:37 PM", 12.0, false, &mut y_offset);

    // 프린터 설정
    unsafe {
        let printer_name = HSTRING::from("Receipt"); // 프린터 이름을 적절히 변경하세요
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

        // 리소스 해제
        DeleteDC(hdc);
    }

    Ok(())
}
