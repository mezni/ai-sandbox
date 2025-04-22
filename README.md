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


create view v_roam_in_metrics as
select ms.name as metric_type, md.name as metric_name, md.description as metric_description, pr.period,cn.name , op.operator, mt.value  from metrics mt 
join metric_definition md on mt.metric_definition_id = md.metric_definition_id
join periods pr on pr.period_id = mt.period_id
join metrics_type ms on ms.metric_type_id = md.metric_type_id
left join countries cn on cn.country_id = mt.country_id
left join operators op on op.operator_id = mt.operator_id
;



INSERT INTO metric_definition (roam_direction_id, metric_type_id, name, description)
SELECT 
    rd.roam_direction_id,
    mt.metric_type_id,
    'number_subscribers_in_by_country',
    'Number of subscribers IN (By country)'
FROM roam_directions rd
CROSS JOIN metrics_type mt
WHERE rd.direction = 'IN' AND mt.name = 'GLOBAL';

INSERT INTO metrics (metric_definition_id, batch_id , period_id, value)
select  (select metric_definition_id from metric_definition where name = 'number_subscribers_in_by_country'),batch_id , period_id, sum(nsub ) 
from stg_roam_in stg join countries cnt on stg.country_id = cnt.country_id
join periods prd on stg.batch_date = prd.period
where cnt.name != 'Tunisia'
group by batch_id , period_id;



INSERT INTO metric_definition (roam_direction_id, metric_type_id, name, description)
SELECT 
    rd.roam_direction_id,
    mt.metric_type_id,
    'number_subscribers_in_by_country',
    'Number of subscribers IN (By country)'
FROM roam_directions rd
CROSS JOIN metrics_type mt
WHERE rd.direction = 'IN' AND mt.name = 'COUNTRY';



INSERT INTO metrics (metric_definition_id, batch_id , period_id, country_id, value)
select  (select metric_definition_id from metric_definition where name = 'number_subscribers_in_by_country'),batch_id , period_id, stg.country_id, sum(nsub ) 
from stg_roam_in stg join countries cnt on stg.country_id = cnt.country_id
join periods prd on stg.batch_date = prd.period
where cnt.name != 'Tunisia'
group by batch_id , period_id, stg.country_id;