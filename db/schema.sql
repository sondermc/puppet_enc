PRAGMA foreign_keys = ON;
CREATE TABLE IF NOT EXISTS environment (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(50) UNIQUE NOT NULL,
    description TEXT,
    created_on DATETIME DEFAULT (datetime('now', 'utc')),
    updated_on DATETIME DEFAULT (datetime('now', 'utc'))
);

CREATE TABLE IF NOT EXISTS role (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(50),
    description TEXT,
    created_on DATETIME DEFAULT (datetime('now', 'utc')),
    updated_on DATETIME DEFAULT (datetime('now', 'utc'))
);

CREATE TABLE IF NOT EXISTS node (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    certname VARCHAR(50),
    environment_id INTEGER,
    role_id INTEGER,
    created_on DATETIME DEFAULT (datetime('now', 'utc')),
    updated_on DATETIME DEFAULT (datetime('now', 'utc')),
    FOREIGN KEY (environment_id) REFERENCES environment (id),
    FOREIGN KEY (role_id) REFERENCES role (id)
);
