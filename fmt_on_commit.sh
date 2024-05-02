#!/bin/bash

for rust_file in $(git diff --name-only --cached | grep ".*\.rs$"); do
    cargo fmt -- $rust_file
    git add $rust_file
done
