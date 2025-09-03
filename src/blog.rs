use pulldown_cmark::{html, Parser};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use tera::{Tera, Context};

#[derive(Debug, Deserialize, Serialize)]
pub struct PostMetadata {
    pub title: String,
    pub date: String,
    pub author: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct Post {
    pub metadata: PostMetadata,
    pub content: String,
    pub slug: String,
}

pub fn generate_site() -> Result<(), Box<dyn std::error::Error>> {
    println!("Generating static site...");

    let tera = Tera::new("templates/**/*")?;

    fs::create_dir_all("output")?;

    let posts = read_posts("content/posts")?;

    for post in &posts {
        generate_post_page(post, &tera)?;
    }

    generate_index_page(&posts, &tera)?;

    copy_static_files()?;

    println!(
        "Site generated successfully! {} posts processed.",
        posts.len()
    );
    Ok(())
}

fn read_posts(posts_dir: &str) -> Result<Vec<Post>, Box<dyn std::error::Error>> {
    let mut posts = Vec::new();

    if !Path::new(posts_dir).exists() {
        println!("Creating posts directory: {}", posts_dir);
        fs::create_dir_all(posts_dir)?;
        return Ok(posts);
    }

    for entry in fs::read_dir(posts_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("md") {
            println!("Processing post: {}", path.display());
            let content = fs::read_to_string(&path)?;
            let post = parse_post(content, &path)?;
            posts.push(post);
        }
    }

    // Sort posts by date (newest first)
    posts.sort_by(|a, b| b.metadata.date.cmp(&a.metadata.date));

    Ok(posts)
}

fn parse_post(content: String, path: &Path) -> Result<Post, Box<dyn std::error::Error>> {
    let parts: Vec<&str> = content.splitn(3, "---").collect();

    if parts.len() < 3 {
        return Err("Invalid post format: missing frontmatter".into());
    }

    let metadata: PostMetadata = serde_yaml::from_str(parts[1].trim())?;
    let markdown_content = parts[2].trim();

    // Convert markdown to HTML
    let parser = Parser::new(markdown_content);
    let mut html_content = String::new();
    html::push_html(&mut html_content, parser);

    let slug = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("untitled")
        .to_string();

    Ok(Post {
        metadata,
        content: html_content,
        slug,
    })
}

fn generate_post_page(post: &Post, tera: &Tera) -> Result<(), Box<dyn std::error::Error>> {
    let mut context = Context::new();
    context.insert("post", post);

    let html = tera.render("post.html", &context)?;
    fs::write(format!("output/{}.html", post.slug), html)?;
    Ok(())
}

fn generate_index_page(posts: &[Post], tera: &Tera) -> Result<(), Box<dyn std::error::Error>> {
    let mut context = Context::new();
    context.insert("posts", posts);

    let html = tera.render("index.html", &context)?;
    fs::write("output/index.html", html)?;
    Ok(())
}

fn copy_static_files() -> Result<(), Box<dyn std::error::Error>> {
    if Path::new("static").exists() {
        for entry in fs::read_dir("static")? {
            let entry = entry?;
            let file_name = entry.file_name();
            let source = entry.path();
            let destination = format!("output/{}", file_name.to_string_lossy());
            fs::copy(&source, &destination)?;
            println!("Copied {} to {}", source.display(), destination);
        }
    } else {
        println!("No static directory found, creating one...");
        fs::create_dir_all("static")?;
    }
    Ok(())
}
