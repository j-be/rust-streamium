ALTER TABLE nodes ADD parent_id INTEGER REFERENCES nodes(id);
DELETE FROM nodes WHERE ID = -24;
DROP TABLE node_parents;
