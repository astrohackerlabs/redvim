/* café: SQLite migration constructs, never executed */
CREATE TABLE IF NOT EXISTS messages (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    body TEXT NOT NULL,
    active BOOLEAN DEFAULT TRUE
);
CREATE INDEX IF NOT EXISTS messages_body ON messages(body);
INSERT INTO messages(body) VALUES ('it''s café');
select m.body from messages AS m JOIN messages AS n ON m.id = n.id WHERE m.id = 42;
UPDATE messages SET body = 'hello' WHERE id = 1;
DELETE FROM messages WHERE id = 2;
