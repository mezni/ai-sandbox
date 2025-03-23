CREATE TABLE IF NOT EXISTS dim_countries (
    id SERIAL PRIMARY KEY,
    country VARCHAR(100) NOT NULL,
    alpha2 VARCHAR(2) NOT NULL,
    alpha3 VARCHAR(3) NOT NULL,
    country_num INT NOT NULL,   
    country_code VARCHAR(10) NOT NULL
);

CREATE TABLE IF NOT EXISTS dim_carriers (
    id SERIAL PRIMARY KEY,
    carrier_id VARCHAR(10) NOT NULL,
    carrier_name VARCHAR(100) NOT NULL,
    country_name VARCHAR(100) NOT NULL,
    country_code VARCHAR(10) NOT NULL,
    national_destination_code VARCHAR(10)
--    country_id INT NOT NULL,    
--    FOREIGN KEY (country_id) REFERENCES dim_countries(id)
);

CREATE TABLE IF NOT EXISTS batch_execs (
    id SERIAL PRIMARY KEY,
    batch_name VARCHAR(100) NOT NULL,
    start_time TIMESTAMP,
    end_time TIMESTAMP,        
    batch_status VARCHAR(10) 
);


CREATE TABLE IF NOT EXISTS stg_roam_out (
    batch_id INT NOT NULL,
    batch_date VARCHAR(20)  NOT NULL,      
    imsi VARCHAR(100) NOT NULL,
    msisdn VARCHAR(100) NOT NULL,
    vlr_number VARCHAR(100) NOT NULL,
    carrier_name VARCHAR(100),   
    country_name VARCHAR(100)   
);

-- \i /docker-entrypoint-initdb.d/ins_countries.sql
-- \i /docker-entrypoint-initdb.d/ins_carriers.sql