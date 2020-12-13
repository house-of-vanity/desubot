PRAGMA foreign_keys = off;
BEGIN TRANSACTION;

-- Table: alert
CREATE TABLE IF NOT EXISTS alert (
    conf_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    created TEXT NOT NULL,
    time    TEXT NOT NULL,
    message TEXT
);


-- Table: conf
CREATE TABLE IF NOT EXISTS conf (
    id    NUMERIC NOT NULL
                  UNIQUE,
    title TEXT,
    date  INTEGER NOT NULL,
    PRIMARY KEY (
        id
    )
);


-- Table: file
CREATE TABLE IF NOT EXISTS file (
    path    TEXT   NOT NULL,
    user_id TEXT   NOT NULL,
    conf_id TEXT   NOT NULL,
    file_id STRING PRIMARY KEY
);


-- Table: messages
CREATE TABLE IF NOT EXISTS messages (
    id   INTEGER NOT NULL
                 PRIMARY KEY AUTOINCREMENT,
    text TEXT    UNIQUE
);


-- Table: relations
CREATE TABLE IF NOT EXISTS relations (
    id      INTEGER NOT NULL
                    PRIMARY KEY AUTOINCREMENT,
    word_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    conf_id INTEGER NOT NULL,
    date    INTEGER NOT NULL,
    msg_id  INTEGER,
    FOREIGN KEY (
        word_id
    )
    REFERENCES word (id) ON DELETE CASCADE,
    FOREIGN KEY (
        user_id
    )
    REFERENCES user (id),
    FOREIGN KEY (
        conf_id
    )
    REFERENCES conf (id)
);


-- Table: reset
CREATE TABLE IF NOT EXISTS reset (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id     INTEGER,
    conf_id     INTEGER,
    date        INTEGER,
    relation_id INTEGER,
    FOREIGN KEY (
        user_id
    )
    REFERENCES user (id)
);


-- Table: stop_words
CREATE TABLE IF NOT EXISTS stop_words (
    word TEXT
);


-- Table: user
CREATE TABLE IF NOT EXISTS user (
    id         INTEGER NOT NULL
                       UNIQUE,
    username   TEXT,
    first_name INTEGER NOT NULL,
    last_name  INTEGER,
    date       INTEGER NOT NULL,
    PRIMARY KEY (
        id
    )
    ON CONFLICT REPLACE
);


-- Table: word
CREATE TABLE IF NOT EXISTS word (
    id   INTEGER PRIMARY KEY AUTOINCREMENT,
    word TEXT    UNIQUE
);


-- Index: conf_ids
CREATE INDEX IF NOT EXISTS conf_ids ON conf (
    id
);


-- Index: file_ids
CREATE INDEX IF NOT EXISTS file_ids ON file (
    file_id
);


-- Index: relations_ids
CREATE INDEX IF NOT EXISTS relations_ids ON relations (
    conf_id,
    word_id,
    user_id,
    msg_id
);


-- Index: user_ids
CREATE INDEX IF NOT EXISTS user_ids ON user (
    id,
    username
);


-- Index: word_id
CREATE INDEX IF NOT EXISTS word_id ON word (
    id
);


COMMIT TRANSACTION;
PRAGMA foreign_keys = on;