# chdb Testing Guide for Brahmand

This guide explains how to test Brahmand with chdb, an embedded ClickHouse database perfect for local development and PoC.

## What is chdb?

**chdb** is an embedded SQL OLAP Engine powered by ClickHouse. It allows you to run ClickHouse queries without installing or running a separate ClickHouse server.

### Key Benefits
- ✅ **No server required** - Embedded in-process execution
- ✅ **Fast setup** - Single pip install
- ✅ **ClickHouse compatible** - Uses same SQL dialect
- ✅ **Perfect for testing** - Ideal for CI/CD and local development
- ✅ **Lightweight** - Minimal resource usage

## Installation

### Prerequisites
- Python 3.7+
- pip

### Install chdb

```bash
pip install chdb
```

## Setup for Brahmand

### Option 1: chdb HTTP Server Mode

If chdb provides an HTTP interface:

```bash
# Start chdb server (check chdb documentation for exact command)
# Example:
chdb-server --http-port 8123
```

Then configure Brahmand:

```bash
cp .env.chdb.example .env
```

Edit `.env`:
```bash
CLICKHOUSE_URL=http://localhost:8123
CLICKHOUSE_USER=default
CLICKHOUSE_PASSWORD=
CLICKHOUSE_DATABASE=default
```

### Option 2: chdb Embedded Mode

For embedded testing, you may need to create a simple Python wrapper:

```python
# chdb_server.py
from chdb import dbapi
import http.server
import socketserver
import json

# Create connection
conn = dbapi.connect()

class ClickHouseHandler(http.server.BaseHTTPRequestHandler):
    def do_POST(self):
        content_length = int(self.headers['Content-Length'])
        query = self.rfile.read(content_length).decode('utf-8')

        try:
            cursor = conn.cursor()
            cursor.execute(query)
            results = cursor.fetchall()

            # Format as JSONEachRow
            response = '\n'.join([json.dumps(dict(zip([d[0] for d in cursor.description], row))) for row in results])

            self.send_response(200)
            self.send_header('Content-type', 'text/plain')
            self.end_headers()
            self.wfile.write(response.encode())
        except Exception as e:
            self.send_response(500)
            self.end_headers()
            self.wfile.write(str(e).encode())

PORT = 8123
with socketserver.TCPServer(("", PORT), ClickHouseHandler) as httpd:
    print(f"chdb HTTP server running on port {PORT}")
    httpd.serve_forever()
```

Run the wrapper:
```bash
python chdb_server.py
```

## Testing Workflow

### 1. Start Brahmand Server

```bash
# Terminal 1: Start Brahmand
cargo run --bin brahmand
```

Expected output:
```
 Server running on - 0.0.0.0:8080
```

### 2. Use Brahmand Client

```bash
# Terminal 2: Start interactive client
cargo run --bin brahmand-client
```

### 3. Run Test Queries

Execute queries from `test_queries_chdb.sql`:

#### Create Schema

```cypher
CREATE NODE TABLE Person (
    id UInt64,
    name String,
    age UInt32,
    city String,
    PRIMARY KEY (id),
    NODE ID (id)
);
```

#### Insert Test Data (via ClickHouse SQL)

Since Brahmand doesn't yet support Cypher CREATE for data, use ClickHouse INSERT:

```sql
-- Execute these directly against chdb or via a SQL client
INSERT INTO Person VALUES (1, 'Alice', 30, 'New York');
INSERT INTO Person VALUES (2, 'Bob', 25, 'San Francisco');
INSERT INTO Person VALUES (3, 'Charlie', 35, 'Seattle');
```

#### Query with Cypher

```cypher
MATCH (p:Person)
RETURN p.name, p.age
ORDER BY p.age DESC;
```

#### Test UNWIND (New Feature!)

```cypher
UNWIND [1, 2, 3, 4, 5] AS num
RETURN num, num * num AS squared;
```

Expected output:
```
num | squared
----|--------
1   | 1
2   | 4
3   | 9
4   | 16
5   | 25
```

#### Test UNWIND with MATCH

```cypher
UNWIND ['Alice', 'Bob', 'Charlie'] AS name
MATCH (p:Person {name: name})
RETURN p.name, p.age;
```

### 4. Run Automated Tests

```bash
# Run all unit tests
cargo test

# Run specific UNWIND tests
cargo test unwind

# Run all tests with output
cargo test -- --nocapture
```

## Test Scenarios

### Scenario 1: Basic Graph Operations

```cypher
-- Create nodes
CREATE NODE TABLE Person (id UInt64, name String, PRIMARY KEY (id), NODE ID (id));
CREATE NODE TABLE Company (id UInt64, name String, PRIMARY KEY (id), NODE ID (id));

-- Create relationships
CREATE REL TABLE WORKS_AT FROM Person TO Company (since UInt32);

-- Query
MATCH (p:Person)-[:WORKS_AT]->(c:Company)
RETURN p.name, c.name;
```

### Scenario 2: UNWIND Operations

```cypher
-- Bulk tag lookup
UNWIND ['python', 'rust', 'javascript'] AS lang
MATCH (t:Tag {name: lang})
RETURN t.name, count(*) AS usage;

-- Generate test data
UNWIND range(1, 10) AS id
RETURN id, id * 2 AS doubled;
```

### Scenario 3: Aggregations

```cypher
-- Count by category
MATCH (p:Person)
RETURN p.city, count(p) AS population
ORDER BY population DESC;

-- Statistical analysis
MATCH (p:Person)
RETURN
    min(p.age) AS youngest,
    max(p.age) AS oldest,
    avg(p.age) AS average_age;
```

## Troubleshooting

### Issue: Connection Refused

**Problem**: Can't connect to chdb

**Solution**:
1. Verify chdb is running: `ps aux | grep chdb`
2. Check port 8123 is available: `lsof -i :8123`
3. Try different port in `.env`

### Issue: Parser Errors

**Problem**: Cypher query fails to parse

**Solution**:
1. Check query syntax against supported features in `PUPPYGRAPH_FEATURE_COMPARISON.md`
2. Verify feature is implemented (DISTINCT, OPTIONAL MATCH not yet available)
3. Review error message for specific clause causing issue

### Issue: Table Not Found

**Problem**: `UNKNOWN_TABLE` error

**Solution**:
1. Ensure CREATE NODE TABLE was executed successfully
2. Check table exists: `SHOW TABLES;` (via direct ClickHouse query)
3. Verify database name in `.env` matches

### Issue: Type Mismatch

**Problem**: Data type errors

**Solution**:
1. chdb/ClickHouse is strict about types
2. Use correct ClickHouse types (UInt64, String, DateTime, etc.)
3. Avoid implicit conversions

## Performance Testing

### Benchmark Queries

Test query performance with chdb:

```bash
# Use built-in timing
time cargo run --bin brahmand-client <<EOF
MATCH (p:Person)-[:WORKS_AT]->(c:Company)
RETURN p.name, c.name
ORDER BY p.name;
EOF
```

### Expected Performance

For small datasets (< 1000 rows):
- Simple MATCH: < 10ms
- Aggregations: < 50ms
- Multi-hop: < 100ms

## Comparison with Full ClickHouse

| Feature | chdb | ClickHouse Server |
|---------|------|-------------------|
| Installation | `pip install` | Docker/Native |
| Startup Time | Instant | 5-30 seconds |
| Resource Usage | Low | Moderate |
| HTTP Protocol | Via wrapper | Native |
| Performance | Good for dev | Production-ready |
| Persistence | File-based | Full database |

## Next Steps

After successful chdb testing:

1. **Load larger datasets** - Test with Stack Overflow data
2. **Benchmark performance** - Compare with Neo4j
3. **Test VeloDB** - Switch to VeloDB for production
4. **Implement missing features** - DISTINCT, OPTIONAL MATCH, variable-length paths
5. **Add more tests** - Expand test coverage

## Resources

- **chdb Documentation**: https://github.com/chdb-io/chdb
- **ClickHouse SQL Reference**: https://clickhouse.com/docs/en/sql-reference/
- **Brahmand Features**: See `PUPPYGRAPH_FEATURE_COMPARISON.md`
- **Example Queries**: See `STACK_OVERFLOW_QUERIES.md`
- **Test Queries**: See `test_queries_chdb.sql`

## Summary

chdb provides an excellent local testing environment for Brahmand:

✅ **Quick Setup** - No complex installation
✅ **ClickHouse Compatible** - Same SQL dialect
✅ **Lightweight** - Perfect for development
✅ **Easy Testing** - Rapid iteration

Perfect for:
- Local development
- CI/CD testing
- Feature validation
- Quick prototypes
- Learning Brahmand

Once chdb testing is complete, you can confidently deploy to production ClickHouse or VeloDB!
