# Task API

This is a simple RESTful API for managing tasks, built with Rust using the Axum web framework. It provides endpoints for creating, retrieving, updating, and deleting tasks.

## Features

- **Create Task:** Add new tasks with a title and description.
- **Get All Tasks:** Retrieve a list of all existing tasks.
- **Get Task by ID:** Fetch a specific task using its unique identifier.
- **Update Task:** Modify an existing task's details.
- **Delete Task:** Remove a task from the system.

## Technologies Used

- **Rust:** A language that empowers everyone to build reliable and efficient software.
- **Axum:** A web application framework that focuses on ergonomics and modularity, built on Tokio and Hyper.
- **Tokio:** An asynchronous runtime for Rust.
- **Serde:** A framework for serializing and deserializing Rust data structures efficiently and generically.

## Setup

To get started with the Task API, follow these steps:

### Prerequisites

- Rust and Cargo (Rust's package manager) installed. If you don't have them, you can install them via `rustup`:
  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

### Installation

1. **Clone the repository:**
   ```sh
   git clone https://github.com/your-username/task-api.git
   cd task-api
   ```

2. **Build the project:**
   ```sh
   cargo build
   ```

## Usage

### Running the Server

To start the API server, run the following command:

```sh
cargo run
```

The API will be accessible at `http://127.0.0.1:3000` (or `http://localhost:3000`).

### API Endpoints

Here are the available API endpoints:

- **`POST /tasks`**
  - **Description:** Create a new task.
  - **Request Body (JSON):**
    ```json
    {
      "title": "Buy groceries",
      "description": "Milk, eggs, bread, and fruits"
    }
    ```
  - **Response (JSON):** Returns the created task with an assigned ID.

- **`GET /tasks`**
  - **Description:** Get all tasks.
  - **Response (JSON):** An array of task objects.

- **`GET /tasks/:id`**
  - **Description:** Get a task by its ID.
  - **Response (JSON):** The task object if found, otherwise a 404 error.

- **`PUT /tasks/:id`**
  - **Description:** Update an existing task.
  - **Request Body (JSON):**
    ```json
    {
      "title": "Buy groceries (updated)",
      "description": "Milk, eggs, bread, fruits, and vegetables"
    }
    ```
  - **Response (JSON):** The updated task object.

- **`DELETE /tasks/:id`**
  - **Description:** Delete a task by its ID.
  - **Response:** 204 No Content on successful deletion.

## Contributing

Contributions are welcome! Please feel free to open issues or submit pull requests.

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.