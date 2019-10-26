ALTER TABLE nodes DROP parent_id;

CREATE TABLE node_parents (
    id SERIAL PRIMARY KEY,
    node_id INTEGER REFERENCES nodes(id) ON DELETE CASCADE NOT NULL,
    parent_id INTEGER REFERENCES nodes(id) NOT NULL,
    node_order INTEGER
);

INSERT INTO nodes(id, title, node_type) VALUES (-24, 'Favorites', 'container');
