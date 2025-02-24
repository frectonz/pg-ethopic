# pg-ethopic

A PostgreSQL extension for formatting ethiopic dates and numbers.

## Usage

Run a PostgreSQL 17 database that has `pg-ethopic` already installed.

```bash
docker run --network=host frectonz/pg-ethopic
```

`pg-ethopic` is also distributed with other PostgreSQL versions.

### PostgreSQL 13

```bash
docker run --network=host frectonz/pg-ethopic:pg13
```

### PostgreSQL 14

```bash
docker run --network=host frectonz/pg-ethopic:pg14
```

### PostgreSQL 15

```bash
docker run --network=host frectonz/pg-ethopic:pg15
```

### PostgreSQL 16

```bash
docker run --network=host frectonz/pg-ethopic:pg16
```

### PostgreSQL 17

```bash
docker run --network=host frectonz/pg-ethopic:pg17
```

Connect to the PostgreSQL database using `psql`.

```bash
psql postgresql://postgres@localhost:5432/
```

Here's some demo usage.

```sql
-- Load PG extension
CREATE EXTENSION pg_ethopic;

-- Format the current date as an ethiopic date.
SELECT ethopic_date(CURRENT_DATE);

-- Format the current date as an ethiopic date with a custom format template.
SELECT ethopic_date(CURRENT_DATE, '{year}/{month}/{day}');

-- Format  a given number as an ethopic number.
SELECT ethopic_number(420);
```
