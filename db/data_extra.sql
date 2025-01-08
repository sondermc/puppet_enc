INSERT INTO node (certname, environment_id, role_id) 
  VALUES(
    'extra.example.com',
    (SELECT id FROM environment WHERE name     = 'production'),
    (SELECT id FROM role        WHERE name     = 'role::puppetserver')
  );
