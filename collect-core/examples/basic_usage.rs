use collect_core::domain::models::{Credentials, Year};
use collect_core::Collect;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get command line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <username> <password>", args[0]);
        std::process::exit(1);
    }

    let username = &args[1];
    let password = &args[2];

    // Create collector with default configuration
    let collect = Collect::default();

    // Authenticate with MOOCs system
    let credentials = Credentials::new(username, password);

    collect.login_moocs(&credentials).await?;
    collect.login_google(&credentials).await?;

    // Check authentication status
    let auth_status = collect.is_authenticated().await?;
    println!("MOOCs authenticated: {}", auth_status.moocs_authenticated);
    println!("Google authenticated: {}", auth_status.google_authenticated);

    // Get courses for a specific year
    let year = Year::new(2024)?;
    let courses = collect.get_courses(Some(year)).await?;
    println!("Found {} courses", courses.len());

    // Get lectures for the first course
    if let Some(course) = courses.first() {
        // Get lectures grouped by category
        let lecture_groups = collect.get_lecture_groups(&course.key).await?;
        for group in lecture_groups.iter() {
            println!(
                "Group '{}' has {} lectures",
                group.display_name(),
                group.lectures.len()
            );
        }
        let lectures = lecture_groups
            .into_iter()
            .flat_map(|group| group.lectures)
            .collect::<Vec<_>>();

        println!(
            "Course '{}' has {} lectures",
            course.display_name(),
            lectures.len()
        );

        // Get pages for the first lecture
        if let Some(lecture) = lectures.first() {
            let lecture_key = &lecture.key;
            let pages = collect.get_pages(lecture_key).await?;
            println!(
                "Found {} pages for lecture: {}",
                pages.len(),
                lecture.display_name()
            );

            // Get slides for the first page
            if let Some(page) = pages.first() {
                let page_key = &page.key;
                let slides = collect.get_slides(page_key).await?;
                println!(
                    "Found {} slides for page: {}",
                    slides.len(),
                    page.display_name()
                );

                // Get content for the first slide (requires Google authentication)
                if let Some(slide) = slides.first() {
                    match collect.get_slide_content(slide).await {
                        Ok(content) => {
                            println!("Slide Pages: {}", content.svgs.len());
                        }
                        Err(e) => {
                            println!("Failed to get slide content: {}", e);
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
