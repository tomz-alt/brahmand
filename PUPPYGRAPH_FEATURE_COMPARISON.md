# PuppyGraph vs Brahmand Feature Comparison

This document compares Cypher features supported by PuppyGraph (based on openCypher v9) with the current implementation in Brahmand.

## ✅ Currently Supported Features in Brahmand

### Query Clauses
- ✅ **MATCH** - Pattern matching on nodes and relationships
- ✅ **WHERE** - Filtering conditions
- ✅ **RETURN** - Result projection
- ✅ **WITH** - Piping intermediate results
- ✅ **ORDER BY** - Result ordering (ASC/DESC)
- ✅ **SKIP** - Skip first N results
- ✅ **LIMIT** - Limit result count
- ✅ **CREATE** - Create nodes and relationships
- ✅ **SET** - Update properties
- ✅ **REMOVE** - Remove properties
- ✅ **DELETE** / **DETACH DELETE** - Delete nodes/relationships

### DDL Operations
- ✅ **CREATE NODE TABLE** - Define node schemas
- ✅ **CREATE REL TABLE** - Define relationship schemas
- ✅ **PRIMARY KEY** - Node primary key definition
- ✅ **NODE ID** - Node identifier specification

### Pattern Matching
- ✅ **Node patterns** - `(n:Label {prop: value})`
- ✅ **Relationship patterns** - `-[:TYPE]->`, `-[:TYPE]-`, `<-[:TYPE]-`
- ✅ **Property filtering** - Inline and WHERE clause
- ✅ **Label-based filtering** - `(v:person)`
- ✅ **Path assignment** - Binding paths to variables

### Expressions & Operators
- ✅ **Logical operators** - AND, OR, NOT
- ✅ **Comparison operators** - =, <>, <, >, <=, >=
- ✅ **Arithmetic operators** - +, -, *, /, %
- ✅ **String operators** - STARTS WITH, ENDS WITH, CONTAINS
- ✅ **List operations** - IN
- ✅ **Property access** - `node.property`
- ✅ **Function calls** - Built-in functions

### Aggregation Functions
- ✅ **count()** - Count aggregation
- ✅ **sum()** - Sum aggregation
- ✅ **avg()** - Average aggregation
- ✅ **min()** - Minimum value
- ✅ **max()** - Maximum value
- ✅ **collect()** - Collect values into list

### Other Features
- ✅ **Aliasing** - AS keyword for result aliasing
- ✅ **GROUP BY** - Implicit grouping with aggregations
- ✅ **Multiple patterns** - Multiple MATCH patterns in single query
- ✅ **CTEs** - Common Table Expressions via WITH

## ❌ Missing Features (vs PuppyGraph)

### Critical Missing Features

#### 1. **UNWIND** Clause
**Status**: ⚠️ Defined in grammar but not implemented

**Description**: Expands a list into a sequence of rows

**Example**:
```cypher
UNWIND [1, 2, 3] AS x
RETURN x
```

**Priority**: HIGH - This is a standard openCypher feature used in PuppyGraph

---

#### 2. **DISTINCT** Keyword
**Status**: ⚠️ Mentioned in grammar but not implemented

**Description**: Deduplicate results

**Example**:
```cypher
MATCH (p:Person)
RETURN DISTINCT p.name
```

**Priority**: HIGH - Essential for data quality queries

---

#### 3. **Variable-Length Patterns**
**Status**: ❌ Not implemented

**Description**: Match paths of variable length using `*` notation

**Example**:
```cypher
MATCH (a)-[:KNOWS*1..3]->(b)
RETURN a, b
```

**Priority**: HIGH - Critical for graph traversal queries

---

#### 4. **elementId() Function**
**Status**: ❌ Not implemented

**Description**: Returns unique identifier for nodes/relationships

**Example**:
```cypher
MATCH (n) RETURN elementId(n)
```

**Priority**: MEDIUM - Used in PuppyGraph for node/edge identification

---

### Other Potential Gaps

#### 5. **UNION / UNION ALL**
**Status**: ⚠️ UNION appears in code but needs verification

**Example**:
```cypher
MATCH (p:Person) RETURN p.name
UNION
MATCH (c:Company) RETURN c.name
```

**Priority**: MEDIUM

---

#### 6. **OPTIONAL MATCH**
**Status**: ❌ Not found in code

**Description**: LEFT JOIN equivalent for graph queries

**Example**:
```cypher
MATCH (p:Person)
OPTIONAL MATCH (p)-[:WORKS_AT]->(c:Company)
RETURN p, c
```

**Priority**: HIGH - Important for real-world queries

---

#### 7. **CASE Expressions**
**Status**: ❌ Not found

**Description**: Conditional expressions

**Example**:
```cypher
MATCH (p:Person)
RETURN p.name,
       CASE WHEN p.age < 18 THEN 'minor' ELSE 'adult' END AS status
```

**Priority**: MEDIUM

---

#### 8. **EXISTS Subqueries**
**Status**: ❌ Not found

**Description**: Check if a pattern exists

**Example**:
```cypher
MATCH (p:Person)
WHERE EXISTS ((p)-[:KNOWS]->())
RETURN p
```

**Priority**: MEDIUM

---

#### 9. **Map Projections**
**Status**: ❌ Not found

**Description**: Project properties as maps

**Example**:
```cypher
MATCH (p:Person)
RETURN p {.name, .age}
```

**Priority**: LOW

---

#### 10. **List Comprehensions**
**Status**: ❌ Not found

**Description**: Filter/transform lists

**Example**:
```cypher
RETURN [x IN [1,2,3,4,5] WHERE x > 2 | x * 2] AS result
```

**Priority**: LOW

---

## Implementation Recommendations

### Phase 1: Critical Features (Immediate)
1. ✅ Implement **UNWIND** clause
2. ✅ Implement **DISTINCT** keyword
3. ✅ Implement **Variable-Length Patterns** (`*`, `*1..3`)

### Phase 2: Important Features (Near-term)
4. Implement **OPTIONAL MATCH**
5. Implement **elementId()** function
6. Verify and complete **UNION/UNION ALL**

### Phase 3: Nice-to-have (Future)
7. Implement **CASE expressions**
8. Implement **EXISTS subqueries**
9. Implement **Map projections**
10. Implement **List comprehensions**

---

## Testing Strategy

### Stack Overflow Dataset Queries

The README mentions benchmarking with a ~12 million-node Stack Overflow dataset. Key query patterns to test:

1. **User-Post Relationships**
   ```cypher
   MATCH (u:User)-[:POSTED]->(q:Question)
   RETURN u.name, count(q) AS questions
   ORDER BY questions DESC
   LIMIT 10
   ```

2. **Multi-hop Traversals**
   ```cypher
   MATCH (u:User)-[:POSTED]->(q:Question)<-[:ANSWERED]-(a:Answer)<-[:POSTED]-(responder:User)
   RETURN DISTINCT u.name, responder.name, count(a) AS answers
   ```

3. **Tag Analysis**
   ```cypher
   MATCH (q:Question)-[:TAGGED]->(t:Tag)
   WITH t, count(q) AS popularity
   ORDER BY popularity DESC
   LIMIT 20
   RETURN t.name, popularity
   ```

4. **Variable-Length Path Queries** (requires implementation)
   ```cypher
   MATCH path = (u1:User)-[:ANSWERED*1..3]->(u2:User)
   WHERE u1.id = 12345
   RETURN DISTINCT u2.name, length(path)
   ```

---

## References

- **PuppyGraph Cypher Documentation**: https://docs.puppygraph.com/reference/cypher-query-language/
- **PuppyGraph openCypher Guide**: https://docs.puppygraph.com/querying/querying-using-opencypher/
- **openCypher Specification**: https://opencypher.org/
- **Brahmand Architecture**: README.md

---

## Summary

**Total PuppyGraph Features Analyzed**: ~15 core features
**Currently Supported in Brahmand**: ~10 features (67%)
**Critical Missing Features**: 3-6 (UNWIND, DISTINCT, Variable-Length Patterns, OPTIONAL MATCH)
**Recommended Next Steps**: Implement Phase 1 features to achieve parity with PuppyGraph's most common use cases.
