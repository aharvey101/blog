use pulldown_cmark::{html, Parser};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
pub struct PostMetadata {
    pub title: String,
    pub date: String,
    pub author: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug)]
pub struct Post {
    pub metadata: PostMetadata,
    pub content: String,
    pub slug: String,
}

pub fn generate_site() -> Result<(), Box<dyn std::error::Error>> {
    println!("Generating static site...");

    // Create output directory
    fs::create_dir_all("output")?;

    // Read and process markdown files
    let posts = read_posts("content/posts")?;

    // Generate individual post pages
    for post in &posts {
        generate_post_page(post)?;
    }

    // Generate index page with post list
    generate_index_page(&posts)?;

    // Copy static assets
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

fn generate_post_page(post: &Post) -> Result<(), Box<dyn std::error::Error>> {
    let template = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{title} - My Blog</title>
    <link rel="stylesheet" href="/style.css">
</head>
<body>
    <header>
        <nav>
            <a href="/">← Back to Home</a>
        </nav>
    </header>
    <main>
        <article>
            <h1>{title}</h1>
            <p class="meta">Published on {date}{author}</p>
            <div class="content">
                {content}
            </div>
        </article>
    </main>
</body>
</html>"#;

    let author_text = match &post.metadata.author {
        Some(author) => format!(" by {}", author),
        None => String::new(),
    };

    let html = template
        .replace("{title}", &post.metadata.title)
        .replace("{date}", &post.metadata.date)
        .replace("{author}", &author_text)
        .replace("{content}", &post.content);

    fs::write(format!("output/{}.html", post.slug), html)?;
    Ok(())
}

fn generate_index_page(posts: &[Post]) -> Result<(), Box<dyn std::error::Error>> {
    let mut posts_html = String::new();

    if posts.is_empty() {
        posts_html = r#"<div class="no-posts">
            <p>No blog posts found. Create your first post in the <code>content/posts/</code> directory!</p>
            <p>Example: <code>content/posts/2024-01-01-hello-world.md</code></p>
        </div>"#.to_string();
    } else {
        for post in posts {
            let author_text = match &post.metadata.author {
                Some(author) => format!(" by {}", author),
                None => String::new(),
            };

            posts_html.push_str(&format!(
                r#"<article class="post-preview">
                    <h2><a href="/{}.html">{}</a></h2>
                    <p class="meta">{}{}</p>
                </article>"#,
                post.slug, post.metadata.title, post.metadata.date, author_text
            ));
        }
    }

    let template = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>My Rust Blog</title>
    <link rel="stylesheet" href="/style.css">
</head>
<body>
    <header>
        <h1>My Rust Blog</h1>
        <p class="subtitle">A simple static blog built with Rust</p>
    </header>
    <main>
        {posts}
    </main>
    <footer>
        <p>Generated with Rust 🦀</p>
    </footer>
</body>
</html>"#;

    let html = template.replace("{posts}", &posts_html);
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
