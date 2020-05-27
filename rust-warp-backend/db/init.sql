CREATE TABLE IF NOT EXISTS shopping_list
(
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name CHAR(255) NOT NULL,
    created_at datetime DEFAULT (datetime(current_timestamp)) NOT NULL
);

CREATE TABLE IF NOT EXISTS shopping_list_item
(
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    shopping_list_id INTEGER NOT NULL,
    name CHAR(255) NOT NULL,
    checked BOOLEAN DEFAULT false NOT NULL,
    quantity INTEGER,
    FOREIGN KEY (shopping_list_id) REFERENCES shooping_list(id)
);

