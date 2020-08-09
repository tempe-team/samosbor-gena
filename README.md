```bash
docker-compose up
```

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

```
export DISCORD_TOKEN=EjT2tBeQ2B8Qj_Q3JpBkFyhR7BdXStCu
export DATABASE_URL=mysql://gena:gena_unsafe_password@localhost/morphology
cargo run
```