# Simple Dockerfile for Rust blog
FROM rust:1.89

WORKDIR /app

# Copy everything
COPY . .

# Build the application
RUN cargo build --release

# Expose port 8080
EXPOSE 8080

ENV BIND_ADDRESS="127.0.0.1:8080"

# Run the blog
CMD ["./target/release/blog"]
