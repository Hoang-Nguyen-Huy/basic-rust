# This directory is about basic CRUD using axum and sqlite 

## 1. Clone this repo

```bash
git clone https://github.com/Hoang-Nguyen-Huy/basic-rust.git
```

```bash
cd basic-rust
```

## 2. Set up

paste this to your .env file

```
DATABASE_URL=sqlite:data.db
```

## 3. Run the project

```bash
cargo run --bin basic-crud
```

## 4. Test API on POSTMAN

### a. GET all tasks

<img width="887" alt="Screen Shot 2024-06-14 at 15 24 33" src="https://github.com/Hoang-Nguyen-Huy/basic-rust/assets/121879570/39f77022-e0ee-44e8-8931-b0786ba8f94c">

```
http://localhost:3000/tasks
```

### b. POST task

<img width="890" alt="Screen Shot 2024-06-14 at 15 21 11" src="https://github.com/Hoang-Nguyen-Huy/basic-rust/assets/121879570/b7d41c89-d1d0-474a-9ab0-8eeb9260d6c2">

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

### c. GET task

<img width="854" alt="Screen Shot 2024-06-14 at 15 43 32" src="https://github.com/Hoang-Nguyen-Huy/basic-rust/assets/121879570/a8270502-c4d6-45cb-a811-51b0d3423959">

```
http://localhost:3000/tasks/:id
```






