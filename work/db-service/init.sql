CREATE TABLE IF NOT EXISTS countries (
    id SERIAL PRIMARY KEY,
    country_code VARCHAR(10) NOT NULL,
    name VARCHAR(100) NOT NULL
);

CREATE TABLE IF NOT EXISTS carriers (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    national_destination_code VARCHAR(10),
    country_id INT REFERENCES countries(id) ON DELETE SET NULL
);

INSERT INTO countries (country_code, name) VALUES ('61', 'Australia');

INSERT INTO carriers (name, national_destination_code, country_id)
VALUES (
    'SingTelOptus',
    '432',
    (SELECT id FROM countries WHERE name = 'Australia')
);

