# Rust Static Blog

A simple static blog generator and server built with Rust.

## Features

- 🦀 Written in Rust for performance and safety
- 📝 Markdown support with YAML frontmatter
- 🎨 Clean, responsive design
- 🚀 Built-in development server
- 🐳 Docker support

## Local Development

### Prerequisites
- Rust 1.75 or later
- Cargo

### Running locally
```bash
# Clone the repository
git clone <your-repo-url>
cd blog

# Run the blog (builds and starts server)
cargo run

# The blog will be available at http://localhost:8080
```

### Adding new posts
Create a new Markdown file in `content/posts/` with this format:

```markdown
---
title: "Your Post Title"
date: "YYYY-MM-DD"
author: "Your Name"
tags: ["tag1", "tag2"]
---

Your markdown content here...
```

## Docker Deployment

### Building the Docker image
```bash
# Build the image
docker build -t rust-blog .

# Run the container
docker run -p 8080:8080 rust-blog
```

### Using Docker Compose
```bash
# Start the service
docker-compose up -d

# View logs
docker-compose logs -f

# Stop the service
docker-compose down
```

### Environment Variables

- `BIND_ADDRESS`: Server bind address (default: `0.0.0.0:8080`)

## Project Structure

```
blog/
├── content/posts/           # Your Markdown blog posts
├── static/                  # CSS, images, and other assets
├── output/                  # Generated HTML files (auto-generated)
├── src/
│   ├── main.rs             # Entry point
│   ├── blog.rs             # Static site generation
│   └── server.rs           # Development server
├── Dockerfile              # Docker configuration
├── docker-compose.yml      # Docker Compose configuration
└── Cargo.toml              # Rust dependencies
```

## Customization

### Styling
Edit `static/style.css` to customize the appearance of your blog.

### Templates
The HTML templates are currently embedded in `src/blog.rs`. You can modify the `generate_post_page()` and `generate_index_page()` functions to customize the HTML output.

## Production Deployment

For production deployment, consider:

1. Using a reverse proxy like Nginx
2. Adding HTTPS/TLS termination
3. Setting up proper logging
4. Implementing health checks

Example nginx configuration:
```nginx
server {
    listen 80;
    server_name yourdomain.com;
    
    location / {
        proxy_pass http://localhost:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

## License

This project is open source and available under the [MIT License](LICENSE).
