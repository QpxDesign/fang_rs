# POSTGRES SQL:
sudo -i -u postgres
UPDATE authors SET perm_level = 5 WHERE email = 'email';
UPDATE authors SET rank = 2 WHERE email = 'email';
