```bash
docker build -f Dockerfile.client -t sawtooth-client-intkey:chime .
```
1) run this YAML using this command
 ```bash  
docker compose -f sawtooth-default-poet.yaml up -d --force-recreate --scale sawtooth-client=100
```
3) You can see the number of client using this command
```bash
 docker compose -f sawtooth-default-poet.yaml ps | grep sawtooth-client | wc -l
 ```
or
```bash
docker ps --format '{{.Names}}' | grep -i sawtooth-client | wc -l
```
it will show "100" the number of client


To send data randomly
```bash
for i in {1..100}
do
   docker-compose exec sawtooth-client-$i python send_transaction.py &
done
wait
```






