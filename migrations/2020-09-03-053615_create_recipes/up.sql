-- Your SQL goes here

CREATE EXTENSION "uuid-ossp";

CREATE TABLE recipes (
    id uuid DEFAULT uuid_generate_v4 (),
    name VARCHAR NOT NULL,
    instructions TEXT NOT NULL,

    PRIMARY KEY (id)
);
