ALTER TABLE nodes ADD parent_id INTEGER REFERENCES nodes(id);
DROP TABLE node_parents;
