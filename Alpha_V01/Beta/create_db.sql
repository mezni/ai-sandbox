The design allows for:

    Multi-level roaming policies (by country, network, subscriber group)

    Real-time monitoring of roaming activities

    Historical reporting on steering effectiveness

    Capacity planning based on HLR utilization

    Audit trail of all steering commands and their results




CREATE TABLE networks (
    network_id SERIAL PRIMARY KEY,
    mcc CHAR(3) NOT NULL,
    mnc CHAR(3) NOT NULL,
    plmn CHAR(6) NOT NULL,
    tadig CHAR(6) NOT NULL,
    operator_id INTEGER REFERENCES operators(operator_id),
    status VARCHAR(20) DEFAULT 'ACTIVE',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_by TEXT,
    updated_at TIMESTAMP,
    updated_by TEXT
    UNIQUE(mcc, mnc)
);


CREATE TABLE subscribers (
    subscriber_id SERIAL PRIMARY KEY,
    imsi VARCHAR(15) NOT NULL UNIQUE,
    msisdn VARCHAR(15),
    roam_type VARCHAR(15),
    last_seen TIMESTAMP
);



CREATE TABLE roaming_policies (
    policy_id SERIAL PRIMARY KEY,
    policy_name VARCHAR(100) NOT NULL,
    description TEXT,
    priority INTEGER DEFAULT 0,
    is_active BOOLEAN DEFAULT TRUE,
    valid_from TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    valid_to TIMESTAMP,
    created_by VARCHAR(50),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);


CREATE TABLE roaming_events (
    event_id SERIAL PRIMARY KEY,
    subscriber_id INTEGER REFERENCES subscribers(subscriber_id),
    visited_network_id INTEGER REFERENCES networks(network_id),
    event_type VARCHAR(20) NOT NULL, -- 'ATTACH', 'DETACH', 'LOCATION_UPDATE'
    event_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    location_info VARCHAR(100),
    serving_node VARCHAR(100),
    policy_applied INTEGER REFERENCES roaming_policies(policy_id),
    is_steered BOOLEAN DEFAULT FALSE
);



CREATE TABLE network_utilization (
    utilization_id SERIAL PRIMARY KEY,
    network_id INTEGER REFERENCES networks(network_id),
    hlr_id INTEGER REFERENCES hlr_records(hlr_id),
    record_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    total_subscribers INTEGER NOT NULL,
    active_subscribers INTEGER NOT NULL,
    utilization_percentage DECIMAL(5,2) GENERATED ALWAYS AS (
        (active_subscribers::DECIMAL / GREATEST(total_subscribers, 1)) * 100
    ) STORED
);


CREATE INDEX idx_subscriber_imsi ON subscribers(imsi);
CREATE INDEX idx_subscriber_hlr ON subscribers(hlr_id);
CREATE INDEX idx_roaming_events_subscriber ON roaming_events(subscriber_id);
CREATE INDEX idx_roaming_events_time ON roaming_events(event_time);
CREATE INDEX idx_network_utilization ON network_utilization(network_id, record_time);
CREATE INDEX idx_policy_network ON policy_network_mapping(policy_id, network_id);




quality_metrics
id, start_date, end_time, type(global, county, operator, subscriber),agg_type, values



CREATE TABLE metrics_type (
  metrics_type_id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  description TEXT,
  unit VARCHAR(255)
);

CREATE TABLE metric_definition (
  id SERIAL PRIMARY KEY,
  metric_type_id INTEGER NOT NULL,
  name VARCHAR(255) NOT NULL,
  description TEXT,
  formula TEXT,
  data_source VARCHAR(255),
  FOREIGN KEY (metric_type_id) REFERENCES metrics_type(id)
);

INSERT INTO metrics_type (name, description, unit)
VALUES 
  ('Revenue', 'Total revenue earned', 'dollars'),
  ('Customer Satisfaction', 'Measure of customer satisfaction', 'percentage');

INSERT INTO metric_definition (metric_type_id, name, description, formula, data_source)
VALUES 
  (1, 'Monthly Revenue', 'Total revenue earned per month', 'total revenue / number of months', 'financial database'),
  (2, 'Customer Satisfaction Score', 'Average customer satisfaction rating', 'average(customer satisfaction ratings)', 'customer survey');


Total roamers out by date
Total roamers in by date
Total anomalies by date
Total notifications by date
