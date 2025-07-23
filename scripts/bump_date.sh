#!/usr/bin/env bash
# Auto-version bump script for MMH-RS documentation
# Run on every push to main

DATE=$(date +"%B %-d, %Y")

# Update LaTeX files
find "Project White Papers" -name "*.tex" -exec sed -i "s/Last Updated:.*$/Last Updated: $DATE/" {} \;

# Update markdown files
find . -name "*.md" -exec sed -i "s/Last Updated:.*$/Last Updated: $DATE/" {} \;

# Commit and push changes
git add .
git commit -am "docs: sync date → $DATE"
git push 