# ---- Builder Stage ----
# We use the official Rust image and specify the target platform.
FROM rust:1-alpine as builder

# Install the MUSL target, which is necessary for creating a static binary
# that can run on Alpine Linux.
RUN rustup target add x86_64-unknown-linux-musl

# Install build dependencies required for a static build
RUN apk add --no-cache musl-dev

# Create a new empty project to cache dependencies.
# This layer will only be rebuilt if Cargo.toml or Cargo.lock changes.
WORKDIR /usr/src/vanish
RUN cargo init --bin

# Copy over the dependency manifests
COPY Cargo.toml Cargo.lock ./

# Build the dependencies to cache them
RUN cargo build --release --target x86_64-unknown-linux-musl

# Now, copy the actual source code
COPY src ./src
COPY static ./static

# Build the application. This will be faster because dependencies are cached.
# We touch the src files to ensure cargo recompiles the main binary but not the deps.
RUN touch src/main.rs && cargo build --release --target x86_64-unknown-linux-musl

# ---- Final Stage ----
# Use a minimal Alpine image for the final container.
FROM alpine:latest

# It's a good practice to run as a non-root user.
RUN addgroup -S appgroup && adduser -S appuser -G appgroup
USER appuser

WORKDIR /home/appuser

# Copy the compiled, static binary from the builder stage
COPY --from=builder /usr/src/vanish/target/x86_64-unknown-linux-musl/release/vanish .

# Copy the static files for the UI
COPY --from=builder /usr/src/vanish/static ./static

# Expose the port the app runs on
EXPOSE 5820

# The command to run the application
CMD ["./vanish"]
