# Databases

Theurgy does not require databases by default.

## Default

- Durable truth starts as plain files.
- Build outputs, indexes, and caches are derived from those files.
- Static pages and API snapshots should be reproducible from source plus config.

## When A Database Is Justified

Use a database only when the app needs at least one of these:

- concurrent writes that need real transactions
- high-cardinality queries that would make file scans too slow
- relational integrity across many changing entities
- audit/event streams with strict append semantics
- global replication or conflict handling that plain files cannot model clearly
- user/session data that must be updated frequently under load

## Preferred Shape

- Start with embedded, free software databases such as SQLite when a database is needed.
- Keep database schema migrations in source.
- Keep export/import paths so data can return to plain files.
- Treat databases as the transaction/index layer, not as an excuse to hide application truth.
- Do not introduce a network database as the first answer to ordinary website scale.
