docker exec -it database psql -U myuser -d roamdb

{
  "data": [
    {
      "last_load": "2025-04-19T10:30:00Z",
      "filename": "roam_in_20250418.csv",
      "status": "Success"
    },
    {
      "last_load": "2025-04-19T10:45:00Z",
      "filename": "roam_out_20250418.csv",
      "status": "Failed"
    }
  ]
}

COPY load_sor_plan (country, operator, rate, routage)
FROM '/tmp/sor_plan.csv'
DELIMITER ',' CSV HEADER;


SELECT ctn.common_name, ope.operator, fct.rate, fct.routage, fct.created_by
from load_sor_plan fct JOIN countries ctn ON fct.country = ctn.common_name
JOIN operators ope ON fct.operator = ope.operator
WHERE ctn.country_id = ope.country_id;


{
  "data": [
    {
      "roamin": 27530,
      "roamout": 58640,
      "alerts": 12,
      "notifications":4
    }
  ]
}


{
  "data": [
    {
      "date": "2025-04-01",
      "Nombre": 64000
    },
    {
      "date": "2025-04-02",
      "Nombre": 53222
    },
    {
      "date": "2025-04-03",
      "Nombre": 57888
    },
    {
      "date": "2025-04-04",
      "Nombre": 58934
    },       
    {
      "date": "2025-04-05",
      "Nombre": 52087
    },
  ]
}



select cnt.name, sum(nsub ) ,  sum(nsuba)
from stg_roam_in stg join countries cnt on stg.country_id = cnt.country_id
group by cnt.name
order by 2 desc


select  sum(nsub ) ,  sum(nsuba)
from stg_roam_in stg join countries cnt on stg.country_id = cnt.country_id
where cnt.name != 'Tunisia';






Number of subscribers IN (Total)
Number of subscribers IN (By country)
Number of subscribers IN (By operator)

Number of active subscribers IN (Total)
Number of active subscribers IN (By country)
Number of active subscribers IN (By operator)


Top Roaming Destinations
Top Inbound Networks
Partner Performance











select date_str,country,operator,value from v_roam_in_metrics 
where  metric_name ='number_subscribers_out_by_operator' order by value desc;


roamdb=# select * from countries where common_name = 'Switzerland';
roamdb=# 
roamdb=# select * from operators where country_id = 78;







localhost:3000/api/v1/metrics/metrics?direction=intot&dimensions=global&start_date=2025-04-24&end_date=2025-04-24&count=1


direction=intot ,inact, out  
dimensions=global, country, operator
start_date
end_date
count=1

curl "http://localhost:3000/api/v1/metrics?direction=out&dimensions=global"


curl "http://localhost:3000/api/v1/metrics?direction=out&dimensions=global"

direction=intot ,inact, out  
dimensions=global, country, operator
kind = lastest, history




CREATE TABLE IF NOT EXISTS sor_plan_config (
    id SERIAL PRIMARY KEY,
    country_id INT NOT NULL,
    operator_id INT NOT NULL,
    rate TEXT,
    routage TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_by TEXT,
    updated_at TIMESTAMP,
    updated_by TEXT
);



CREATE TABLE IF NOT EXISTS roam_out_perf (
    roam_out_perf_id SERIAL PRIMARY KEY,    
    date_id INT NOT NULL,
    batch_id INT NOT NULL,
    country_id INT,
    operator_id INT,
    country_count INT NOT NULL,
    operator_count INT NOT NULL,
    percent REAL
);




INSERT INTO roam_out_perf (date_id, batch_id, country_id, operator_id, country_count, operator_count, percent)
SELECT
    d.date_id,
    t.batch_id,    
    t.country_id,
    t.operator_id,
    COUNT(*) AS count_by_country_operator,
    c.total_by_country,
    ROUND(100.0 * COUNT(*) / c.total_by_country, 2) AS percentage
FROM stg_roam_out t
JOIN (
    SELECT country_id, COUNT(*) AS total_by_country
    FROM stg_roam_out
    WHERE batch_id = 1
    GROUP BY country_id
) c ON t.country_id = c.country_id
JOIN dates d ON t.batch_date = d.date_str
WHERE t.batch_id = 1
GROUP BY  d.date_id,t.batch_id, t.country_id, t.operator_id, c.total_by_country
ORDER BY  d.date_id,t.batch_id, t.country_id, t.operator_id;


INSERT INTO notifications (date_id, batch_id, rule_id, ref_id, message) 

SELECT date_id, batch_id, rule_id, ref_id, 
       '- ' || operator || ' ('|| common_name ||') config=' || rate || ' reel=' || percent 
FROM (
    SELECT 
        agg.date_id, 
        agg.batch_id, 
        (SELECT id FROM rules WHERE name = 'sor_plan_deviation') AS rule_id, 
        agg.roam_out_perf_id AS ref_id,
        pln.rate,
        agg.percent,
        ope.operator,
        cnt.common_name
    FROM (
        SELECT *
        FROM roam_out_perf fct
        WHERE batch_id = 1
          AND country_id IN (SELECT country_id FROM sor_plan_config)
    ) agg
    LEFT JOIN sor_plan_config pln 
        ON agg.country_id = pln.country_id 
       AND agg.operator_id = pln.operator_id
    JOIN operators ope 
        ON pln.operator_id = ope.operator_id
    JOIN countries cnt ON agg.country_id = cnt.country_id 
    WHERE agg.percent NOT BETWEEN COALESCE(pln.rate::float, 0) - 2 
                          AND COALESCE(pln.rate::float, 0) + 2
) deviations;