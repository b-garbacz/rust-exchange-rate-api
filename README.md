# Rust-ExchangeRate-API
1. Classic lauch
2. Use Docker environment

## Api Key
Obtain your API Key from  https://www.exchangerate-api.com
## Classic launch (Linux)
1. Enter to the project directory 
```sh
   $ cargo build --release
```
2. Add an environment variable named API_KEY which contains your api key to https://www.exchangerate-api.com API
```sh
   $ export API_KEY=111111111111111111111111
```
3. Enter to the /target/release
```sh
   $ cd /target/release
```
4. You can use the --help flag to see how to use the program
```sh
   $ ./exchange-rate-api --help
```
![image](https://github.com/b-garbacz/rust-exchange-rate-api/assets/45511879/8f58a23f-4a08-4096-98b1-2df1785719fa)<br>
5. Default mode
```sh
  $ ./exchange-rate-api PLN USD 1000.123
```
![image](https://github.com/b-garbacz/rust-exchange-rate-api/assets/45511879/0c0c2b99-cf2d-4324-98ac-c86bd26edc34) <br>
6. List all available currencies
```sh
  $ ./exchange-rate-api --codes
```
![image](https://github.com/b-garbacz/rust-exchange-rate-api/assets/45511879/d122f205-f3e3-424e-b1ac-afad63e1baa5)<br>
6. List all available currencies and the current exchange rates against a single currency:
```sh
  $ ./exchange-rate-api USD
```
![image](https://github.com/b-garbacz/rust-exchange-rate-api/assets/45511879/8084c36c-5cfd-484c-947e-45fce3b7e865)<br>

## Use Docker environment

1. Pull docker image from https://hub.docker.com/repository/docker/bgarbach/exchange_rate_api/general or use:
```sh
  $ docker pull bgarbach/exchange_rate_api
```

2. You can use the --help flag to see how to use the program
```sh
   $ docker run -e API_KEY=xxxxxxx bgarbach/exchange_rate_api --help
```

3. Default mode
```sh
   $ docker run -e API_KEY=xxxxxxx bgarbach/exchange_rate_api PLN USD 1000.123
```

4. List all available currencies
```sh
   $ docker run -e API_KEY=xxxxxxx bgarbach/exchange_rate_api --codes
```

5. List all available currencies and the current exchange rates against a single currency:
```sh
   $ docker run -e API_KEY=xxxxxxx bgarbach/exchange_rate_api USD
```
