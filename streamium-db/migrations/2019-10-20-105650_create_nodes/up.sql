CREATE TYPE NodeTypes AS ENUM (
    'container',
    'stream',
    'file'
);
CREATE TABLE nodes (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    url VARCHAR(4096),
    artist VARCHAR,
    year INTEGER,
    album VARCHAR,
    node_type NodeTypes NOT NULL,
    parent_id INTEGER REFERENCES nodes(id)
);

INSERT INTO nodes(title, node_type) VALUES ('Artists', 'container');
INSERT INTO nodes(title, node_type) VALUES ('Streams', 'container');
