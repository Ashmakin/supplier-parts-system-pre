# ---- Stage 1: Build Stage ----
# Use a specific, recent Rust version (based on Debian Bookworm)
FROM rust:1.88 as builder

WORKDIR /usr/src/supplier-parts-system

# Copy project files
COPY . .

# Install a known-compatible version of sqlx-cli
RUN cargo install sqlx-cli --version=0.7.4 --no-default-features --features rustls,mysql

# Build our main application in release mode
RUN cargo build --release

# ---- Stage 2: Final Stage ----
# Use a matching OS version for the final image
FROM debian:bookworm-slim

# Install runtime dependencies (as root)
RUN apt-get update && apt-get install -y libssl3 && rm -rf /var/lib/apt/lists/*

# Copy all necessary files (as root)
COPY --from=builder /usr/src/supplier-parts-system/target/release/supplier-parts-app /usr/local/bin/supplier-parts-app
COPY --from=builder /usr/local/cargo/bin/sqlx /usr/local/bin/sqlx
COPY entrypoint.sh /usr/local/bin/entrypoint.sh

# Set executable permissions on the script (as root)
RUN chmod +x /usr/local/bin/entrypoint.sh

# Create the non-root user (as root)
RUN groupadd -r appuser && useradd -r -g appuser appuser

# =======================================================
# === THE FIX: Switch to the non-root user at the end ===
# =======================================================
USER appuser

# Expose the application's port
EXPOSE 8080

# Set the entrypoint, which will now run as 'appuser'
ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]
