CREATE TABLE IF NOT EXISTS mac_vendors (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS mac_addresses (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    mac_address TEXT UNIQUE NOT NULL,
    mac_vendor_id INTEGER,
    FOREIGN KEY(mac_vendor_id) REFERENCES mac_vendors(id)
);
