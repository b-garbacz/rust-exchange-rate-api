# Rust-ExchangeRate-API

1. Enter to the project directory 
```sh
   $ cargo build --release
```
2. Add an environment variable named API_KEY which contains your api key to app.exchangerate-api.com
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
![image](https://github.com/b-garbacz/rust-exchange-rate-api/assets/45511879/8f58a23f-4a08-4096-98b1-2df1785719fa)
5. Default mode
```sh
  $ ./exchange-rate-api PLN USD 1000.123
```
![image](https://github.com/b-garbacz/rust-exchange-rate-api/assets/45511879/0c0c2b99-cf2d-4324-98ac-c86bd26edc34)
6. List all available currencies
```sh
  $ ./exchange-rate-api --codes
```
![image](https://github.com/b-garbacz/rust-exchange-rate-api/assets/45511879/d122f205-f3e3-424e-b1ac-afad63e1baa5)
6. List all available currencies and the current exchange rates against a single currency:
```sh
  $ ./exchange-rate-api USD
```
![image](https://github.com/b-garbacz/rust-exchange-rate-api/assets/45511879/8084c36c-5cfd-484c-947e-45fce3b7e865)
