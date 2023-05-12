

Pgsql docker command 

// connect to database
psql -h localhost -U zerotoprod -p 5432 -d newsletter

// create database
createdb -h localhost -p 5432 -U zerotoprod <database name>

 psql -h localhost -U zerotoprod -p 5432 -d <database name> -c "CREATE USER db_user WITH PASSWORD 'db_pass' SUPERUSER CREATEDB CREATEROLE INHERIT LOGIN;"