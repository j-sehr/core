# Core
Mein Projekt für die Langeweile.
## Modules
- Authentication (MVP)

## Dependencies
- SurrealDB Datbase Initialization Script
```surrealql
DEFINE TABLE accounts SCHEMAFULL;
DEFINE FIELD username      ON TABLE accounts TYPE string;
DEFINE FIELD password ON TABLE accounts TYPE string;
DEFINE FIELD created_at    ON TABLE accounts TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at    ON TABLE accounts TYPE datetime VALUE time::now();

DEFINE INDEX account_username_unique ON TABLE accounts COLUMNS username UNIQUE;


DEFINE TABLE sessions SCHEMAFULL;
DEFINE FIELD account_id   ON TABLE sessions TYPE record<accounts>;
DEFINE FIELD refresh_hash ON TABLE sessions TYPE string;
DEFINE FIELD created_at   ON TABLE sessions TYPE datetime DEFAULT time::now();
DEFINE FIELD expires_at   ON TABLE sessions TYPE datetime;
DEFINE FIELD is_active    ON TABLE sessions TYPE bool DEFAULT false;

DEFINE INDEX session_refresh_unique ON TABLE sessions COLUMNS refresh_hash UNIQUE;
DEFINE INDEX session_account_idx    ON TABLE sessions COLUMNS account_id;
```
