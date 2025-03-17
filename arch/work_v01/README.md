docker-compose up --build -d

docker exec -it roam-db psql -U myuser -d roamdb

SELECT * FROM users;