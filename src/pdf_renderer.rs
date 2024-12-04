use slint::{Image, ComponentHandle, SharedPixelBuffer}; 
use pdfium_render::prelude::*; 
use image::{RgbaImage, ImageBuffer, Rgba};

pub struct PDFViewer<'a> {
    pdf_document: PdfDocument<'a>, 
    current_page: usize,       
}

impl<'a> PDFViewer<'a> {
    pub fn new(file_path: &str) -> Result<Self, PdfiumError> {
        let pdfium = Pdfium::default();
        let document = pdfium.load_pdf_from_file(file_path, None)?;
        Ok(Self {
            pdf_document: document,
            current_page: 0,
        })
    }

    fn imagebuff_to_sharedpix(image: RgbaImage) -> SharedPixelBuffer<Rgba<u8>> {
        let (width, height) = image.dimensions();
        let raw_data = image.into_raw(); // Extract raw RGBA data
    
        // Create a SharedPixelBuffer with the pixel data
        SharedPixelBuffer::from_raw(raw_data, width as usize, height as usize)
            .expect("Failed to create SharedPixelBuffer")
    }
    pub fn render_current_page(&self) -> Image {
        let page = self.pdf_document.pages().get(self.current_page as u16).unwrap();
        let render_config = PdfRenderConfig::new().set_target_width(800).set_maximum_height(800);
        let image = page.render_with_config(&render_config).unwrap().as_image().into_rgba8();

        Image::from_rgba8(Self::imagebuff_to_sharedpix(image))
    }

    pub fn navigate_previous(&mut self) {
        if self.current_page > 0 {
            self.current_page -= 1;
        }
    }

    pub fn navigate_next(&mut self) {
        if self.current_page + 1 < self.pdf_document.pages().len().into() {
            self.current_page += 1;
        }
    }
}



