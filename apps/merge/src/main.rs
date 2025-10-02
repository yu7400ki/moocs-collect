use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use dialoguer::Select;
use indicatif::{ProgressBar, ProgressStyle};
use lopdf::{Bookmark, Document, Object, ObjectId};
use regex::Regex;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone, ValueEnum)]
enum MergeLevel {
    Lecture,
    Course,
}

#[derive(Parser, Debug)]
#[command(author, version, about = "PDF merge CLI tool", long_about = None)]
struct Cli {
    #[arg(value_enum)]
    level: MergeLevel,

    #[arg(short, long)]
    path: PathBuf,
}

#[derive(Debug, Clone)]
struct PdfInfo {
    path: PathBuf,
    course_name: String,
    _lecture_group: String,
    lecture_name: String,
    page_slug: String,
    page_title: String,
    page_number: usize,
}

fn parse_pdf_path(path: &Path) -> Result<PdfInfo> {
    let path_str = path.to_string_lossy();
    let normalized = path_str.replace('\\', "/");

    // Try Desktop format first: year/course/lecture/page_title (number).pdf
    let desktop_re = Regex::new(
        r"(?P<year>\d{4})/(?P<course>[^/]+)/(?P<lecture>[^/]+)/(?P<page_title>[^(]+?)(?:\s+\((?P<page_number>\d+)\))?\.pdf$",
    )?;

    if let Some(caps) = desktop_re.captures(&normalized) {
        let page_title = caps["page_title"].trim();
        let page_slug = page_title
            .split(&[' ', '-'][..])
            .next()
            .unwrap_or(page_title)
            .to_string();

        return Ok(PdfInfo {
            path: path.to_path_buf(),
            course_name: caps["course"].trim().to_string(),
            _lecture_group: String::new(),
            lecture_name: caps["lecture"].trim().to_string(),
            page_slug,
            page_title: page_title.to_string(),
            page_number: caps
                .name("page_number")
                .map(|m| m.as_str().parse::<usize>().unwrap_or(1))
                .unwrap_or(1),
        });
    }

    // Try CLI format: course/lecture_group - lecture_name/page_slug - page_title (number).pdf
    let cli_re = Regex::new(
        r"(?P<course>[^/]+)/(?P<lecture_group>[^/]+)\s*-\s*(?P<lecture_name>[^/]+)/(?P<page_slug>[^/\s-]+)\s*-\s*(?P<page_title>[^(]+?)(?:\s+\((?P<page_number>\d+)\))?\.pdf$",
    )?;

    if let Some(caps) = cli_re.captures(&normalized) {
        return Ok(PdfInfo {
            path: path.to_path_buf(),
            course_name: caps["course"].trim().to_string(),
            _lecture_group: caps["lecture_group"].trim().to_string(),
            lecture_name: caps["lecture_name"].trim().to_string(),
            page_slug: caps["page_slug"].trim().to_string(),
            page_title: caps["page_title"].trim().to_string(),
            page_number: caps
                .name("page_number")
                .map(|m| m.as_str().parse::<usize>().unwrap_or(1))
                .unwrap_or(1),
        });
    }

    anyhow::bail!(
        "ファイル名のパターンが一致しません:\n\
         - Desktop形式: year/course/lecture/page_title (number).pdf\n\
         - CLI形式: course/lecture_group - lecture_name/page_slug - page_title (number).pdf"
    )
}

fn collect_pdfs(dir: &Path) -> Result<Vec<PdfInfo>> {
    let mut pdfs = Vec::new();

    println!("PDFファイルを検索中...");
    let entries: Vec<_> = WalkDir::new(dir).into_iter().collect();
    let pb = ProgressBar::new(entries.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({msg})",
            )
            .unwrap()
            .progress_chars("#>-"),
    );

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.extension().is_some_and(|ext| ext == "pdf") {
            if let Ok(info) = parse_pdf_path(path) {
                pb.set_message(format!("発見: {}", info.page_title));
                pdfs.push(info);
            }
        }
        pb.inc(1);
    }

    pb.finish_with_message(format!("{}個のPDFファイルを発見", pdfs.len()));
    Ok(pdfs)
}

fn filter_by_lecture(pdfs: Vec<PdfInfo>, course_name: &str, lecture_name: &str) -> Vec<PdfInfo> {
    let mut filtered: Vec<PdfInfo> = pdfs
        .into_iter()
        .filter(|pdf| pdf.course_name == course_name && pdf.lecture_name == lecture_name)
        .collect();

    filtered.sort_by(|a, b| {
        a.page_slug
            .cmp(&b.page_slug)
            .then_with(|| a.page_number.cmp(&b.page_number))
    });

    filtered
}

fn filter_by_course(pdfs: Vec<PdfInfo>, course_name: &str) -> Vec<PdfInfo> {
    let mut filtered: Vec<PdfInfo> = pdfs
        .into_iter()
        .filter(|pdf| pdf.course_name == course_name)
        .collect();

    filtered.sort_by(|a, b| {
        a.lecture_name
            .cmp(&b.lecture_name)
            .then_with(|| a.page_slug.cmp(&b.page_slug))
            .then_with(|| a.page_number.cmp(&b.page_number))
    });

    filtered
}

fn merge_pdfs(pdfs: &[PdfInfo], output_path: &Path, level: &MergeLevel) -> Result<()> {
    if pdfs.is_empty() {
        anyhow::bail!("結合するPDFファイルがありません");
    }

    let mut max_id = 1;
    let mut documents_pages: BTreeMap<ObjectId, Object> = BTreeMap::new();
    let mut documents_objects: BTreeMap<ObjectId, Object> = BTreeMap::new();
    let mut document = Document::with_version("1.5");

    match level {
        MergeLevel::Lecture => {
            let mut current_page_slug = String::new();
            let mut page_bookmark = None;

            for pdf_info in pdfs {
                let mut doc = Document::load(&pdf_info.path)?;
                doc.renumber_objects_with(max_id);
                max_id = doc.max_id + 1;

                let pages = doc.get_pages();
                let first_object = pages.values().next().copied().unwrap_or((0, 0));

                if current_page_slug != pdf_info.page_slug {
                    let title = format!("{} - {}", pdf_info.page_slug, pdf_info.page_title);
                    page_bookmark = Some(document.add_bookmark(
                        Bookmark::new(title, [0.0, 0.0, 0.0], 0, first_object),
                        None,
                    ));
                    current_page_slug = pdf_info.page_slug.clone();
                }

                if pdf_info.page_number > 1 {
                    let title = format!("({}) {}", pdf_info.page_number, pdf_info.page_title);
                    document.add_bookmark(
                        Bookmark::new(title, [0.0, 0.0, 0.0], 0, first_object),
                        page_bookmark,
                    );
                }

                for (_, object_id) in doc.get_pages() {
                    if let Ok(page_obj) = doc.get_object(object_id) {
                        documents_pages.insert(object_id, page_obj.to_owned());
                    }
                }
                documents_objects.extend(doc.objects);
            }
        }
        MergeLevel::Course => {
            let mut current_lecture = String::new();
            let mut lecture_bookmark = None;
            let mut current_page_slug = String::new();
            let mut page_bookmark = None;

            for pdf_info in pdfs {
                let mut doc = Document::load(&pdf_info.path)?;
                doc.renumber_objects_with(max_id);
                max_id = doc.max_id + 1;

                let pages = doc.get_pages();
                let first_object = pages.values().next().copied().unwrap_or((0, 0));

                if current_lecture != pdf_info.lecture_name {
                    lecture_bookmark = Some(document.add_bookmark(
                        Bookmark::new(
                            pdf_info.lecture_name.clone(),
                            [0.0, 0.0, 0.0],
                            0,
                            first_object,
                        ),
                        None,
                    ));
                    current_lecture = pdf_info.lecture_name.clone();
                    current_page_slug.clear();
                }

                if current_page_slug != pdf_info.page_slug {
                    let title = format!("{} - {}", pdf_info.page_slug, pdf_info.page_title);
                    page_bookmark = Some(document.add_bookmark(
                        Bookmark::new(title, [0.0, 0.0, 0.0], 0, first_object),
                        lecture_bookmark,
                    ));
                    current_page_slug = pdf_info.page_slug.clone();
                }

                if pdf_info.page_number > 1 {
                    let title = format!("({}) {}", pdf_info.page_number, pdf_info.page_title);
                    document.add_bookmark(
                        Bookmark::new(title, [0.0, 0.0, 0.0], 0, first_object),
                        page_bookmark,
                    );
                }

                for (_, object_id) in doc.get_pages() {
                    if let Ok(page_obj) = doc.get_object(object_id) {
                        documents_pages.insert(object_id, page_obj.to_owned());
                    }
                }
                documents_objects.extend(doc.objects);
            }
        }
    }

    let mut catalog_object: Option<(ObjectId, Object)> = None;
    let mut pages_object: Option<(ObjectId, Object)> = None;

    for (object_id, object) in documents_objects {
        match object.type_name().unwrap_or(b"") {
            b"Catalog" => {
                catalog_object = Some((
                    catalog_object.map(|(id, _)| id).unwrap_or(object_id),
                    object,
                ));
            }
            b"Pages" => {
                if let Ok(dictionary) = object.as_dict() {
                    let mut dictionary = dictionary.clone();
                    if let Some((_, ref obj)) = pages_object {
                        if let Ok(old_dict) = obj.as_dict() {
                            dictionary.extend(old_dict);
                        }
                    }
                    pages_object = Some((
                        pages_object.map(|(id, _)| id).unwrap_or(object_id),
                        Object::Dictionary(dictionary),
                    ));
                }
            }
            b"Page" => {}
            b"Outlines" => {}
            b"Outline" => {}
            _ => {
                document.objects.insert(object_id, object);
            }
        }
    }

    let (page_id, page_object) = pages_object.context("Pagesオブジェクトが見つかりません")?;

    for (&object_id, object) in documents_pages.iter() {
        if let Ok(dictionary) = object.as_dict() {
            let mut dictionary = dictionary.clone();
            dictionary.set("Parent", page_id);
            document
                .objects
                .insert(object_id, Object::Dictionary(dictionary));
        }
    }

    let (catalog_id, catalog_object) =
        catalog_object.context("Catalogオブジェクトが見つかりません")?;

    if let Ok(dictionary) = page_object.as_dict() {
        let mut dictionary = dictionary.clone();
        dictionary.set("Count", documents_pages.len() as u32);
        dictionary.set(
            "Kids",
            documents_pages
                .keys()
                .map(|&id| Object::Reference(id))
                .collect::<Vec<_>>(),
        );
        document
            .objects
            .insert(page_id, Object::Dictionary(dictionary));
    }

    if let Ok(dictionary) = catalog_object.as_dict() {
        let mut dictionary = dictionary.clone();
        dictionary.set("Pages", page_id);
        dictionary.set("PageMode", "UseOutlines");
        dictionary.remove(b"Outlines");
        document
            .objects
            .insert(catalog_id, Object::Dictionary(dictionary));
    }

    document.trailer.set("Root", catalog_id);
    document.max_id = document.objects.len() as u32;
    document.renumber_objects();
    document.adjust_zero_pages();

    if let Some(outline_id) = document.build_outline() {
        if let Ok(Object::Dictionary(dict)) = document.get_object_mut(catalog_id) {
            dict.set("Outlines", Object::Reference(outline_id));
        }
    }

    document.save(output_path)?;
    Ok(())
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let all_pdfs = collect_pdfs(&args.path)?;

    if all_pdfs.is_empty() {
        anyhow::bail!("指定されたディレクトリにPDFファイルが見つかりません");
    }

    match args.level {
        MergeLevel::Lecture => {
            let mut courses: Vec<String> =
                all_pdfs.iter().map(|pdf| pdf.course_name.clone()).collect();
            courses.sort();
            courses.dedup();

            if courses.is_empty() {
                anyhow::bail!("コースが見つかりません");
            }

            let selection = Select::new()
                .with_prompt("コースを選択してください")
                .items(&courses)
                .default(0)
                .interact()?;

            let selected_course = &courses[selection];

            let mut lectures: Vec<String> = all_pdfs
                .iter()
                .filter(|pdf| &pdf.course_name == selected_course)
                .map(|pdf| pdf.lecture_name.clone())
                .collect();
            lectures.sort();
            lectures.dedup();

            if lectures.is_empty() {
                anyhow::bail!("選択されたコースに講義が見つかりません");
            }

            let pb = ProgressBar::new(lectures.len() as u64);
            pb.set_style(
                ProgressStyle::default_bar()
                    .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
                    .unwrap()
                    .progress_chars("#>-"),
            );

            for lecture in lectures {
                let filtered = filter_by_lecture(all_pdfs.clone(), selected_course, &lecture);

                if filtered.is_empty() {
                    pb.inc(1);
                    continue;
                }

                let output_filename = format!("{lecture}.pdf");

                let course_dir = filtered[0]
                    .path
                    .parent()
                    .and_then(|p| p.parent())
                    .context("コースディレクトリの取得に失敗しました")?;

                let output_path = course_dir.join(&output_filename);

                pb.set_message(format!("結合中: {lecture}"));
                merge_pdfs(&filtered, &output_path, &args.level)?;
                pb.inc(1);
            }

            pb.finish_with_message("すべての講義の結合が完了しました");
        }
        MergeLevel::Course => {
            let mut courses: Vec<String> =
                all_pdfs.iter().map(|pdf| pdf.course_name.clone()).collect();
            courses.sort();
            courses.dedup();

            if courses.is_empty() {
                anyhow::bail!("コースが見つかりません");
            }

            let selection = Select::new()
                .with_prompt("コースを選択してください")
                .items(&courses)
                .default(0)
                .interact()?;

            let course = &courses[selection];
            let filtered = filter_by_course(all_pdfs, course);

            if filtered.is_empty() {
                anyhow::bail!("一致するPDFファイルが見つかりません");
            }

            let output_filename = format!("{course}.pdf");

            let year_dir = filtered[0]
                .path
                .parent()
                .and_then(|p| p.parent())
                .and_then(|p| p.parent())
                .context("年ディレクトリの取得に失敗しました")?;

            let output_path = year_dir.join(&output_filename);

            let pb = ProgressBar::new_spinner();
            pb.set_style(
                ProgressStyle::default_spinner()
                    .template("{spinner:.green} {msg}")
                    .unwrap(),
            );
            pb.set_message(format!(
                "{}個のファイルを{}に結合中",
                filtered.len(),
                output_filename
            ));

            merge_pdfs(&filtered, &output_path, &args.level)?;

            pb.finish_with_message(format!("完了: {}", output_path.display()));
        }
    }

    Ok(())
}
