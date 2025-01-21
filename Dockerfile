# Build frontend
FROM node:latest as frontend-builder
WORKDIR /usr/src/nadamark
COPY frontend/ ./frontend/
WORKDIR /usr/src/nadamark/frontend
RUN npm install
RUN npm run build

# Build backend
FROM rust:latest as backend-builder
WORKDIR /usr/src/nadamark
COPY backend/ ./backend/
WORKDIR /usr/src/nadamark/backend

RUN apt-get update && apt-get install -y libwayland-dev && \
    rm -rf /var/lib/apt/lists/*

RUN cargo clean
RUN cargo build --release

# Final image
FROM debian:bookworm-slim
WORKDIR /usr/local/bin/nadamark

# Install SQLite dependencies
RUN apt-get update && apt-get install -y libsqlite3-0 && \
    rm -rf /var/lib/apt/lists/*

# Copy the built frontend files
COPY --from=frontend-builder /usr/src/nadamark/frontend/build ./static
# Copy the backend binary
COPY --from=backend-builder /usr/src/nadamark/backend/target/release/nadamark ./

# Add gosu for running as non-root
RUN apt-get update && apt-get install -y gosu && \
    rm -rf /var/lib/apt/lists/*

# Create a default non-root user (will be modified at runtime if needed)
RUN useradd -r -s /bin/false nadamark && \
    mkdir -p /bookmarks

# Create entrypoint script
COPY <<'EOF' /entrypoint.sh
#!/bin/bash
USER_ID=${USER_ID:-1000}
GROUP_ID=${GROUP_ID:-1000}

# Recreate nadamark group and user with new IDs
groupmod -g $GROUP_ID nadamark
usermod -u $USER_ID nadamark

# Set correct ownership
chown -R nadamark:nadamark /usr/local/bin/nadamark /bookmarks

# Run command as nadamark user
exec gosu nadamark "$@"
EOF

RUN chmod +x /entrypoint.sh

# Set the static files path as an environment variable
ENV STATIC_FILES_PATH="./static"

EXPOSE 8663
ENTRYPOINT ["/entrypoint.sh"]
CMD ["./nadamark"]
