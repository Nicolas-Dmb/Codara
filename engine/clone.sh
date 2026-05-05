#!/bin/bash

if [ -z "$1" ] || [ -z "$2" ] || [ -z "$3" ]; then
    echo "Usage: ./clone.sh <repo_url> <branch> <path_to_clone_into>"
    exit 1
fi

REPO_URL="$1"
BRANCH="$2"
CODEBASE_DIR="$3"

echo "Cloning from '$REPO_URL' (branch: $BRANCH) into '$CODEBASE_DIR'..."

if ! rm -rf "$CODEBASE_DIR"; then
    echo "Failed to clean existing codebase directory"
    exit 1
fi

if ! git clone -b "$BRANCH" "$REPO_URL" "$CODEBASE_DIR"; then
    echo "Failed to clone repository"
    exit 1
fi

echo "Project cloned successfully"
exit 0