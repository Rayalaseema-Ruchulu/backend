-- Insert breakfast category
-- Insert categories
INSERT INTO categories (id, name) VALUES
	(1, 'Breakfast');

-- Insert popular category and make it featured
INSERT INTO categories (id, name, featured) VALUES (2, 'Popular', 1);

-- Insert ingredients (no duplicates)
INSERT INTO ingredients (id, name) VALUES
	(1, 'Rice'),
	(2, 'Urad dal'),
	(3, 'Water'),
	(4, 'Salt'),
	(5, 'Oil'),
	(6, 'Onion'),
	(7, 'Potato'),
	(8, 'Mustard seeds'),
	(9, 'Turmeric'),
	(10, 'Curry leaves'),
	(11, 'Green chilies'),
	(12, 'Red chili powder'),
	(13, 'Garlic'),
	(14, 'Cumin seeds'),
	(15, 'Ghee');

-- Insert menu items with concise descriptions
INSERT INTO menu_items (id, name, description, price) VALUES
	(1, 'Idli (3)',            'Steamed rice and lentil cakes',          5.99),
	(2, 'Plain Dosa',          'Crispy rice and lentil pancake',         6.99),
	(3, 'Onion Dosa',          'Rice pancake topped with onions',        7.99),
	(4, 'Masala Dosa',         'Crisp pancake stuffed with spiced potato',7.99),
	(5, 'Onion Masala Dosa',   'Spiced potato pancake topped with onion',8.49),
	(6, 'Kadapa Karam Dosa',   'Spicy rice pancake with chili-garlic kick',7.99),
	(7, 'Kadapa Ghee Karam Dosa','Spicy pancake enriched with ghee',      8.99);

-- Associate each menu item with the breakfast category
INSERT INTO item_categories (menu_item_id, category_id) VALUES
	(1, 1),
	(2, 1),
	(3, 1), 
	(4, 1),
	(5, 1),
	(6, 1),
	(7, 1);

-- Link popular items to the popular category
INSERT INTO item_categories (menu_item_id, category_id) VALUES
	(4, 2), -- Masala Dosa
	(5, 2), -- Onion Masala Dosa
	(7, 2); -- Kadapa Ghee Karam Dosa

-- Link menu items to their ingredients
INSERT INTO item_ingredients (menu_item_id, ingredient_id) VALUES
	-- Idli (3)
	(1, 1), (1, 2), (1, 3), (1, 4),

	-- Plain Dosa
	(2, 1), (2, 2), (2, 3), (2, 4), (2, 5),

	-- Onion Dosa
	(3, 1), (3, 2), (3, 3), (3, 4), (3, 5), (3, 6),

	-- Masala Dosa
	(4, 1), (4, 2), (4, 3), (4, 4), (4, 5),
	(4, 6), (4, 7), (4, 8), (4, 9), (4, 10), (4, 11),

	-- Onion Masala Dosa
	(5, 1), (5, 2), (5, 3), (5, 4), (5, 5),
	(5, 6), (5, 7), (5, 8), (5, 9), (5, 10), (5, 11),

	-- Kadapa Karam Dosa
	(6, 1), (6, 2), (6, 3), (6, 4), (6, 5),
	(6, 6), (6, 12), (6, 13), (6, 14), (6, 10),

	-- Kadapa Ghee Karam Dosa
	(7, 1), (7, 2), (7, 3), (7, 4), (7, 5),
	(7, 6), (7, 12), (7, 13), (7, 14), (7, 10), (7, 15);
