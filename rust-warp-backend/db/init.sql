CREATE TABLE IF NOT EXISTS shopping_list
(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name CHAR(255) NOT NULL,
    created_at datetime DEFAULT (datetime(current_timestamp))
);
