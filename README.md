Run database:
```bash
docker-compose up
```

Prepare database:
```bash
docker-compose exec mysql bash
mysql -p
# the root password goes here...
create database morphology;
GRANT ALL PRIVILEGES ON morphology.* TO 'gena'@'%';
exit;
cd /tmp/morphology
zcat words-russian-adjectives.sql.gz | mysql -u root -p morphology
zcat words-russian-adjectives-morf.sql.gz | mysql -u root -p morphology
zcat words-russian-nouns.sql.gz | mysql -u root -p morphology
zcat words-russian-nouns-morf.sql.gz | mysql -u root -p morphology
```

Export configuration (or create a `.env` file of form KEY=value):
```bash
export DISCORD_TOKEN=your_token
export DATABASE_URL=mysql://gena:gena_unsafe_password@localhost/morphology
```

Run bot:
```
cargo run
```