CREATE TYPE NodeTypes AS ENUM (
    'container',
    'stream',
    'file'
);
CREATE TABLE nodes (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    url VARCHAR(4096) NOT NULL,
    artist VARCHAR,
    year INTEGER,
    node_type NodeTypes NOT NULL
);
