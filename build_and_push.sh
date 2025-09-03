#!/bin/sh

# build_and_push.sh - A script to build and push the Vanish Docker image.

# --- Configuration ---
# The script will attempt to automatically determine the Docker Hub username.
# You can override it by setting it manually here:
# DOCKER_HUB_USERNAME="your_username"

# --- Stop on Error ---
set -e

# --- Functions ---
# Helper function to print messages
info() {
    echo "INFO: $1"
}

# --- Main Script ---
info "Starting build and push process..."

# 1. Determine Docker Hub Username
if [ -z "$DOCKER_HUB_USERNAME" ]; then
    DOCKER_HUB_USERNAME=$(grep 'image:' docker-compose.yml | sed 's/.*image: \([^/]*\)\/.*/\1/')
    if [ -z "$DOCKER_HUB_USERNAME" ]; then
        echo "ERROR: Could not determine Docker Hub username from docker-compose.yml."
        echo "Please set it manually in the script."
        exit 1
    fi
fi
info "Docker Hub Username detected: $DOCKER_HUB_USERNAME"

# 2. Determine Image Version
VERSION=$1
if [ -z "$VERSION" ]; then
    VERSION="latest"
    info "No version provided. Defaulting to 'latest'."
else
    info "Version provided: $VERSION"
fi

IMAGE_NAME="$DOCKER_HUB_USERNAME/vanish"
TAG_VERSION="$IMAGE_NAME:$VERSION"
TAG_LATEST="$IMAGE_NAME:latest"

# 3. Build the Docker image
info "Building Docker image with tag: $TAG_VERSION"
docker build -t "$TAG_VERSION" .

# 4. Tag as 'latest' if a specific version is provided
if [ "$VERSION" != "latest" ]; then
    info "Also tagging as: $TAG_LATEST"
    docker tag "$TAG_VERSION" "$TAG_LATEST"
fi

# 5. Push the image (version tag)
info "Pushing tag: $TAG_VERSION"
docker push "$TAG_VERSION"

# 6. Push the 'latest' tag if a specific version is provided
if [ "$VERSION" != "latest" ]; then
    info "Pushing tag: $TAG_LATEST"
    docker push "$TAG_LATEST"
fi

info "----------------------------------------"
info "✅ Process completed successfully!"
info "Pushed images:"
info "   - $TAG_VERSION"
if [ "$VERSION" != "latest" ]; then
    info "   - $TAG_LATEST"
fi
info "----------------------------------------"
