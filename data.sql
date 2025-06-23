INSERT INTO categories (name) VALUES ('Breakfast'), ('Indo Chinese Specials');
INSERT INTO ingredients (name) VALUES ('Rice'), ('Onion');

INSERT INTO menu_items (name, description, price) VALUES ('Idli', 'Test1', 5.99), ('Plain Dosa', 'Test2', 6.99), ('Veg Fried Rice', 'Test3', 10.99), ('Veg Noodles', 'Test4', 10.99);

INSERT INTO item_categories (menu_item_id, category_id) VALUES (1, 1), (2, 1), (3, 2), (4, 2);
INSERT INTO item_ingredients (menu_item_id, ingredient_id) VALUES (1, 1), (2, 1), (3, 2), (4, 2);
