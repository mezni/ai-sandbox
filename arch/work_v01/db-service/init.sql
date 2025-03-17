CREATE TABLE IF NOT EXISTS batch_executions (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    start_date TIMESTAMP,
    end_date TIMESTAMP,
    status VARCHAR(20) NOT NULL
);

CREATE TABLE IF NOT EXISTS stg_roam_out (
    batch_id INTEGER NOT NULL,
    imsi VARCHAR(100),
    msisdn VARCHAR(100),
    vlr_number VARCHAR(100),
    carrier VARCHAR(100),
    region VARCHAR(100),
    country VARCHAR(100)
);

CREATE TABLE IF NOT EXISTS dim_imsi (
    id SERIAL PRIMARY KEY,
    imsi VARCHAR(100)
);

CREATE TABLE IF NOT EXISTS dim_msisdn(
    id SERIAL PRIMARY KEY,
    msisdn VARCHAR(100)
);

CREATE TABLE IF NOT EXISTS dim_carrier(
    id SERIAL PRIMARY KEY,
    carrier VARCHAR(100)
);

CREATE TABLE IF NOT EXISTS dim_country(
    id SERIAL PRIMARY KEY,
    country VARCHAR(100)
);

CREATE TABLE IF NOT EXISTS dim_region(
    id SERIAL PRIMARY KEY,
    country_id VARCHAR(100),
    region VARCHAR(100)
);