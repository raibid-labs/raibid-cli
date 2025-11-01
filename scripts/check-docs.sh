#!/usr/bin/env bash
set -euo pipefail

# Documentation validation script for raibid-ci
# Checks markdown files for common issues and broken links

DOCS_DIR="${DOCS_DIR:-./docs}"
ERRORS=0

echo "=================================================="
echo "raibid-ci Documentation Validation"
echo "=================================================="
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

error() {
    echo -e "${RED}ERROR:${NC} $1"
    ERRORS=$((ERRORS + 1))
}

warning() {
    echo -e "${YELLOW}WARNING:${NC} $1"
}

success() {
    echo -e "${GREEN}SUCCESS:${NC} $1"
}

info() {
    echo "INFO: $1"
}

# Check 1: Find all markdown files
echo "1. Finding markdown files..."
MARKDOWN_FILES=$(find "$DOCS_DIR" -type f -name "*.md")
FILE_COUNT=$(echo "$MARKDOWN_FILES" | wc -l)
info "Found $FILE_COUNT markdown files"
echo ""

# Check 2: Validate markdown syntax
echo "2. Checking markdown syntax..."
SYNTAX_ERRORS=0
for file in $MARKDOWN_FILES; do
    # Check for common markdown issues

    # Check for missing blank lines around code blocks
    if grep -Pzo '(?<!\n)\n```' "$file" > /dev/null 2>&1 || \
       grep -Pzo '```\n(?!\n)' "$file" > /dev/null 2>&1; then
        warning "Missing blank lines around code blocks in: $file"
        SYNTAX_ERRORS=$((SYNTAX_ERRORS + 1))
    fi

    # Check for tabs (should use spaces)
    if grep -P '\t' "$file" > /dev/null 2>&1; then
        warning "Tabs found in: $file (use spaces)"
        SYNTAX_ERRORS=$((SYNTAX_ERRORS + 1))
    fi

    # Check for trailing whitespace
    if grep -n ' $' "$file" > /dev/null 2>&1; then
        warning "Trailing whitespace in: $file"
    fi
done

if [ $SYNTAX_ERRORS -eq 0 ]; then
    success "No syntax errors found"
else
    warning "Found $SYNTAX_ERRORS syntax warnings"
fi
echo ""

# Check 3: Validate internal links
echo "3. Checking internal links..."
BROKEN_LINKS=0
for file in $MARKDOWN_FILES; do
    # Extract markdown links: [text](path)
    LINKS=$(grep -oP '\[([^\]]+)\]\(([^)]+)\)' "$file" | grep -oP '\(([^)]+)\)' | tr -d '()' || true)

    for link in $LINKS; do
        # Skip external links (http/https)
        if [[ "$link" =~ ^https?:// ]]; then
            continue
        fi

        # Skip anchors and fragments for now
        if [[ "$link" =~ ^# ]]; then
            continue
        fi

        # Resolve relative path
        file_dir=$(dirname "$file")
        link_path="${link%%#*}" # Remove fragment

        # Check if file exists
        if [[ "$link_path" =~ ^/ ]]; then
            # Absolute path from repo root
            target=".$link_path"
        else
            # Relative path
            target="$file_dir/$link_path"
        fi

        if [ ! -f "$target" ] && [ ! -d "$target" ]; then
            error "Broken link in $file: $link"
            BROKEN_LINKS=$((BROKEN_LINKS + 1))
        fi
    done
done

if [ $BROKEN_LINKS -eq 0 ]; then
    success "No broken internal links found"
else
    error "Found $BROKEN_LINKS broken internal links"
fi
echo ""

# Check 4: Validate code blocks have language specified
echo "4. Checking code block languages..."
CODE_BLOCK_ISSUES=0
for file in $MARKDOWN_FILES; do
    # Find code blocks without language: ```\n (but not ```language\n)
    if grep -Pn '^```$' "$file" > /dev/null 2>&1; then
        warning "Code blocks without language in: $file"
        CODE_BLOCK_ISSUES=$((CODE_BLOCK_ISSUES + 1))
    fi
done

if [ $CODE_BLOCK_ISSUES -eq 0 ]; then
    success "All code blocks have languages specified"
else
    warning "Found $CODE_BLOCK_ISSUES files with unlabeled code blocks"
fi
echo ""

# Check 5: Check for required sections in component READMEs
echo "5. Checking component README structure..."
COMPONENT_READMES=$(find "$DOCS_DIR/components" -name "README.md" 2>/dev/null || true)
STRUCTURE_ISSUES=0

for readme in $COMPONENT_READMES; do
    required_sections=("## Overview" "## Architecture" "## Status" "## Features")

    for section in "${required_sections[@]}"; do
        if ! grep -q "^$section" "$readme"; then
            warning "Missing section '$section' in: $readme"
            STRUCTURE_ISSUES=$((STRUCTURE_ISSUES + 1))
        fi
    done
done

if [ $STRUCTURE_ISSUES -eq 0 ] && [ -n "$COMPONENT_READMES" ]; then
    success "All component READMEs have required sections"
elif [ -z "$COMPONENT_READMES" ]; then
    info "No component READMEs found to check"
else
    warning "Found $STRUCTURE_ISSUES missing sections in component READMEs"
fi
echo ""

# Check 6: Validate Mermaid diagrams
echo "6. Checking Mermaid diagrams..."
MERMAID_ISSUES=0
for file in $MARKDOWN_FILES; do
    # Find mermaid code blocks
    if grep -q '```mermaid' "$file"; then
        # Basic validation: check for graph declaration
        mermaid_blocks=$(awk '/```mermaid/,/```/' "$file")
        if ! echo "$mermaid_blocks" | grep -qE '(graph|sequenceDiagram|classDiagram|stateDiagram)'; then
            warning "Possibly invalid Mermaid diagram in: $file"
            MERMAID_ISSUES=$((MERMAID_ISSUES + 1))
        fi
    fi
done

if [ $MERMAID_ISSUES -eq 0 ]; then
    success "Mermaid diagrams appear valid"
else
    warning "Found $MERMAID_ISSUES potential Mermaid diagram issues"
fi
echo ""

# Check 7: Check for orphaned files
echo "7. Checking for orphaned documentation files..."
ORPHANED=0

# Get all markdown files except README.md in root
LINKABLE_FILES=$(find "$DOCS_DIR" -type f -name "*.md" ! -name "README.md")

for file in $LINKABLE_FILES; do
    # Skip if it's a template
    if [[ "$file" =~ /templates/ ]]; then
        continue
    fi

    # Check if this file is linked from anywhere
    file_basename=$(basename "$file")
    if ! grep -r "\]($file_basename)" "$DOCS_DIR" > /dev/null 2>&1 && \
       ! grep -r "\](.*/$file_basename)" "$DOCS_DIR" > /dev/null 2>&1; then
        # Check if it's a README (usually linked by directory)
        if [[ "$file_basename" != "README.md" ]]; then
            warning "Potentially orphaned file: $file"
            ORPHANED=$((ORPHANED + 1))
        fi
    fi
done

if [ $ORPHANED -eq 0 ]; then
    success "No orphaned files found"
else
    warning "Found $ORPHANED potentially orphaned files"
fi
echo ""

# Check 8: Validate frontmatter/metadata
echo "8. Checking file metadata..."
METADATA_ISSUES=0
for file in $MARKDOWN_FILES; do
    # Check if file has "Last Updated" footer
    if ! grep -q "Last Updated:" "$file"; then
        # Only warn for non-template files
        if [[ ! "$file" =~ /templates/ ]]; then
            warning "Missing 'Last Updated' metadata in: $file"
            METADATA_ISSUES=$((METADATA_ISSUES + 1))
        fi
    fi
done

if [ $METADATA_ISSUES -eq 0 ]; then
    success "All files have required metadata"
else
    warning "Found $METADATA_ISSUES files missing metadata"
fi
echo ""

# Summary
echo "=================================================="
echo "Validation Summary"
echo "=================================================="
echo "Total markdown files: $FILE_COUNT"
echo "Syntax warnings: $SYNTAX_ERRORS"
echo "Broken links: $BROKEN_LINKS"
echo "Code block issues: $CODE_BLOCK_ISSUES"
echo "Structure issues: $STRUCTURE_ISSUES"
echo "Mermaid issues: $MERMAID_ISSUES"
echo "Orphaned files: $ORPHANED"
echo "Metadata issues: $METADATA_ISSUES"
echo ""

if [ $ERRORS -eq 0 ]; then
    echo -e "${GREEN}✓ Documentation validation passed!${NC}"
    exit 0
else
    echo -e "${RED}✗ Documentation validation failed with $ERRORS errors${NC}"
    echo "Please fix the errors above before committing."
    exit 1
fi
