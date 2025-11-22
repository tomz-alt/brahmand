-- Test Queries for chdb PoC
-- These demonstrate Brahmand's Cypher query capabilities
-- Execute through Brahmand server after it connects to chdb

-- ============================================
-- Part 1: Schema Setup (DDL)
-- ============================================

-- Create Person node table
CREATE NODE TABLE Person (
    id UInt64,
    name String,
    age UInt32,
    city String,
    PRIMARY KEY (id),
    NODE ID (id)
);

-- Create Company node table
CREATE NODE TABLE Company (
    id UInt64,
    name String,
    industry String,
    founded UInt32,
    PRIMARY KEY (id),
    NODE ID (id)
);

-- Create WORKS_AT relationship table
CREATE REL TABLE WORKS_AT
FROM Person TO Company (
    since UInt32,
    position String
);

-- Create KNOWS relationship table
CREATE REL TABLE KNOWS
FROM Person TO Person (
    since UInt32
);

-- ============================================
-- Part 2: Data Insertion (using ClickHouse INSERT)
-- Note: These would be executed directly against chdb
-- ============================================

-- Insert People
-- INSERT INTO Person VALUES (1, 'Alice', 30, 'New York');
-- INSERT INTO Person VALUES (2, 'Bob', 25, 'San Francisco');
-- INSERT INTO Person VALUES (3, 'Charlie', 35, 'Seattle');
-- INSERT INTO Person VALUES (4, 'Diana', 28, 'Austin');
-- INSERT INTO Person VALUES (5, 'Eve', 32, 'Boston');

-- Insert Companies
-- INSERT INTO Company VALUES (101, 'TechCorp', 'Software', 2010);
-- INSERT INTO Company VALUES (102, 'DataSystems', 'Analytics', 2015);
-- INSERT INTO Company VALUES (103, 'CloudNet', 'Cloud Services', 2018);

-- Insert Relationships - WORKS_AT
-- INSERT INTO WORKS_AT VALUES (1, 101, 2018, 'Engineer');
-- INSERT INTO WORKS_AT VALUES (2, 102, 2020, 'Analyst');
-- INSERT INTO WORKS_AT VALUES (3, 101, 2015, 'Manager');
-- INSERT INTO WORKS_AT VALUES (4, 103, 2019, 'DevOps');
-- INSERT INTO WORKS_AT VALUES (5, 102, 2021, 'Scientist');

-- Insert Relationships - KNOWS
-- INSERT INTO KNOWS VALUES (1, 2, 2019);
-- INSERT INTO KNOWS VALUES (1, 3, 2018);
-- INSERT INTO KNOWS VALUES (2, 4, 2020);
-- INSERT INTO KNOWS VALUES (3, 5, 2016);

-- ============================================
-- Part 3: Cypher Query Tests
-- ============================================

-- Test 1: Simple MATCH
MATCH (p:Person)
RETURN p.name, p.age, p.city;

-- Test 2: MATCH with WHERE
MATCH (p:Person)
WHERE p.age > 28
RETURN p.name, p.age
ORDER BY p.age DESC;

-- Test 3: MATCH with relationship
MATCH (p:Person)-[:WORKS_AT]->(c:Company)
RETURN p.name, c.name AS company
ORDER BY p.name;

-- Test 4: UNWIND clause (NEW FEATURE!)
UNWIND [1, 2, 3, 4, 5] AS num
RETURN num, num * num AS squared;

-- Test 5: UNWIND with array of strings
UNWIND ['Alice', 'Bob', 'Charlie'] AS person_name
MATCH (p:Person {name: person_name})
RETURN p.name, p.age;

-- Test 6: Multi-hop traversal
MATCH (p1:Person)-[:KNOWS]->(p2:Person)-[:WORKS_AT]->(c:Company)
RETURN p1.name AS person, p2.name AS friend, c.name AS friend_company;

-- Test 7: Aggregation
MATCH (p:Person)-[:WORKS_AT]->(c:Company)
RETURN c.name, count(p) AS employee_count
ORDER BY employee_count DESC;

-- Test 8: WITH clause
MATCH (p:Person)
WITH p.city AS city, count(p) AS person_count
WHERE person_count > 0
RETURN city, person_count
ORDER BY person_count DESC;

-- Test 9: COUNT aggregation
MATCH (p:Person)
RETURN count(p) AS total_people;

-- Test 10: String matching
MATCH (p:Person)
WHERE p.name STARTS WITH 'A'
RETURN p.name, p.age;

-- Test 11: SKIP and LIMIT
MATCH (p:Person)
RETURN p.name, p.age
ORDER BY p.age
SKIP 1
LIMIT 3;

-- Test 12: Complex UNWIND with MATCH
UNWIND [101, 102, 103] AS company_id
MATCH (c:Company)
WHERE c.id = company_id
OPTIONAL MATCH (p:Person)-[:WORKS_AT]->(c)
RETURN c.name, count(p) AS employees;

-- Test 13: Industry analysis
MATCH (c:Company)
RETURN c.industry, count(c) AS company_count, min(c.founded) AS oldest
ORDER BY company_count DESC;

-- Test 14: Network analysis
MATCH (p:Person)-[:KNOWS]->(friend:Person)
RETURN p.name, collect(friend.name) AS friends;

-- Test 15: Date-based filtering
MATCH (p:Person)-[r:WORKS_AT]->(c:Company)
WHERE r.since >= 2019
RETURN p.name, c.name, r.since AS start_year
ORDER BY r.since DESC;

-- ============================================
-- Part 4: Performance Tests
-- ============================================

-- Test 16: Large UNWIND
UNWIND range(1, 100) AS i
RETURN i, i * 2 AS doubled
LIMIT 10;

-- Test 17: Complex join
MATCH (p:Person)-[:WORKS_AT]->(c:Company)
MATCH (p)-[:KNOWS]->(friend:Person)
RETURN p.name, c.name, count(friend) AS friend_count
ORDER BY friend_count DESC;

-- ============================================
-- Expected Results Summary
-- ============================================

-- Test 1: Should return 5 people
-- Test 2: Should return 3 people (age > 28): Alice (30), Charlie (35), Eve (32)
-- Test 4: Should return 5 rows with numbers 1-5 and their squares
-- Test 5: Should return matching people with their ages
-- Test 7: Should show companies with employee counts
-- Test 9: Should return 5 (total people)

-- ============================================
-- Notes for Testing
-- ============================================

-- 1. These queries test the following Brahmand features:
--    ✓ MATCH clause
--    ✓ WHERE clause
--    ✓ RETURN clause
--    ✓ ORDER BY
--    ✓ LIMIT / SKIP
--    ✓ WITH clause
--    ✓ UNWIND clause (newly implemented)
--    ✓ Aggregations (count, min, collect)
--    ✓ String operators (STARTS WITH)
--    ✓ Relationship traversal

-- 2. Features NOT yet implemented:
--    ✗ OPTIONAL MATCH (Test 12 will fail)
--    ✗ DISTINCT
--    ✗ Variable-length patterns (*1..3)
--    ✗ range() function (Test 16)
--    ✗ CASE expressions

-- 3. For chdb PoC:
--    - Execute DDL statements first
--    - Insert sample data
--    - Run queries through brahmand-client
--    - Compare results with expectations
