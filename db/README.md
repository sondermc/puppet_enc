# Why SQL
* the choise for sqlite is because of the flexibility while developing
* because of the constraints that can be laid on the data (avoid duplicates)
* Because of typecasting (the data can be forced to be stored in a particular format)
* the puppetdb is already based on SQL (postgres)

# Questions?
* Where to safely store the database credentials when using puppetdb? 

# CREATE DB
sqlite3: 
```bash
DB=db/puppet_enc.sqlite
rm -f ${DB}
sqlite3 ${DB} < db/schema.sql
sqlite3 ${DB} < db/data.sql
```

# Query DB
```bash
DB=db/puppet_enc.sqlite
sqlite3 ${DB} 'SELECT * from node';
sqlite3 ${DB} 'SELECT * from environment';
sqlite3 ${DB} 'SELECT * from role';

sqlite3 ${DB} '
SELECT node.id, node.certname, environment.name, role.name, node.created_on, node.updated_on 
  FROM node
  INNER JOIN environment ON node.environment_id = environment.id
  INNER JOIN role ON node.role_id = role.id
'
```