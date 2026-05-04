#!/bin/bash


if [ -z "$1" ]; then
    echo "Usage: ./clone.sh <path> <branch> <path_to_clone_into>"
    exit 1
fi

if [ -z "$2"]; then
    echo "Usage: ./clone.sh <path> <branch> <path_to_clone_into>"
    exit 1
fi

if [ -z "$3"]; then
    echo "Usage: ./clone.sh <path> <branch> <path_to_clone_into>"
    exit 1
fi

PATH="$1"
BRANCH="$2"
CODEBASE_DIR="$3"

echo "Cloning project with ID '$PROJECT_ID'..."

try
    rm -rf "$CODEBASE_DIR"
catch
    echo "Failed to clean existing codebase directory: $error"
    exit 1

try
    git clone -b "$BRANCH" "$PATH" "$CODEBASE_DIR"
catch
    echo "Failed to clone repository: $error"
    exit 1

echo "Project cloned successfully"
exit 0