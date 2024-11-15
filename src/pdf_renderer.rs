use dotenv::dotenv;
use std::env;
use pdfium_render::prelude::*;

fn export_pdf_to_jpegs(path: &impl AsRef<Path>, password: Option<&str>) -> Result<(), PdfiumError> {

    let pdfium = Pdfium::default();

    let document = pdfium.load_pdf_from_file(path, password)?;

    let render_config = PdfRenderConfig::new()
        .set_target_width(2000)
        .set_maximum_height(2000)
        .rotate_if_landscape(PdfPageRenderRotation::Degrees90, true);

    for (index, page) in document.pages().iter().enumerate() {
        page.render_with_config(&render_config)?
            .as_image()
            .into_rgb8()
            .save_with_format(
                format!("test-page-{}.jpg", index),
                image::ImageFormat::Jpeg
            )
            .map_err(|_| Pdfium::ImageError)?;
    }

    Ok(())
}





