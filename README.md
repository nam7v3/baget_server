# baget_server
[Baget](https://github.com/trungpq27/Baget) synchronize server written in rust, actix-web and diesel with jwt authentication
## Dependencies
- [rust](https://www.rust-lang.org/)
- [postgresql](https://www.postgresql.org/)
- [docker](https://www.docker.com/)
## API
### POST /auth/register
- Body: 
``` javascript
{
  "username": "nam7v3",
  "password": "nam7v3",
}
```
- Response:
``` javascript
{
  "status": "success" // "fail"
}
```
### POST /auth/login
- Body:
``` javascript
{
  "username": "nam7v3",
  "password": "nam7v3"
}
```
- Response:
``` javascript
{
  "status": "success", // "fail"
  "token": "<TOKEN>" // "message": "..."
}
```  
### GET /sync
- Header:
  ```
  Authorization: Bearer <TOKEN>
  ```
- Response:
``` javascript
{
  "status": "success", // "fail"
  "data": [
    "transactions": [
      {
          "transaction_id": 1,
          "entry_date": "Saturday",
          "amount": 32.0,
          "account": "account",
          "category": "category",
          "transaction_type": "type",
          "transaction_title": "title"
      }
      // ...
    ],
    "expenses": [
      {
        "expense_id": 1,
        "entry_date": "Saturday",
        "amount": 30.0,
        "expense": "expense",
      }
      // ...
    ]
  ]
}
```  
### POST /sync
- Header:
  ```
  Authorization: Bearer <TOKEN>
  ```
- Body
``` javascript
{
  "transactions": [
    {
        "entry_date": "Saturday",
        "amount": 32.0,
        "account": "account",
        "category": "category",
        "transaction_type": "type",
        "transaction_title": "title"
    }
    // ...
  ],
  "expenses": [
    {
      "entry_date": "Saturday",
      "amount": 30.0,
      "expense": "expense",
    }
    // ...
  ]
}
```  
