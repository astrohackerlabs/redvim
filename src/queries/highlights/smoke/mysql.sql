-- café: MySQL migration constructs, never executed
CREATE TABLE `messages` (
    id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
    created_at DATETIME(3) NOT NULL,
    body VARCHAR(255) NOT NULL,
    UNIQUE KEY messages_body (body)
) ENGINE=InnoDB DEFAULT CHARACTER SET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
ALTER TABLE messages MODIFY id BIGINT UNSIGNED NULL;
SET @next_id = (SELECT COALESCE(MAX(id), 0) + 1 FROM messages);
SET @ddl = CONCAT('ALTER TABLE messages AUTO_INCREMENT = ', @next_id);
PREPARE migration_stmt FROM @ddl;
EXECUTE migration_stmt;
DEALLOCATE PREPARE migration_stmt;
SELECT 'café', 42 FROM messages;
CREATE TABLE `domains` (
    `name` varchar(255) NOT NULL,
    CONSTRAINT `domains_name_unique` UNIQUE(`name`)
);
