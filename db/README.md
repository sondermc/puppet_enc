# Why SQL
* the choise for sqlite is because of the flexibility while developing
* because of the constraints that can be laid on the data (avoid duplicates)
* Because of typecasting (the data can be forced to be stored in a particular format)
* the puppetdb is already based on SQL (postgres)

# Questions?
* Where to safely store the database credentials when using puppetdb? 

# CREATE DB
sqlite3: in bash
```bash
DATABASE_URL=db/puppet_enc.sqlite
rm -f ${DATABASE_URL}
sqlite3 ${DATABASE_URL} < db/schema.sql
sqlite3 ${DATABASE_URL} < db/data.sql
```

# Query DB
```bash
DATABASE_URL=db/puppet_enc.sqlite
sqlite3 ${DATABASE_URL} 'SELECT * from node';
sqlite3 ${DATABASE_URL} 'SELECT * from environment';
sqlite3 ${DATABASE_URL} 'SELECT * from role';

sqlite3 ${DATABASE_URL} '
SELECT node.id, node.certname, environment.name, role.name, node.created_on, node.updated_on 
  FROM node
  INNER JOIN environment ON node.environment_id = environment.id
  INNER JOIN role ON node.role_id = role.id
'

sqlite3 ${DATABASE_URL} "
SELECT environment.name AS environment, role.name AS role
  FROM node
  INNER JOIN environment ON node.environment_id = environment.id
  INNER JOIN role ON node.role_id = role.id
  WHERE node.certname = 'puppetserver.example.com';
SELECT environment.name AS environment, role.name AS role
  FROM node
  INNER JOIN environment ON node.environment_id = environment.id
  INNER JOIN role ON node.role_id = role.id
  WHERE node.certname = 'default';
"
```

# Issues
Need to solve this: on buldtime the rustcompiler checks if the database sql commands do have a valid syntax. So at this point it seems to need access to the database. To avoid this buildtime error:
```bash
DATABASE_URL=db/puppet_enc.sqlite
rm -f ${DATABASE_URL}
sqlite3 ${DATABASE_URL} < db/schema.sql
sqlite3 ${DATABASE_URL} < db/data.sql
```
After building, this file can be safely removed.
