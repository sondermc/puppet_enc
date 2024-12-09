INSERT INTO environment (name,description) VALUES ('production', 'The example.com Production Environment');

INSERT INTO role (name,description) VALUES ('role::default','The default role');
INSERT INTO role (name,description) VALUES ('role::puppetserver','All nodes become a puppetserver');

INSERT INTO node (id,certname, environment_id, role_id) 
  VALUES(
    0,
    'default',
    (SELECT id FROM environment WHERE name     = 'production'),
    (SELECT id FROM role        WHERE name     = 'role::default')
  );
INSERT INTO node (certname, environment_id, role_id) 
  VALUES(
    'puppetserver.example.com',
    (SELECT id FROM environment WHERE name     = 'production'),
    (SELECT id FROM role        WHERE name     = 'role::puppetserver')
  );
