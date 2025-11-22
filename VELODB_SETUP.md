# VeloDB Setup Guide

This guide explains how to use Brahmand with VeloDB (Apache Doris-based database).

## What is VeloDB?

VeloDB is a cloud-native real-time data warehouse service based on Apache Doris. It uses the MySQL protocol and is highly compatible with MySQL syntax.

## Prerequisites

- VeloDB database instance (cloud or self-hosted Apache Doris)
- Network connectivity to the VeloDB instance

## Configuration

### 1. Create Environment File

Copy the example environment file:

```bash
cp .env.velodb.example .env.velodb
```

### 2. Configure VeloDB Credentials

Edit `.env.velodb` and set your actual credentials:

```bash
VELODB_HOST=your-velodb-host.example.com
VELODB_PORT=9030
VELODB_USER=your_username
VELODB_PASSWORD=your_password
VELODB_DATABASE=your_database

DATABASE_TYPE=velodb
```

### 3. Load the Environment

```bash
cp .env.velodb .env
```

## Testing Connection

Test your VeloDB connection:

```bash
cargo run --bin test_velodb
```

This will:
- Connect to your VeloDB instance
- List all tables in the database
- Show table structures
- Verify the connection is working

## Running Brahmand with VeloDB

Start the Brahmand server:

```bash
cargo run --bin brahmand
```

The server will connect to VeloDB using the credentials from your `.env` file.

## Architecture

VeloDB support is implemented through:

1. **Database Abstraction Layer** (`src/server/database/`)
   - `DatabaseConnector` trait for database-agnostic operations
   - `VeloDBConnector` implementation using MySQL protocol (sqlx)
   - `ClickHouseConnector` for backward compatibility

2. **VeloDB Client** (`src/server/velodb_client.rs`)
   - Connection pool management
   - Helper functions for database operations

3. **SQL Compatibility**
   - VeloDB is highly compatible with MySQL and standard SQL
   - Most ClickHouse queries can be adapted to VeloDB

## Security Notes

⚠️ **Important**: Never commit credentials to version control!

- The `.env` and `.env.velodb` files are in `.gitignore`
- Only `.env.velodb.example` with placeholders should be committed
- Use environment variables or secrets management in production

## Supported Features

- ✅ MySQL protocol connectivity
- ✅ Standard SQL queries
- ✅ Table creation and management
- ✅ Data insertion and retrieval
- ✅ Graph catalog operations

## Switching Between Databases

Set the `DATABASE_TYPE` environment variable:

```bash
# For VeloDB
DATABASE_TYPE=velodb

# For ClickHouse
DATABASE_TYPE=clickhouse
```

## Troubleshooting

### Connection Errors

If you see "failed to lookup address information":
- Verify the hostname is correct
- Check network connectivity to the VeloDB instance
- Ensure firewall rules allow connections on port 9030

### Authentication Errors

- Verify username and password are correct
- Check that the user has access to the specified database
- Ensure the user has appropriate permissions

## References

- [VeloDB MySQL Compatibility](https://docs.velodb.io/cloud/user-guide/query-data/mysql-compatibility)
- [Apache Doris Documentation](https://doris.apache.org/docs/dev/)
- [VeloDB Cloud](https://www.velodb.io/)
