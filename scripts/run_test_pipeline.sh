#!/bin/bash

# move to directory of the project
SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd $SCRIPTPATH
cd ..


# spin up docker and hold script until accepting connections
docker-compose up -d
until pg_isready -h localhost -p 5432 -U username
do
  echo "Waiting for postgres"
  sleep 2;
done

cargo build
cargo test

# run server in background
cargo run config.yml &
SERVER_PID=$!
sleep 5

diesel migration run

# create the user
CREATE_RESPONSE=$(curl --write-out "\nHTTPSTATUS:%{http_code}\n" --location --request POST 'http://localhost:8080/v1/user/create' \
--header 'Content-Type: application/json' \
--data-raw '{
    "name": "test",
    "email": "test@email.com",
    "password": "test"
}')
# login getting a fresh token
LOGIN_RESPONSE=$(curl --location --request POST 'http://localhost:8080/v1/auth/login' \
--header 'Content-Type: application/json' \
--data-raw '{
    "username": "test",
    "password": "test"
}')

#save token to file and print contents
echo $LOGIN_RESPONSE | jq -r '. | fromjson | .token' > ./fresh_token.json

# modify Postman collection file, run test, and print response
jq '.auth.apikey[0].value = "'$(cat ./fresh_token.json)'"' scripts/to_do_items.postman_collection.json > test_newman.json

NEWMAN_RESPONSE=$(newman run test_newman.json)

rm ./test_newman.json
rm ./fresh_token.json

# shut down rust server
kill $SERVER_PID

docker-compose down