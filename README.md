# Memorable

This API is inteneded for developers who are still learning the **[Fetch API](https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API)** and the basic _CRUD_ operations (Create, Read, Update, Delete).

It can be used as teach/learning materials to grasp the basics of how the client (Web/Mobile) should interact with the server!

# How it works

The project doesn't use any database to store the data. Instead, it stores data **in memory** and uses the **client's IP address** to map the client to its own data. to avoid the server running out of memory, the server clears the cache every certain period of time and will put a limit on how much you can store on the server.

# API

| Method   | Route                | Description                     |
| -------- | -------------------- | ------------------------------- |
| `POST`   | `/api/v1/todos`      | **Create** new todo             |
| `GET`    | `/api/v1/todos`      | **Get** todos list              |
| `GET`    | `/api/v1/todos/{id}` | **Get** todo item using `ID`    |
| `PUT`    | `/api/v1/todos/{id}` | **Update** todo item using `ID` |
| `DELETE` | `/api/v1/todos/{id}` | **Delete** todo item using `ID` |

### Create TODO

###### Route: `POST /api/v1/todos`

###### Request body

All fileds are required, will get `401` status code if any field is missing or invalid.

```rs
struct CreateTodo {
    title: String,
    desc: String,
    is_done: bool,
}
```

###### JS Example

```js
fetch("https://memorable.ahmedibrahim.dev/api/v1/todos", {
  method: "POST",
  headers: {
    "Content-Type": "application/json",
  },
  body: JSON.stringify({
    title: "Learn fetch api",
    desc: "learning the basic CRUD operations",
    is_done: false,
  }),
})
  .then((res) => res.text())
  .then(console.log);
// {"message":"Todo created successfully"}
```

### Get TODOs

###### Route: `GET /api/v1/todos`

###### Response:

Array of all todos

```rs
struct Todo {
    id: usize,
    title: String,
    desc: String,
    is_done: bool,
}
```

###### JS Example

```js
fetch("https://memorable.ahmedibrahim.dev/api/v1/todos")
  .then((res) => res.json())
  .then(console.log);

// Response
[
  {
    id: 1,
    title: "Learn fetch api",
    desc: "learning the basic CRUD operations",
    is_done: false,
  },
];
```

### Get Todo Item

Get todo item by **id**.

###### Route: `GET /api/v1/todos/{id}`

###### Response:

Will respond with the todo item and status code **200 (success)** if it exists. Will return 404 (not found) status code.

###### JS Example

```js
fetch('https://memorable.ahmedibrahim.dev/api/v1/todos/1').then(res => res.json()).then(console.log)


// Response
{
    "id": 1,
    "title": "Learn fetch api",
    "desc": "learning the basic CRUD operations",
    "is_done": false
}
```

### Update todo by id

###### Route: `PUT /api/v1/todos/{id}`

###### Request:

In the request body can include any todo feild other than the ID (`title`, `desc`, `is_done`).

###### Response:

The API will respond back with the new updated todo.

###### JS Example

```js
fetch('https://memorable.ahmedibrahim.dev/api/v1/todos/1', {
    method: "PUT",
    headers: {
        "Content-Type": "application/json",
    },
    body: JSON.stringify({
        is_done: true,
    })
}).then(res => res.json()).then(console.log)

// Response
{
    "id": 1,
    "title": "Learn fetch api",
    "desc": "learning the basic CRUD operations",
    "is_done": true
}
```

### Delete todo by id

###### Route: `DELETE /api/v1/todos/{id}`

###### Response:

The API will respond back with a status code 200.

###### JS Example

```js
fetch("https://memorable.ahmedibrahim.dev/api/v1/todos/1", {
  method: "DELETE",
});
```
