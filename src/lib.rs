use wasm_bindgen::prelude::*;
use rxing::{BarcodeFormat, MultiFormatWriter, Writer};
use image::{ImageBuffer, Luma};
use svg::Document;


#[wasm_bindgen]
pub enum BarcodeType {
    QRCode,
    DataMatrix,
    PDF417,
    EAN13,
    CODE39,
    CODE93,
    CODE128,
    AZTEC,
    CODA,
    ITF,
    EAN8,
    UPCA,
    UPCE,
    TELEPEN,
}

#[wasm_bindgen]
pub enum OutputFormat {
    PNG,
    JPEG,
    SVG,
}

#[wasm_bindgen]
pub fn generate_barcode(data: &str, barcode_type: BarcodeType, width: u32, height: u32, output_format: OutputFormat) -> Result<Vec<u8>, JsValue> {
    let format = match barcode_type {
        BarcodeType::QRCode => BarcodeFormat::QR_CODE,
        BarcodeType::DataMatrix => BarcodeFormat::DATA_MATRIX,
        BarcodeType::PDF417 => BarcodeFormat::PDF_417,
        BarcodeType::EAN13 => BarcodeFormat::EAN_13,
        BarcodeType::CODE128 => BarcodeFormat::CODE_128,
        BarcodeType::AZTEC => BarcodeFormat::AZTEC,
        BarcodeType::CODA => BarcodeFormat::CODABAR,
        BarcodeType::ITF => BarcodeFormat::ITF,
        BarcodeType::EAN8 => BarcodeFormat::EAN_8,
        BarcodeType::UPCA => BarcodeFormat::UPC_A,
        BarcodeType::UPCE => BarcodeFormat::UPC_E,
        BarcodeType::TELEPEN => BarcodeFormat::TELEPEN,
        BarcodeType::CODE39 => BarcodeFormat::CODE_39,
        BarcodeType::CODE93 => BarcodeFormat::CODE_93,
    };

    let writer = MultiFormatWriter::default();
    let matrix = writer.encode(data, &format, width as i32, height as i32)
        .map_err(|e| JsValue::from_str(&format!("Failed to generate barcode: {}", e)))?;

    match output_format {
        OutputFormat::PNG | OutputFormat::JPEG => {
            let img = ImageBuffer::from_fn(width, height, |x, y| {
                if matrix.get(x as u32, y as u32) {
                    Luma([0u8])
                } else {
                    Luma([255u8])
                }
            });

            let mut buffer = Vec::new();
            let format = match output_format {
                OutputFormat::PNG => image::ImageFormat::Png,
                OutputFormat::JPEG => image::ImageFormat::Jpeg,
                _ => return Err(JsValue::from_str("Unsupported output format")),
            };
            img.write_to(&mut std::io::Cursor::new(&mut buffer), format)
                .map_err(|e| JsValue::from_str(&format!("Failed to encode image: {}", e)))?;

            Ok(buffer)
        },
        OutputFormat::SVG => {
            let mut document = Document::new()
                .set("viewBox", (0, 0, width, height))
                .set("width", width)
                .set("height", height);
            for y in 0..height {
                for x in 0..width {
                    if matrix.get(x as u32, y as u32) {
                        let rect = svg::node::element::Rectangle::new()
                            .set("x", x)
                            .set("y", y)
                            .set("width", 1)
                            .set("height", 1)
                            .set("fill", "black");
                        document = document.add(rect);
                    }
                }
            }

            let svg_data = document.to_string();
            Ok(svg_data.into_bytes())
        },
    }
}