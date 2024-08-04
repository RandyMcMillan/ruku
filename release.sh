#!/bin/bash

# Validate semver
validate_semver() {
    local version=$1
    if [[ $version =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
        return 0
    else
        return 1
    fi
}

# Update version in src/version.rs
update_version_file() {
    local version=$1
    local file="src/version.rs"
    if [[ -f $file ]]; then
        sed -i '' "s/^pub const VERSION: &str = \".*\";/pub const VERSION: &str = \"$version\";/" $file
    else
        echo "File $file does not exist."
        exit 1
    fi
}

# Create a new git tag and push it
create_and_push_git_tag() {
    local version=$1
    local tag="v$version"
    git tag "$tag"
    git push origin "$tag"
}

# Main script
read -r -p "Enter the version number: " version

if validate_semver "$version"; then
    update_version_file "$version"
    create_and_push_git_tag "$version"
    echo "Created a new release $version"
else
    echo "Invalid version number. Please follow semver format (e.g., 1.0.0)."
    exit 1
fi
