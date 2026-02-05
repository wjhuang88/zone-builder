#!/bin/bash

echo "Running Zone Builder (developed with Vibe Coding approach) on sample test blog directory..."
echo

cargo run -- -p ./samples/test-blog

echo
echo "Sample test blog processing completed!"
echo
echo "Generated JSON files:"
echo "- ./samples/test-blog/index.json"
echo "- ./samples/test-blog/latest.json" 
echo "- ./samples/test-blog/recommended.json"
echo "- ./samples/test-blog/notebooks.json"
echo "- ./samples/test-blog/tech/meta.json"
echo "- ./samples/test-blog/tutorials/meta.json"
echo "- ./samples/test-blog/essays/meta.json"
echo
echo "Project developed using the Vibe Coding approach for rapid and intuitive implementation."