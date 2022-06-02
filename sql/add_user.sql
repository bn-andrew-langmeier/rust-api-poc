INSERT INTO testing.users(email)
VALUES ($1)
RETURNING $table_fields;
