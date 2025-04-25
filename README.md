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
type = lastest, history

