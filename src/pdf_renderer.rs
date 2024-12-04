use slint::{Image, ComponentHandle}; 
use pdfium_render::prelude::*; 

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

    pub fn render_current_page(&self) -> Image {
        let page = self.pdf_document.pages().get(self.current_page as u16).unwrap();
        let render_config = PdfRenderConfig::new().set_target_width(800).set_maximum_height(800);
        let image = page.render_with_config(&render_config).unwrap().as_image().into_rgb8();

        Image::from_rgba8(image.into_raw())
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



