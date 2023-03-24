# To Do App

## docker/postgres
### run docker container based on docker-compose.yaml file configuration
docker-compose up

### list running containers
docker container ls  

### connect to database in interactive mode
docker exec -it [container id] psql -U username to_do
#### then \c - connect, \d - list of relations(tables), \d+ 'name' - details about relation, \q - quit


### run migrations
diesel migration run

### redo migrations 
diesel migration redo


### frontend
cd frontend
npm run dev

### backend
cargo run config.yml