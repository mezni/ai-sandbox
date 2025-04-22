CREATE TABLE IF NOT EXISTS periods (
    period_id SERIAL PRIMARY KEY,
    date DATE NOT NULL,
    year INT NOT NULL,
    quarter INT NOT NULL,
    month INT NOT NULL,
    day INT NOT NULL,
    day_of_week INT NOT NULL,
    day_name TEXT NOT NULL,
    week_of_year INT NOT NULL,
    is_weekend BOOLEAN NOT NULL,
    period TEXT
);

CREATE TABLE roam_directions (
    roam_direction_id SERIAL PRIMARY KEY,
    direction VARCHAR(3) NOT NULL,
    description TEXT
);

CREATE TABLE metrics_type (
    metric_type_id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT
);

CREATE TABLE metric_definition (
    metric_definition_id SERIAL PRIMARY KEY,
    roam_direction_id INTEGER NOT NULL REFERENCES roam_directions(roam_direction_id),
    metric_type_id INTEGER NOT NULL REFERENCES metrics_type(metric_type_id),  
    name VARCHAR(255) NOT NULL,
    description TEXT,
    is_active BOOLEAN,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_by TEXT,
    updated_at TIMESTAMP,
    updated_by TEXT    
);

CREATE TABLE IF NOT EXISTS countries (
    country_id SERIAL PRIMARY KEY,
    iso TEXT,
    name TEXT,
    name_fr TEXT,
    prefix TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_by TEXT,
    updated_at TIMESTAMP,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS networks (
    network_id SERIAL PRIMARY KEY,
    tadig TEXT,
    plmn TEXT,
    mcc TEXT,
    mnc TEXT,
    tech_2g TEXT,
    tech_3g TEXT,
    tech_lte TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_by TEXT,
    updated_at TIMESTAMP,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS operators (
    operator_id SERIAL PRIMARY KEY,
    operator TEXT,
    brand TEXT,
    country_id INT,
    network_id INT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_by TEXT,
    updated_at TIMESTAMP,
    updated_by TEXT
);

CREATE TABLE IF NOT EXISTS prefixes (
    prefixe_id SERIAL PRIMARY KEY,
    country_id INT,
    operator_id INT,
    cc TEXT,
    ndc TEXT,
    prefix TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_by TEXT,
    updated_at TIMESTAMP,
    updated_by TEXT
);

CREATE TABLE subscribers (
    subscriber_id SERIAL PRIMARY KEY,
    imsi VARCHAR(20) NOT NULL,
    msisdn VARCHAR(20) NOT NULL,
    roam_direction_id INTEGER NOT NULL REFERENCES roam_directions(roam_direction_id),
    first_seen TIMESTAMP,
    last_seen TIMESTAMP    
);

CREATE TABLE metrics (
    metric_id SERIAL PRIMARY KEY,
    metric_definition_id INTEGER NOT NULL REFERENCES metric_definition(metric_definition_id),
    batch_id INTEGER,
    period_id INTEGER NOT NULL REFERENCES periods(period_id),
    country_id INTEGER REFERENCES countries(country_id),
    operator_id INTEGER REFERENCES operators(operator_id),  
    subscriber_id INTEGER REFERENCES subscribers(subscriber_id),  
    value INTEGER 
);

CREATE TABLE IF NOT EXISTS stg_roam_out (
    batch_id INT NOT NULL,
    batch_date TEXT NOT NULL,
    imsi TEXT NOT NULL,
    msisdn TEXT NOT NULL,
    vlr_number TEXT NOT NULL,
    prefix TEXT,
    country_id INT,
    operator_id INT
);

CREATE TABLE IF NOT EXISTS stg_roam_in (
    batch_id INT NOT NULL,
    batch_date TEXT NOT NULL,
    hlraddr TEXT,
    nsub INT,
    nsuba INT,
    prefix TEXT,
    country_id INT,
    operator_id INT
);


CREATE TABLE IF NOT EXISTS load_operators (
    id           SERIAL PRIMARY KEY,
    tadig        TEXT,
    plmn         TEXT,
    mcc          TEXT,
    mnc          TEXT,
    t2g          TEXT,
    t3g          TEXT,
    lte          TEXT,
    operator     TEXT,
    brand        TEXT,
    country_iso  TEXT,
    created_by   TEXT DEFAULT 'system'
);

CREATE TABLE IF NOT EXISTS load_prefixes (
    id         SERIAL PRIMARY KEY,
    country    TEXT,
    operator   TEXT,
    cc         TEXT,
    ndc        TEXT,
    prefix     TEXT,
    created_by TEXT DEFAULT 'system'
);

-- =========================
-- LOAD CONFIG
-- =========================

INSERT INTO roam_directions (direction, description) 
VALUES 
    ('IN', 'ROAM IN'),
    ('OUT', 'ROAM OUT');

INSERT INTO metrics_type (name, description) 
VALUES 
    ('GLOBAL', 'GLOBAL'),
    ('COUNTRY', 'COUNTRY'),
    ('OPERATOR', 'OPERATOR'),
    ('SUBSCRIBER', 'SUBSCRIBER');

INSERT INTO metric_definition (roam_direction_id, metric_type_id, name, description)
SELECT 
    rd.roam_direction_id,
    mt.metric_type_id,
    'TOT_DAILY_ROUT',
    'Total daily roamers out'
FROM roam_directions rd
CROSS JOIN metrics_type mt
WHERE rd.direction = 'OUT' AND mt.name = 'GLOBAL';

INSERT INTO metric_definition (roam_direction_id, metric_type_id, name, description)
SELECT 
    rd.roam_direction_id,
    mt.metric_type_id,
    'TOT_DAILY_ROUT_COUNTRY',
    'Total daily roamers out by country'
FROM roam_directions rd
CROSS JOIN metrics_type mt
WHERE rd.direction = 'OUT' AND mt.name = 'COUNTRY';

INSERT INTO metric_definition (roam_direction_id, metric_type_id, name, description)
SELECT 
    rd.roam_direction_id,
    mt.metric_type_id,
    'TOT_DAILY_ROUT_OPERATOR',
    'Total daily roamers out by operator'
FROM roam_directions rd
CROSS JOIN metrics_type mt
WHERE rd.direction = 'OUT' AND mt.name = 'OPERATOR';

INSERT INTO periods (
    date, year, quarter, month, day, day_of_week, day_name,
    week_of_year, is_weekend, period
)
SELECT
    d::date AS date,
    EXTRACT(YEAR FROM d) AS year,
    EXTRACT(QUARTER FROM d) AS quarter,
    EXTRACT(MONTH FROM d) AS month,
    EXTRACT(DAY FROM d) AS day,
    EXTRACT(ISODOW FROM d) AS day_of_week,
    TO_CHAR(d, 'FMDay') AS day_name,
    EXTRACT(WEEK FROM d) AS week_of_year,
    CASE WHEN EXTRACT(ISODOW FROM d) IN (6, 7) THEN TRUE ELSE FALSE END AS is_weekend,
    TO_CHAR(d, 'YYYY-MM-DD') AS date_text
FROM GENERATE_SERIES(
    (DATE_TRUNC('year', NOW()) - INTERVAL '1 year')::DATE,
    (DATE_TRUNC('year', NOW()) + INTERVAL '5 years - 1 day')::DATE,
    '1 day'::INTERVAL
) AS d;    



COPY countries (iso, name, name_fr, prefix)
FROM '/countries.csv'
DELIMITER ',' CSV HEADER;

COPY load_operators (tadig, plmn, mcc, mnc, t2g, t3g, lte, operator, brand, country_iso)
FROM '/operators.csv'
DELIMITER ',' CSV HEADER;

COPY load_prefixes (country, operator, cc, ndc, prefix)
FROM '/prefixes.csv'
DELIMITER ',' CSV HEADER;

update countries set created_by = 'system';


INSERT INTO prefixes (country_id, cc, prefix, created_by)
SELECT 
  country_id, 
  REPLACE(COALESCE(prefix_item, ''), '-', '') AS cc,  
  REPLACE(COALESCE(prefix_item, ''), '-', '') AS prefix,
  'system'
FROM 
  countries,
  unnest(string_to_array(prefix, ',')) AS prefix_item
  WHERE iso not in ('FK', 'GP', 'BL', 'MF', 'AX', 'CX', 'CC', 'KZ', 'RU' , 'BN', 'CW', 'BV', 'SJ', 'EH', 'GG', 'IM', 'JE', 'CA', 'US', 'PR', 'TF', 'YT', 'RE','AQ','HM','NF');


INSERT INTO networks (tadig, plmn, mcc, mnc, tech_2g, tech_3g, tech_lte, created_by)
SELECT tadig, plmn, mcc, mnc, t2g, t3g, lte, created_by
FROM load_operators;


INSERT INTO operators (operator, brand, country_id, network_id, created_by)
SELECT ldr.operator, ldr.brand, c.country_id, n.network_id, ldr.created_by
FROM load_operators ldr
JOIN countries c ON ldr.country_iso = c.iso
JOIN networks n ON ldr.tadig = n.tadig;


INSERT INTO prefixes (country_id,operator_id, cc, ndc, prefix, created_by)
SELECT cnt.country_id, opr.operator_id, ldr.cc, ldr.ndc, ldr.prefix, ldr.created_by
FROM load_prefixes ldr
JOIN operators opr ON opr.operator = ldr.operator
JOIN countries cnt ON cnt.name = ldr.country
WHERE opr.country_id = cnt.country_id;

DELETE FROM prefixes
WHERE prefix IN (
    SELECT prefix
    FROM (
        SELECT prefix,
               ROW_NUMBER() OVER (PARTITION BY prefix ORDER BY prefixe_id) AS rn
        FROM prefixes
    ) t
    WHERE t.rn > 1
);



-- DROP TABLE load_operators;
-- DROP TABLE load_prefixes;




CREATE TABLE IF NOT EXISTS batch_execs (
    id SERIAL PRIMARY KEY,
    batch_name TEXT NOT NULL,
    source_type TEXT,
    source_name TEXT,
    start_time TIMESTAMP,
    end_time TIMESTAMP,        
    corr_id INT,
    batch_status TEXT
);