--
-- File generated with SQLiteStudio v3.2.1 on Sun Dec 6 13:46:57 2020
--
-- Text encoding used: System
--
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


-- Table: xxx_message
CREATE TABLE IF NOT EXISTS xxx_message (
                             id   INTEGER NOT NULL
                                 PRIMARY KEY AUTOINCREMENT,
                             text TEXT    UNIQUE
);


COMMIT TRANSACTION;
PRAGMA foreign_keys = on;
