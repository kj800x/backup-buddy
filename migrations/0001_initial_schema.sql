CREATE TABLE backup_policies (
  id TEXT PRIMARY KEY, -- UUID as string
  path TEXT NOT NULL,
  max_staleness INTEGER NOT NULL, -- in milliseconds
  kind TEXT NOT NULL CHECK(kind IN ('backup', 'exclude', 'null')),
  recursive BOOLEAN NOT NULL DEFAULT 1
);

CREATE TABLE backup_attempts (
  id TEXT PRIMARY KEY, -- UUID as string
  policy_id TEXT NOT NULL,
  started_at TEXT NOT NULL, -- ISO8601 UTC timestamp
  completed_at TEXT NOT NULL,
  status TEXT NOT NULL CHECK(status IN ('succeeded', 'failed'))
);
