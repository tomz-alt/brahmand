# Stack Overflow Dataset - Example Graph Queries

This document provides example Cypher queries for the Stack Overflow dataset mentioned in the README (12 million nodes). These queries demonstrate real-world graph analysis patterns for testing and benchmarking Brahmand.

## Dataset Schema

The Stack Overflow graph model consists of:

### Nodes
- **User**: Stack Overflow users
  - Properties: `id`, `name`, `reputation`, `created_at`
- **Question**: Questions posted on Stack Overflow
  - Properties: `id`, `title`, `body`, `score`, `view_count`, `created_at`
- **Answer**: Answers to questions
  - Properties: `id`, `body`, `score`, `is_accepted`, `created_at`
- **Tag**: Technology/topic tags
  - Properties: `id`, `name`
- **Comment**: Comments on questions/answers
  - Properties: `id`, `text`, `created_at`

### Relationships
- `(:User)-[:POSTED]->(:Question)` - User posts a question
- `(:User)-[:POSTED]->(:Answer)` - User posts an answer
- `(:Answer)-[:ANSWERS]->(:Question)` - Answer belongs to a question
- `(:Question)-[:TAGGED]->(:Tag)` - Question has tags
- `(:User)-[:COMMENTED]->(:Question|Answer)` - User comments on content

---

## Basic Queries

### 1. Find Top Users by Question Count

```cypher
MATCH (u:User)-[:POSTED]->(q:Question)
RETURN u.name, u.id, count(q) AS questions
ORDER BY questions DESC
LIMIT 10;
```

**Purpose**: Identify most active question askers

**Expected Result**: Top 10 users with their question counts

---

### 2. Most Popular Tags

```cypher
MATCH (q:Question)-[:TAGGED]->(t:Tag)
RETURN t.name, count(q) AS question_count
ORDER BY question_count DESC
LIMIT 20;
```

**Purpose**: Find most frequently used tags

**Expected Result**: Top 20 tags by usage

---

### 3. Questions with Most Views

```cypher
MATCH (q:Question)
RETURN q.id, q.title, q.view_count, q.score
ORDER BY q.view_count DESC
LIMIT 10;
```

**Purpose**: Identify most viewed questions

**Expected Result**: Top 10 questions by view count

---

## Intermediate Queries

### 4. User Expertise Analysis

```cypher
MATCH (u:User)-[:POSTED]->(q:Question)-[:TAGGED]->(t:Tag)
WITH u, t, count(q) AS tag_questions
WHERE tag_questions >= 5
RETURN u.name, t.name AS expertise, tag_questions
ORDER BY u.name, tag_questions DESC;
```

**Purpose**: Find users' areas of expertise based on their questions

**Expected Result**: Users with their frequent tags (5+ questions per tag)

---

### 5. Answered vs Unanswered Questions

```cypher
MATCH (q:Question)
OPTIONAL MATCH (q)<-[:ANSWERS]-(a:Answer)
WITH q, count(a) AS answer_count
RETURN
    CASE WHEN answer_count = 0 THEN 'Unanswered' ELSE 'Answered' END AS status,
    count(q) AS questions
ORDER BY status;
```

**Purpose**: Calculate ratio of answered to unanswered questions

**Expected Result**: Count of answered vs unanswered questions

---

### 6. Questions with Accepted Answers

```cypher
MATCH (q:Question)<-[:ANSWERS]-(a:Answer)
WHERE a.is_accepted = true
RETURN q.title, a.score AS accepted_answer_score, q.score AS question_score
ORDER BY question_score DESC
LIMIT 20;
```

**Purpose**: Find high-quality Q&A pairs (accepted answers)

**Expected Result**: Top 20 questions with accepted answers

---

## Multi-Hop Traversal Queries

### 7. User to User via Question-Answer

```cypher
MATCH (asker:User)-[:POSTED]->(q:Question)<-[:ANSWERS]-(a:Answer)<-[:POSTED]-(answerer:User)
WHERE asker.id <> answerer.id
RETURN asker.name, answerer.name, count(a) AS answers_given
ORDER BY answers_given DESC
LIMIT 10;
```

**Purpose**: Find users who frequently answer each other's questions

**Expected Result**: Top 10 asker-answerer pairs

---

### 8. Tag Co-occurrence Analysis

```cypher
MATCH (q:Question)-[:TAGGED]->(t1:Tag)
MATCH (q)-[:TAGGED]->(t2:Tag)
WHERE t1.id < t2.id
RETURN t1.name, t2.name, count(q) AS co_occurrences
ORDER BY co_occurrences DESC
LIMIT 20;
```

**Purpose**: Find tags that frequently appear together

**Expected Result**: Top 20 tag combinations

---

### 9. Question-Answer Thread Depth

```cypher
MATCH (u:User)-[:POSTED]->(q:Question)
OPTIONAL MATCH (q)<-[:ANSWERS]-(a:Answer)
OPTIONAL MATCH (a)<-[:COMMENTED]-(c:Comment)
RETURN q.title, count(DISTINCT a) AS answers, count(DISTINCT c) AS comments
ORDER BY (count(DISTINCT a) + count(DISTINCT c)) DESC
LIMIT 10;
```

**Purpose**: Find most engaging threads (answers + comments)

**Expected Result**: Top 10 threads by engagement

---

## Aggregation Queries

### 10. Daily Question Volume

```cypher
MATCH (q:Question)
RETURN substring(q.created_at, 0, 10) AS date, count(q) AS questions
ORDER BY date DESC
LIMIT 30;
```

**Purpose**: Track question trends over time

**Expected Result**: Last 30 days of question volumes

---

### 11. User Reputation Distribution

```cypher
MATCH (u:User)
WITH u.reputation AS rep
RETURN
    CASE
        WHEN rep < 100 THEN '0-99'
        WHEN rep < 1000 THEN '100-999'
        WHEN rep < 10000 THEN '1000-9999'
        ELSE '10000+'
    END AS reputation_bracket,
    count(*) AS users
ORDER BY reputation_bracket;
```

**Purpose**: Understand user reputation distribution

**Expected Result**: User counts per reputation bracket

---

### 12. Average Answer Score by Tag

```cypher
MATCH (q:Question)-[:TAGGED]->(t:Tag)
MATCH (q)<-[:ANSWERS]-(a:Answer)
RETURN t.name, avg(a.score) AS avg_answer_score, count(a) AS total_answers
ORDER BY avg_answer_score DESC
LIMIT 20;
```

**Purpose**: Find tags with highest quality answers

**Expected Result**: Top 20 tags by average answer score

---

## UNWIND Examples

### 13. Bulk Tag Analysis

```cypher
UNWIND ['python', 'javascript', 'java', 'rust', 'go'] AS tag_name
MATCH (t:Tag {name: tag_name})<-[:TAGGED]-(q:Question)
RETURN tag_name, count(q) AS questions, avg(q.score) AS avg_score
ORDER BY questions DESC;
```

**Purpose**: Compare multiple tags at once using UNWIND

**Expected Result**: Statistics for specified tags

---

### 14. Multi-User Query with UNWIND

```cypher
UNWIND [123, 456, 789, 101112] AS user_id
MATCH (u:User {id: user_id})
OPTIONAL MATCH (u)-[:POSTED]->(q:Question)
RETURN u.name, count(q) AS questions
ORDER BY questions DESC;
```

**Purpose**: Query multiple users efficiently

**Expected Result**: Question counts for specified users

---

## Performance Testing Queries

### 15. Large Scale Join Query

```cypher
MATCH (u:User)-[:POSTED]->(q:Question)-[:TAGGED]->(t:Tag)
MATCH (q)<-[:ANSWERS]-(a:Answer)
WHERE t.name = 'python' AND q.score > 10
RETURN u.name, q.title, count(a) AS answers, q.score
ORDER BY q.score DESC
LIMIT 100;
```

**Purpose**: Test complex join performance

**Expected Result**: Top 100 high-scored Python questions with answer counts

---

### 16. Graph Traversal Benchmark

```cypher
MATCH path = (u1:User)-[:POSTED]->(:Question)<-[:ANSWERS]-(:Answer)<-[:POSTED]-(u2:User)
WHERE u1.id = 12345 AND u1.id <> u2.id
RETURN DISTINCT u2.name, u2.reputation
ORDER BY u2.reputation DESC
LIMIT 50;
```

**Purpose**: Benchmark multi-hop traversal

**Expected Result**: Top 50 answerers for a specific user's questions

---

## DDL Examples

### 17. Create Node Tables

```cypher
CREATE NODE TABLE User (
    id UInt64,
    name String,
    reputation UInt32,
    created_at DateTime,
    PRIMARY KEY (id),
    NODE ID (id)
);

CREATE NODE TABLE Question (
    id UInt64,
    title String,
    body String,
    score Int32,
    view_count UInt64,
    created_at DateTime,
    PRIMARY KEY (id),
    NODE ID (id)
);

CREATE NODE TABLE Tag (
    id UInt64,
    name String,
    PRIMARY KEY (id),
    NODE ID (id)
);
```

---

### 18. Create Relationship Tables

```cypher
CREATE REL TABLE POSTED_QUESTION
FROM User TO Question (
    created_at DateTime
);

CREATE REL TABLE ANSWERS
FROM Answer TO Question (
    created_at DateTime
);

CREATE REL TABLE TAGGED
FROM Question TO Tag ();
```

---

## Advanced Analysis

### 19. User Influence Score

```cypher
MATCH (u:User)-[:POSTED]->(q:Question)
OPTIONAL MATCH (q)<-[:ANSWERS]-(a:Answer)
WITH u,
     count(DISTINCT q) AS questions,
     count(DISTINCT a) AS answers_received,
     sum(q.score) AS total_question_score,
     avg(q.view_count) AS avg_views
RETURN u.name,
       questions,
       answers_received,
       total_question_score,
       avg_views,
       (total_question_score + answers_received + avg_views/100) AS influence_score
ORDER BY influence_score DESC
LIMIT 20;
```

**Purpose**: Calculate user influence based on multiple metrics

**Expected Result**: Top 20 influential users

---

### 20. Tag Network Clustering

```cypher
MATCH (t:Tag)<-[:TAGGED]-(q:Question)-[:TAGGED]->(other:Tag)
WHERE t.id < other.id
WITH t, other, count(q) AS shared_questions
WHERE shared_questions > 100
RETURN t.name, collect(other.name) AS related_tags, count(other) AS cluster_size
ORDER BY cluster_size DESC
LIMIT 10;
```

**Purpose**: Identify tag clusters (related technologies)

**Expected Result**: Top 10 tags with their related tag networks

---

## Testing Notes

### Performance Expectations

Based on the README benchmarks (MacBook Pro M3 Pro, 18 GB RAM):
- **Simple queries (1-5)**: < 100ms
- **Intermediate queries (6-12)**: 100-500ms
- **Multi-hop traversals (7-9, 15-16)**: 500ms-2s
- **Complex aggregations (19-20)**: 1-5s

### Dataset Size
- **12 million nodes** across User, Question, Answer, Tag, Comment
- **30+ million relationships**
- **Ideal for**: Testing at scale, benchmark comparisons with Neo4j

### Benchmark Comparison
The README states: "multihop traversals running approximately 10× faster than Neo4j v2025.03"

Use queries 7, 8, 16, and 19 for comparative benchmarks.

---

## Future Query Patterns (Requires Additional Features)

### Variable-Length Paths (Not Yet Implemented)

```cypher
-- Find users connected through answer chains
MATCH path = (u1:User)-[:POSTED]->(:Question)<-[:ANSWERS*1..3]-(:Answer)<-[:POSTED]-(u2:User)
WHERE u1.id = 12345
RETURN DISTINCT u2.name, length(path)
ORDER BY length(path);
```

### OPTIONAL MATCH (Not Yet Implemented)

```cypher
-- Find questions with optional accepted answers
MATCH (q:Question)
OPTIONAL MATCH (q)<-[:ANSWERS]-(a:Answer WHERE a.is_accepted = true)
RETURN q.title, a.score AS accepted_score
LIMIT 20;
```

---

## Conclusion

These queries provide a comprehensive test suite for:
1. ✅ Basic pattern matching
2. ✅ Aggregations and grouping
3. ✅ Multi-hop traversals
4. ✅ UNWIND clause functionality
5. ✅ ORDER BY, LIMIT, WHERE clauses
6. ⏳ Variable-length patterns (to be implemented)
7. ⏳ OPTIONAL MATCH (to be implemented)
8. ⏳ DISTINCT keyword (to be implemented)

Run these queries against the Stack Overflow dataset to verify Brahmand's graph query capabilities and performance characteristics.
