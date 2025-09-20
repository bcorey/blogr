# Use the official Rust image as the base image
FROM rust:1.75 as chef
# Install cargo-chef for optimized Docker layer caching
RUN cargo install cargo-chef
WORKDIR /app

# Prepare the build plan
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Build dependencies (this layer will be cached)
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Cook dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json

# Copy source code and build the application
COPY . .
RUN cargo build --release --bin blogr

# Runtime image
FROM debian:bookworm-slim AS runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    git \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN useradd -m -u 1001 blogr

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/blogr /usr/local/bin/blogr

# Set ownership and permissions
RUN chown blogr:blogr /usr/local/bin/blogr && chmod +x /usr/local/bin/blogr

# Switch to non-root user
USER blogr

# Set working directory
WORKDIR /home/blogr

# Expose the default port for the development server
EXPOSE 3000

# Set the default command
CMD ["blogr", "--help"]
