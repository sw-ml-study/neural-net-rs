#!/bin/bash
# Batch test script for neural network examples
# Tests all examples with recommended settings and known-good seeds
# Reports training results and accuracy

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
OUTPUT_DIR="${OUTPUT_DIR:-/tmp/neural-net-batch-test}"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Example configurations: name|epochs|learning_rate|seed
EXAMPLES=(
    "and|5000|0.5|42"
    "or|5000|0.5|42"
    "xor|10000|0.5|42"
    "parity3|20000|0.5|123"
    "quadrant|15000|0.3|42"
    "adder2|20000|0.5|42"
    "iris|15000|0.3|42"
    "pattern3x3|15000|0.5|42"
)

print_header() {
    echo ""
    echo -e "${BLUE}========================================${NC}"
    echo -e "${BLUE}  Neural Network Batch Test${NC}"
    echo -e "${BLUE}========================================${NC}"
    echo ""
}

print_example_header() {
    local name=$1
    local epochs=$2
    local lr=$3
    local seed=$4
    echo ""
    echo -e "${YELLOW}------------------------------------------${NC}"
    echo -e "${YELLOW}  Example: ${name}${NC}"
    echo -e "${YELLOW}  Epochs: ${epochs} | LR: ${lr} | Seed: ${seed}${NC}"
    echo -e "${YELLOW}------------------------------------------${NC}"
}

# Parse output from CLI eval command
# Input: "Output: [0.123, 0.456, 0.789]"
# Returns: predicted class (index of max value) or thresholded binary
parse_output() {
    local output="$1"
    local num_outputs="$2"

    # Extract the array content between [ and ]
    local values
    values=$(echo "$output" | sed -n 's/.*\[\(.*\)\].*/\1/p' | tr -d ' ')

    if [ -z "$values" ]; then
        echo "0"
        return
    fi

    # Split by comma and find max
    local max_val="-999"
    local max_idx=0
    local idx=0

    IFS=',' read -ra vals <<< "$values"
    for val in "${vals[@]}"; do
        # Remove whitespace
        val=$(echo "$val" | tr -d ' ')
        if [ -n "$val" ]; then
            # Use awk for float comparison
            if awk "BEGIN {exit !($val > $max_val)}"; then
                max_val=$val
                max_idx=$idx
            fi
            ((idx++)) || true
        fi
    done

    # For single-output examples, threshold at 0.5
    if [ "$idx" -eq 1 ]; then
        if awk "BEGIN {exit !($max_val > 0.5)}"; then
            echo "1"
        else
            echo "0"
        fi
    else
        echo "$max_idx"
    fi
}

# Create output directory
mkdir -p "$OUTPUT_DIR"

print_header

echo "Output directory: $OUTPUT_DIR"
echo "Building project..."
cd "$PROJECT_ROOT"
cargo build --release --bin neural-net-cli 2>/dev/null

PASSED=0
FAILED=0
RESULTS=()

for config in "${EXAMPLES[@]}"; do
    IFS='|' read -r name epochs lr seed <<< "$config"

    print_example_header "$name" "$epochs" "$lr" "$seed"

    output_file="$OUTPUT_DIR/${name}.json"

    # Run training
    if cargo run --release --bin neural-net-cli -- train \
        --example "$name" \
        --epochs "$epochs" \
        --learning-rate "$lr" \
        --seed "$seed" \
        --output "$output_file" 2>&1; then

        # Extract final loss from the checkpoint
        if [ -f "$output_file" ]; then
            echo ""
            echo -e "${GREEN}SUCCESS${NC}: Training completed"
            echo "  Model saved to: $output_file"

            # Test all inputs for the example
            echo ""
            echo "Testing predictions..."

            case "$name" in
                "and")
                    tests=("0.0,0.0|0" "0.0,1.0|0" "1.0,0.0|0" "1.0,1.0|1")
                    num_outputs=1
                    ;;
                "or")
                    tests=("0.0,0.0|0" "0.0,1.0|1" "1.0,0.0|1" "1.0,1.0|1")
                    num_outputs=1
                    ;;
                "xor")
                    tests=("0.0,0.0|0" "0.0,1.0|1" "1.0,0.0|1" "1.0,1.0|0")
                    num_outputs=1
                    ;;
                "parity3")
                    tests=("0.0,0.0,0.0|0" "0.0,0.0,1.0|1" "0.0,1.0,0.0|1" "0.0,1.0,1.0|0"
                           "1.0,0.0,0.0|1" "1.0,0.0,1.0|0" "1.0,1.0,0.0|0" "1.0,1.0,1.0|1")
                    num_outputs=1
                    ;;
                "quadrant")
                    # Test one point from each quadrant
                    tests=("1.0,1.0|0" "-1.0,1.0|1" "-1.0,-1.0|2" "1.0,-1.0|3")
                    num_outputs=4
                    ;;
                "adder2")
                    # Test a few additions: 0+0=0, 1+1=2, 2+2=4, 3+3=6
                    # Note: outputs are binary [S2, S1, S0], so class is interpreted differently
                    tests=("0.0,0.0,0.0,0.0|0" "0.0,1.0,0.0,1.0|1" "1.0,0.0,1.0,0.0|0" "1.0,1.0,1.0,1.0|0")
                    num_outputs=3
                    ;;
                "iris")
                    # Test one from each species
                    tests=("5.1,3.5,1.4,0.2|0" "7.0,3.2,4.7,1.4|1" "6.3,3.3,6.0,2.5|2")
                    num_outputs=3
                    ;;
                "pattern3x3")
                    # Test each pattern type
                    tests=("1.0,0.0,1.0,0.0,1.0,0.0,1.0,0.0,1.0|0"
                           "1.0,1.0,1.0,1.0,0.0,1.0,1.0,1.0,1.0|1"
                           "0.0,1.0,0.0,1.0,1.0,1.0,0.0,1.0,0.0|2"
                           "0.0,0.0,0.0,1.0,1.0,1.0,0.0,0.0,0.0|3")
                    num_outputs=4
                    ;;
                *)
                    tests=()
                    num_outputs=1
                    ;;
            esac

            correct=0
            total=${#tests[@]}

            for test in "${tests[@]}"; do
                IFS='|' read -r input expected_class <<< "$test"

                # Get prediction
                output=$(cargo run --release --bin neural-net-cli -- eval \
                    --model "$output_file" \
                    --input "$input" 2>/dev/null || echo "error")

                if [[ "$output" != "error" ]] && [[ "$output" == *"Output:"* ]]; then
                    # Parse the output line
                    output_line=$(echo "$output" | grep "Output:")
                    predicted=$(parse_output "$output_line" "$num_outputs")

                    if [ "$predicted" -eq "$expected_class" ] 2>/dev/null; then
                        echo -e "  ${GREEN}✓${NC} Input: [$input] -> Class $predicted (expected $expected_class)"
                        ((correct++)) || true
                    else
                        echo -e "  ${RED}✗${NC} Input: [$input] -> Class $predicted (expected $expected_class)"
                    fi
                else
                    echo -e "  ${RED}✗${NC} Input: [$input] -> ERROR"
                fi
            done

            accuracy=$((correct * 100 / total))

            if [ "$accuracy" -eq 100 ]; then
                echo -e "\n  ${GREEN}Accuracy: ${accuracy}% (${correct}/${total})${NC}"
                RESULTS+=("${GREEN}✓ ${name}: ${accuracy}%${NC}")
                ((PASSED++)) || true
            elif [ "$accuracy" -ge 80 ]; then
                echo -e "\n  ${YELLOW}Accuracy: ${accuracy}% (${correct}/${total})${NC}"
                RESULTS+=("${YELLOW}~ ${name}: ${accuracy}%${NC}")
                ((PASSED++)) || true
            else
                echo -e "\n  ${RED}Accuracy: ${accuracy}% (${correct}/${total})${NC}"
                RESULTS+=("${RED}✗ ${name}: ${accuracy}%${NC}")
                ((FAILED++)) || true
            fi
        fi
    else
        echo -e "${RED}FAILED${NC}: Training error"
        RESULTS+=("${RED}✗ ${name}: FAILED${NC}")
        ((FAILED++)) || true
    fi
done

# Summary
echo ""
echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}  Summary${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""

for result in "${RESULTS[@]}"; do
    echo -e "  $result"
done

echo ""
echo -e "Passed: ${GREEN}${PASSED}${NC} / $((PASSED + FAILED))"
if [ "$FAILED" -gt 0 ]; then
    echo -e "Failed: ${RED}${FAILED}${NC}"
fi
echo ""
echo "Model files saved to: $OUTPUT_DIR"
echo ""

exit $FAILED
