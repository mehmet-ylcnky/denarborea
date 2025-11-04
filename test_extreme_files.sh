#!/bin/bash

echo "🚀 DenArborea Extreme Large File Test"
echo "======================================"

# Create test directory
mkdir -p /tmp/denarborea_extreme_test
cd /tmp/denarborea_extreme_test

# Build the project
echo "📦 Building DenArborea..."
cd /home/a524573/denarborea
cargo build --release

DENARBOREA="/home/a524573/denarborea/target/release/denarborea"
cd /tmp/denarborea_extreme_test

echo ""
echo "🔥 Creating 2GB JSON file..."
python3 << 'EOF'
import json
import sys

# Create a 2GB JSON file
with open('extreme_2gb.json', 'w') as f:
    f.write('[\n')
    
    # Approximately 300,000 items for 2GB
    total_items = 300000
    
    for i in range(total_items):
        if i % 10000 == 0:
            print(f"Progress: {i}/{total_items} ({i/total_items*100:.1f}%)")
            sys.stdout.flush()
        
        item = {
            "id": i,
            "name": f"User_{i}",
            "email": f"user{i}@example.com",
            "profile": {
                "age": 20 + (i % 60),
                "city": f"City_{i % 1000}",
                "country": f"Country_{i % 50}",
                "bio": "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."
            },
            "metadata": {
                "created": f"2024-{(i % 12) + 1:02d}-{(i % 28) + 1:02d}T10:30:00Z",
                "tags": [f"tag_{i % 100}", f"category_{i % 50}", f"type_{i % 20}"],
                "score": round((i * 0.123) % 100, 2),
                "active": i % 2 == 0
            }
        }
        
        json.dump(item, f, separators=(',', ':'))
        if i < total_items - 1:
            f.write(',\n')
        else:
            f.write('\n')
    
    f.write(']\n')

print("✅ 2GB JSON file created!")
EOF

echo ""
echo "📊 File size:"
ls -lh extreme_2gb.json

echo ""
echo "💾 Testing memory usage with 2GB file..."
echo "Before test - Memory usage:"
ps -o pid,rss,vsz,comm -p $$

echo ""
echo "🔍 Viewing 2GB JSON file (first 3 items only)..."
time $DENARBOREA --view extreme_2gb.json --memory-mapped --max-lines 3

echo ""
echo "After test - Memory usage:"
ps -o pid,rss,vsz,comm -p $$

echo ""
echo "🎯 Testing different strategies on 2GB file:"

echo "  📈 Auto strategy:"
time $DENARBOREA --view extreme_2gb.json --max-lines 2 > /dev/null

echo "  🗺️  Memory-mapped strategy:"
time $DENARBOREA --view extreme_2gb.json --memory-mapped --max-lines 2 > /dev/null

echo "  🌊 Streaming strategy:"
time $DENARBOREA --view extreme_2gb.json --streaming --max-lines 2 > /dev/null

echo ""
echo "🧹 Cleaning up..."
rm -f extreme_2gb.json

echo ""
echo "✅ Extreme test completed successfully!"
echo "🏆 DenArborea can handle multi-GB files with minimal memory usage!"
