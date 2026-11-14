# ==========================================
# Stage 1: Build Stage
# ==========================================
FROM rust:slim AS builder

WORKDIR /usr/src/app

# Copy dependency manifests first to cache dependencies
COPY app/Cargo.toml app/Cargo.lock* ./

# Create a dummy source file to build dependencies and cache them
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy real source code
COPY app/src ./src

# Touch main.rs to force a fresh build of our actual code
RUN touch src/main.rs
RUN cargo build --release

# ==========================================
# Stage 2: Runtime Stage
# ==========================================
FROM debian:bookworm-slim

# Install runtime dependencies (ca-certificates for HTTPS calls if needed)
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root system user and group for security
RUN groupadd -g 10001 appuser && \
    useradd -u 10001 -g appuser -ms /bin/bash appuser

WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/status-api /app/status-api

# Change ownership of the app directory to the non-root user
RUN chown -R appuser:appuser /app

# Switch to the non-root user
USER appuser

# Expose port 8080 to traffic
EXPOSE 8080

# Command to run the microservice
CMD ["./status-api"]
