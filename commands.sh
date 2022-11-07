docker network create hw2
cd db
docker build -t db .
cd ..
cd webserver
docker build -t webserver .
cd ..
docker run -d --name db --network hw2 db
docker run -d --name webserver --network hw2 -p 8080:8080 webserver
#wait until webserver is running and then curl
sleep 5
docker exec -it webserver curl http://localhost:8080
