#--------- Stage 1: Build -----------
FROM rust:1.82 as builder

# Set the working directory inside the container
WORKDIR /app

# Copy Cargo.toml and Cargo.lock first to cache dependencies
COPY Cargo.toml Cargo.lock ./

# Create an empty source folder (for caching dependency builds)
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies only (for faster rebuilds later)
RUN cargo build --release || true

# Now copy the actual source code
COPY . .

# --------- Stage 2: Runtime ----------
FROM debian:bookworm-slim

# Install minimal runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*

# Create working directory
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/chordcalc .

# Expose a port if the app listens on a port. Example: EXPORT 8080

# Default command
CMD ["./chordcalc"]