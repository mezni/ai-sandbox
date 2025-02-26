CREATE TABLE IF NOT EXISTS mac_vendors (
    id INTEGER PRIMARY KEY ,
    designation TEXT NOT NULL,
    org_name TEXT
);

CREATE TABLE IF NOT EXISTS mac_addresses (
    id INTEGER PRIMARY KEY ,
    mac_address TEXT UNIQUE NOT NULL,
    mac_vendor_id INTEGER,
    FOREIGN KEY(mac_vendor_id) REFERENCES mac_vendors(id)
);

INSERT INTO mac_vendors (id, designation, org_name) VALUES (2650041, '28:6F:B9', 'Nokia'); 
