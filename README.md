# rust-crud-api

This repository contains a Rust CRUD API project that provides basic CRUD (Create, Read, Update, Delete) operations for managing user data.

## Setup

### Prerequisites

Before setting up the project, ensure you have the following prerequisites installed:

- Rust programming language ([Installation Guide](https://www.rust-lang.org/tools/install))
- Docker ([Installation Guide](https://docs.docker.com/get-docker/))
- Docker Compose ([Installation Guide](https://docs.docker.com/compose/install/))

### Installation

1. Clone the project repository:
    ```bash
    git clone <repository_url>
    ```

2. Navigate to the project directory:
    ```bash
    cd <project_directory>
    ```

3. Build the Docker image:
    ```bash
    docker-compose build
    ```

### Configuration

Ensure that you have a PostgreSQL database URL configured. You can set it as an environment variable named `DATABASE_URL`. Example:
```bash
export DATABASE_URL=postgres://username:password@localhost/database_name
```

## Endpoints

### Create User

- **Endpoint**: `POST /users`
- **Description**: Creates a new user with the provided name and email.
- **Request Body**: JSON object containing `name` and `email`.
- **Response**:
    - Status Code: 200 OK if successful.
    - Status Code: 500 INTERNAL SERVER ERROR if an error occurs.

### Get User by ID

- **Endpoint**: `GET /users/:id`
- **Description**: Retrieves user details by ID.
- **Parameters**:
    - `id`: ID of the user to retrieve.
- **Response**:
    - Status Code: 200 OK if successful.
    - Status Code: 404 NOT FOUND if user with the provided ID does not exist.
    - Status Code: 500 INTERNAL SERVER ERROR if an error occurs.

### Get All Users

- **Endpoint**: `GET /users`
- **Description**: Retrieves details of all users.
- **Response**:
    - Status Code: 200 OK if successful.
    - Status Code: 500 INTERNAL SERVER ERROR if an error occurs.

### Update User

- **Endpoint**: `PUT /users/:id`
- **Description**: Updates the details of the user with the provided ID.
- **Parameters**:
    - `id`: ID of the user to update.
- **Request Body**: JSON object containing updated `name` and `email`.
- **Response**:
    - Status Code: 200 OK if successful.
    - Status Code: 404 NOT FOUND if user with the provided ID does not exist.
    - Status Code: 500 INTERNAL SERVER ERROR if an error occurs.

### Delete User

- **Endpoint**: `DELETE /users/:id`
- **Description**: Deletes the user with the provided ID.
- **Parameters**:
    - `id`: ID of the user to delete.
- **Response**:
    - Status Code: 200 OK if successful.
    - Status Code: 404 NOT FOUND if user with the provided ID does not exist.
    - Status Code: 500 INTERNAL SERVER ERROR if an error occurs.

## Usage Examples

### Creating a User

```bash
curl -X POST -H "Content-Type: application/json" -d '{"name":"John Doe","email":"john@example.com"}' http://localhost:8080/users
```


### Retrieving a User by ID

```bash
curl http://localhost:8080/users/1
```

### Retrieving All Users

```bash
curl curl http://localhost:8080/users
```

### Updating a User

```bash
curl curl -X PUT -H "Content-Type: application/json" -d '{"name":"Jane Doe","email":"jane@example.com"}' http://localhost:8080/users/1
```

### Deleting a User

```bash
curl curl -X DELETE http://localhost:8080/users/1
```