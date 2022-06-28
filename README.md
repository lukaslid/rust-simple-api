# rust-simple-api
Simple RESTful API created with rust, actix-web, Diesel, JWT.


### Running application

#### Manual
* Firstly generate a `secret.key` which will be used for JWT enocding/decoding. `head -c16 /dev/urandom > src/secret.key`
* Run `cp .env.example .env` to create environmental variables file, edit variables to match your needs.
* Create PostgreSQL database in pgAdmin, postgres cli or using diesel cli (if diesel cargo package installed run `diesel setup`).
* You can start use PostgreSQL via provided `docker-compose.yml` running `docker-compose up` or use a system service
* Build release: `cargo build --release`
* Run release version (on linux): `target/release/api`
* Run debug version locally: `cargo run`

#### Run with docker
* Firstly generate a `secret.key` which will be used for JWT enocding/decoding. `head -c16 /dev/urandom > src/secret.key`
* Build docker image locally `docker build -t users-api .`
* Edit environmental files in `docker-compose.yml` if needed.
* Run `docker-compose up` to start the application. However, right now DB must be created before API service starts. Create database in pgAdmin, postgres cli or using diesel cli `diesel setup`. To run this command `DATABASE_URL` must be stored in `.env`. Run `cp .env.example .env` to use the default parameters before running `diesel setup`.


### Routes

| Route                | METHOD | BODY | Response | Description |
| ---------------------| ------ | ------ |:------------|------------|
|`/api/users`          | `GET`  |   `-`  | `[{"id": "..", "email": "..", "name": "..", "created_at": ".."}, { ... }` | Lists all users. Protected route, needs authorized user |
|`/api/users/<user_id>`| `GET`  |   `-`  | `{ "id": "f9f95d00-c9b4-4244-9048-f420ea38f873", "email": "..", "name": "..", "created_at": ".."}`  | Finds user by id. Protected route, needs authorized user |
|`/api/users`          | `POST` | `{"email": "user@email.com", "name": "Some User", "password": "Pa$$w0rd"}` | Success - returs entity, failure - error message.  | Creates new user (signup route). |
|`/api/login`          | `POST` | `{"email": "user@email.com", "password": "Pa$$w0rd"}`  | `{ "token": "ey...", "token_type": "bearer" }` | Returns token which should be added to Authorization header in order to reach secured routes |
|`/api/refresh-token`  | `POST` | `-` | Success returns new token: `{ "token": "ey...", "token_type": "bearer" }`. Failure - `401` | To refresh token a valid token is needed in Authorization header |


### Testing
* Create db with `diesel_cli`: `diesel setup --database-url='postgres://postgres:admin@localhost/test_api_db'`. Here I am using test DB url which is also in `.env` file.
* Run `cargo test`



### ToDo's
* Add Unit/Integration tests. (Increase coverage)
* Pagination
