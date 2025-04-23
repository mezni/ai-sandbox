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







roamdb=# select * from metrics;
 metric_id | metric_definition_id | batch_id | date_id | country_id | operator_id | subscriber_id | value 
-----------+----------------------+----------+---------+------------+-------------+---------------+-------


INSERT INTO metrics (metric_definition_id, batch_id , date_id, value)
SELECT (SELECT metric_definition_id FROM metric_definition WHERE name = 'number_subscribers_in'), stg.batch_id, dat.date_id, SUM(nsub) AS value
FROM stg_roam_in stg
JOIN countries cnt ON stg.country_id = cnt.country_id
JOIN dates dat ON stg.batch_date = dat.date_str 
WHERE stg.operator_id != (
SELECT operator_id FROM operators opr 
JOIN countries cnt ON opr.country_id = cnt.country_id
WHERE cnt.common_name = (select value from global_config WHERE key='home_country')
AND opr.operator = (select value from global_config WHERE key='home_operator')  
)
AND stg.batch_id = 2
GROUP BY stg.batch_id, dat.date_id;