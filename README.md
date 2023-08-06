# rust-jobs

## Configuration

### Environment variables

| Name                | Required? | Default   | Desc                |
| ------------------- | --------- | --------- | ------------------- |
| SURREALDB_HOST      | Y         |           | Surrealdb host      |
| SURREALDB_USERNAME  | Y         |           | Surrealdb user      |
| SURREALDB_PASSWORD  | Y         |           | Surrealdb pass      |
| SURREALDB_PORT      | N         | 8000      | Surrealdb port      |
| SURREALDB_NAMESPACE | N         | apps      | Surrealdb namespace |
| SURREALDB_DATABASE  | N         | rust_jobs | Surrealdb database  |

## Running locally

```bash
$ docker compose up -d
```

This will expose surrealdb on the port `8000`
The one can use https://surrealist.app/ and configure it like 

![Alt text](surrealist.png)

The password is surreal (see in `docker-compose.yaml`)