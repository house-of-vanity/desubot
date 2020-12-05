CREATE TABLE `word` (
        `id`    INTEGER PRIMARY KEY AUTOINCREMENT,
        `word`  TEXT UNIQUE
);
CREATE TABLE sqlite_sequence(name,seq);
CREATE TABLE `user` (
        `id`    INTEGER NOT NULL UNIQUE,
        `username`      TEXT NOT NULL,
        `first_name`    INTEGER NOT NULL,
        `last_name`     INTEGER NOT NULL,
        `date`  INTEGER NOT NULL,
        PRIMARY KEY(`id`)
);
CREATE TABLE `conf` (
        `id`    NUMERIC NOT NULL UNIQUE,
        `title` TEXT,
        `date`  INTEGER NOT NULL,
        PRIMARY KEY(`id`)
);
CREATE TABLE `file` (
         `path` TEXT NOT NULL UNIQUE,
         `user_id`  TEXT NOT NULL,
         `conf_id`  TEXT NOT NULL,
         PRIMARY KEY(`path`)
);
CREATE TABLE IF NOT EXISTS "relations" (
        `id`    INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
        `word_id`       INTEGER NOT NULL,
        `user_id`       INTEGER NOT NULL,
        `conf_id`       INTEGER NOT NULL,
        `date`  INTEGER NOT NULL, `msg_id` INTEGER NULL,
        FOREIGN KEY(`word_id`) REFERENCES `word`(`id`) ON DELETE CASCADE,
        FOREIGN KEY(`user_id`) REFERENCES `user`(`id`),
        FOREIGN KEY(`conf_id`) REFERENCES `conf`(`id`)
);
CREATE TABLE `reset` (
        `id`    INTEGER PRIMARY KEY AUTOINCREMENT,
        `user_id`       INTEGER,
        `conf_id`       INTEGER,
        `date`  INTEGER,
        `relation_id`   INTEGER,
        FOREIGN KEY(`user_id`) REFERENCES `user`(`id`)
);
CREATE TABLE `alert` (
`conf_id`TEXT NOT NULL,
`user_id`TEXT NOT NULL,
`created`TEXT NOT NULL,
`time`TEXT NOT NULL,
`message`TEXT
);
CREATE TABLE `xxx_message` (`id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT, `text`TEXT UNIQUE NULL);