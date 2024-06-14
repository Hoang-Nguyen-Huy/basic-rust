# This directory is about basic CRUD using axum and sqlite 

## 1. Clone this repo

```bash
git clone https://github.com/Hoang-Nguyen-Huy/basic-rust.git
```

```bash
cd basic-rust
```

## 2. Set up

```
DATABASE_URL=sqlite:data.db
```

## 3. Run the project

```bash
cargo run --bin basic-crud
```

## 4. Test API on POSTMAN

### a. GET all tasks

```
http://localhost:3000/tasks
```

### b. POST task

```
http://localhost:3000/tasks
```

paste this into your request body

```JSON
{
    "title": "First Task",
    "completed": false
}
```




