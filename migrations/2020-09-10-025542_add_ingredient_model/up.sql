-- Your SQL goes here
CREATE table ingredients
(
    id uuid DEFAULT uuid_generate_v4 (),
    name VARCHAR NOT NULL,
    density FLOAT NOT NULL,
    grams_per_unit FLOAT,

    PRIMARY KEY (id)
);