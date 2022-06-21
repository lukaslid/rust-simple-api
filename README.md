# rust-api
Simple RESTful API created with rust, actix-web, Diesel, JWT.


### Running application

* Firstly generate a `secret.key` which will be used for JWT enocding/decoding. `head -c16 /dev/urandom > secret.key`
* Run `cp .env.example .env` to create environmental variables file.
* Create PostgreSQL database in pgAdmin, postgres cli or using diesel cli (if diesel cargo package installed run `diesel migration run`).
* You can start use PostgreSQL via provided `docker-compose.yml` running `docker-compose up -d`
* Build release: `cargo build --release`
* Run release version (on linux): `target/release/rust-api`
* Run debug version locally: `cargo run`


### Routes

| Route                | METHOD | BODY | Response | Description |
| ---------------------| ------ | ------ |:------------|------------|
|`/api/users`          | `GET`  |   `-`  | `[{"id": "..", "email": "..", "name": "..", "created_at": ".."}, { ... }` | Lists all users. Protected route, needs authorized user |
|`/api/users/<user_id>`| `GET`  |   `-`  | `{ "id": "f9f95d00-c9b4-4244-9048-f420ea38f873", "email": "..", "name": "..", "created_at": ".."}`  | Finds user by id. Protected route, needs authorized user |
|`/api/users`          | `POST` | `{"email": "user@email.com", "name": "Some User", "password": "Pa$$w0rd"}` | Success - returs entity, failure - error message.  | Creates new user (signup route). |
|`/api/login`          | `POST` | `{"email": "user@email.com", "password": "Pa$$w0rd"}`  | `{ "token": "ey...", "token_type": "bearer" }` | Returns token which should be added to Authorization header in order to reach secured routes |
|`/api/refresh-token`  | `POST` | `-` | Success returns new token: `{ "token": "ey...", "token_type": "bearer" }`. Failure - `401` | To refresh token a valid token is needed in Authorization header |



### ToDo's
* Add api as a service in `docker-compose.yml`.
* Add Unit/Integration tests.
* Embed migrations.
* Tune inital dockerfile.
* Pagination
