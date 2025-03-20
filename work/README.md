docker-compose up --build -d

docker exec -it roam-db psql -U myuser -d roamdb

SELECT * FROM users;



docker cp ins_countries.sql roam-db:/tmp/ins_countries.sql
docker exec -it roam-db psql -U myuser -d roamdb -f /tmp/ins_countries.sql


docker cp ins_carriers.sql roam-db:/tmp/ins_carriers.sql
docker exec -it roam-db psql -U myuser -d roamdb -f /tmp/ins_carriers.sql


select distinct (carrier_name,country_id,official_name_fr,code) from
(
select cr.carrier_name,cr.country_id,co.official_name_fr, cr.country_code||cr.national_destination_code code 
from dim_carriers cr, dim_countries co
where cr.country_id = co.id
and cr.country_code||cr.national_destination_code is not null
)



select * from dim_carriers 
where (carrier_id,id) in (select carrier_id,  max(id)  
from dim_carriers where country_code = '216' 
group by carrier_id);

select code_number, count(*) from (
select carrier_id, carrier_name, country_code||national_destination_code code_number 
from dim_carriers where country_code = '216' 
and national_destination_code is not null  
order by carrier_id,id) 
group by code_number;



WITH CTE AS (
select id, carrier_id , carrier_name , country_name , country_code , national_destination_code, country_code || national_destination_code as code,
ROW_NUMBER() OVER (PARTITION BY country_code || national_destination_code ORDER BY id DESC) AS rn
from dim_carriers
where country_code = '216'
and national_destination_code is not null
)
SELECT carrier_id , carrier_name , country_name , country_code , national_destination_code,  code
 FROM CTE WHERE rn = 1




WITH CTE AS (
select id, carrier_id , carrier_name , country_name , country_code , national_destination_code, country_code || national_destination_code as code,
ROW_NUMBER() OVER (PARTITION BY country_code || national_destination_code ORDER BY id DESC) AS rn
from dim_carriers
where national_destination_code is not null
)
SELECT country_name , carrier_id , carrier_name ,  country_code , national_destination_code,  code
FROM CTE WHERE rn = 1
ORDER by country_name,carrier_id