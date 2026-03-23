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

##########This part is for Rust and Sawtooth#########
Step 1) Install Rust, or check if it is already installed in your system.
This is for new installation
```bash
curl https://sh.rustup.rs -sSf | sh
```
I got error using this command, and here is how I solve it: This is for checking already in the system

```bash
ls ~/.cargo/bin
```
if cargo rustc rustup found in the list, Rust already in your system. check the version
```bash
~/.cargo/bin/cargo --version
```
Make the path permanent
```bash
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

Now create sawtooth client using this command

```bash
cargo new sawtooth_client
cd sawtooth_client
cargo run
```

It will create a new directory name "sawtooth_client". 
Inside sawtoot_client there is a folder "src". Inside src open main.rs file remove it, and and replace this main.rs file there.

replace cargo.toml file upload here.
 Next run it using this command
```bash
cargo run
```

make sure docker is running and port is correct 

```bash
curl http://localhost:8008/blocks
```

 






