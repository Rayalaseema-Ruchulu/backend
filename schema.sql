CREATE TABLE IF NOT EXISTS categories (
    id INT PRIMARY KEY,
    name TEXT NOT NULL
) STRICT;

CREATE TABLE IF NOT EXISTS ingredients (
    id INT PRIMARY KEY,
    name TEXT NOT NULL
) STRICT;

CREATE TABLE IF NOT EXISTS menu_items (
    id INT PRIMARY KEY,
    item_name TEXT NOT NULL,
    description TEXT NOT NULL,
    price REAL NOT NULL
) STRICT;

CREATE TABLE IF NOT EXISTS item_categories (
    menu_item_id INT NOT NULL,
    category_id INT NOT NULL,
    PRIMARY KEY (menu_item_id, category_id),
    FOREIGN KEY (menu_item_id) REFERENCES menu_items(id),
    FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE CASCADE
) STRICT;

-- We will be seraching by category ID as well, so we will make an index just for it so that
-- queries will be fast
CREATE INDEX IF NOT EXISTS idx_item_categories_category_id ON item_categories (category_id);

CREATE TABLE IF NOT EXISTS item_ingredients (
    menu_item_id INT NOT NULL,
    ingredient_id INT NOT NULL,
    PRIMARY KEY (menu_item_id, ingredient_id),
    FOREIGN KEY (menu_item_id) REFERENCES menu_items(id) ON DELETE CASCADE,
    FOREIGN KEY (ingredient_id) REFERENCES ingredients(id) ON DELETE CASCADE
) STRICT;

PRAGMA optimize;
