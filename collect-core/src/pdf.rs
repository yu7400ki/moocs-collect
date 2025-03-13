use crate::moocs::SlideContent;
use lopdf::dictionary;
use lopdf::{Document, Object};
use rayon::prelude::*;
use svg2pdf::{
    to_pdf,
    usvg::{fontdb, Options, Tree},
    ConversionOptions, PageOptions,
};

pub fn convert(slide: &SlideContent) -> anyhow::Result<Document> {
    let mut db = fontdb::Database::new();
    db.load_system_fonts();
    let options = Options::default();
    let documents = slide
        .content
        .par_iter()
        .map(|src| {
            let conversion_options = ConversionOptions::default();
            let page_options = PageOptions::default();
            let tree = Tree::from_str(&src.src, &options, &db).unwrap();
            let pdf = to_pdf(&tree, conversion_options, page_options, &db);
            Document::load_mem(&pdf).map_err(anyhow::Error::from)
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    let pdf = merge(documents)?;

    Ok(pdf)
}

fn merge(documents: Vec<Document>) -> anyhow::Result<Document> {
    let mut merged = Document::with_version("1.5");
    let mut document_pages = vec![];

    for mut document in documents {
        document.renumber_objects_with(merged.max_id + 1);
        merged.max_id = document.max_id;

        for (_, object_id) in document.get_pages() {
            let reference = Object::Reference(object_id);
            document_pages.push(reference);
        }

        merged.objects.extend(document.objects);
    }

    let count = document_pages.len() as u32;
    let pages = lopdf::dictionary! {
        "Type" => "Pages",
        "Kids" => document_pages,
        "Count" => count,
    };
    let pages_id = merged.new_object_id();
    merged.objects.insert(pages_id, Object::Dictionary(pages));

    let catalog_id = merged.new_object_id();
    let catalog = lopdf::dictionary! {
        "Type" => "Catalog",
        "Pages" => pages_id,
    };

    merged
        .objects
        .insert(catalog_id, Object::Dictionary(catalog));
    merged.trailer.set("Root", catalog_id);

    Ok(merged)
}
