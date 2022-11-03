sudo docker network create hw2
cd db
sudo docker build -t db .
cd ..
cd webserver
sudo docker build -t webserver .
cd ..
sudo docker run -d --name db --network hw2 db
sudo docker run -d --name webserver --network hw2 -p 8080:8080 webserver
#wait until webserver is running and then curl
sleep 5
sudo docker exec -it webserver curl http://localhost:8080
