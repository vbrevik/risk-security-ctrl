#!/bin/bash
# API Test Script using curl
# Usage: ./test-api.sh [base_url]
# Example: ./test-api.sh http://localhost:5580

BASE_URL="${1:-http://localhost:5580}"
PASSED=0
FAILED=0

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

test_endpoint() {
    local name="$1"
    local method="$2"
    local endpoint="$3"
    local expected_status="$4"
    local check_pattern="$5"

    echo -n "Testing: $name ... "

    response=$(curl -s -w "\n%{http_code}" -X "$method" "$BASE_URL$endpoint")
    http_code=$(echo "$response" | tail -n 1)
    body=$(echo "$response" | head -n -1)

    if [ "$http_code" != "$expected_status" ]; then
        echo -e "${RED}FAILED${NC}"
        echo "  Expected status: $expected_status"
        echo "  Got status: $http_code"
        ((FAILED++))
        return 1
    fi

    if [ -n "$check_pattern" ]; then
        if echo "$body" | grep -q "$check_pattern"; then
            echo -e "${GREEN}PASSED${NC}"
            ((PASSED++))
        else
            echo -e "${RED}FAILED${NC}"
            echo "  Pattern '$check_pattern' not found in response"
            echo "  Response: $body" | head -c 200
            ((FAILED++))
            return 1
        fi
    else
        echo -e "${GREEN}PASSED${NC}"
        ((PASSED++))
    fi
}

echo -e "${YELLOW}=== Ontology API Test Suite ===${NC}"
echo "Testing against: $BASE_URL"
echo ""

# Health check
test_endpoint "Health check" "GET" "/api/health" "200" '"status":"ok"'

# Frameworks
test_endpoint "List frameworks" "GET" "/api/ontology/frameworks" "200" '"id":"iso31000"'
test_endpoint "Get framework by ID" "GET" "/api/ontology/frameworks/iso31000" "200" '"ISO 31000:2018"'
test_endpoint "Get nonexistent framework" "GET" "/api/ontology/frameworks/nonexistent" "404" ""

# Concepts
test_endpoint "List concepts" "GET" "/api/ontology/concepts?limit=10" "200" '"data":'
test_endpoint "List concepts with pagination" "GET" "/api/ontology/concepts?page=2&limit=5" "200" '"page":2'
test_endpoint "List concepts filtered by framework" "GET" "/api/ontology/concepts?framework_id=iso31000" "200" '"framework_id":"iso31000"'
test_endpoint "Get concept by ID" "GET" "/api/ontology/concepts/iso31000-principles" "200" '"name_en":"Principles"'
test_endpoint "Get nonexistent concept" "GET" "/api/ontology/concepts/nonexistent" "404" ""
test_endpoint "Get concept relationships" "GET" "/api/ontology/concepts/iso31000-principles/relationships" "200" '"related_concepts":'

# Search
test_endpoint "Search concepts" "GET" "/api/ontology/concepts/search?q=risk&limit=10" "200" '"data":'
test_endpoint "Search with framework filter" "GET" "/api/ontology/concepts/search?q=management&framework_id=iso31000" "200" '"data":'

# Relationships
test_endpoint "List relationships" "GET" "/api/ontology/relationships" "200" '"relationship_type"'

# Summary
echo ""
echo -e "${YELLOW}=== Test Summary ===${NC}"
echo -e "${GREEN}Passed: $PASSED${NC}"
echo -e "${RED}Failed: $FAILED${NC}"

if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}All tests passed!${NC}"
    exit 0
else
    echo -e "${RED}Some tests failed!${NC}"
    exit 1
fi
