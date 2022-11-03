# basic-docker-db-api

This project creates two docker containers. One is a database that has a table called "people". This table contains some information on Game of Thrones characters (my favorite show). The other container is a webserver written in Rust. This webserver will call the database in the other container and use it to respond to http requests. The script spins up both containers and curls the webserver at the very end. 

## Run the Containers
To run these containers, execute the following commands:
```
chmod a+x ./commands.sh
./commands.sh
```
