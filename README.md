# Rust web frameworks benchmark

In this project we benchmark different Rust web frameworks to see which framework can handle more requests/second. We have a 512kb HTML file that each candidate should serve the directory in which the file is. The gap between these frameworks are not much and may change based on the hardware and after time with newer framework updates.

### Benchmarks on Intel core i5 12400:
|  Name  | Version    | Requests/sec | Transfer/sec |
|:------:|------------|:------------:|:------------:|
| Axum   | 0.6.18     |    8497.79   |    4.15GB   |
| Hyper  | 0.14.26    |    8310.59   |    4.06GB   |
| Warp   | 0.3.5      |    8282.09   |    4.04GB   |
| Actix  | 4.3.1      |    8157.53   |    3.98GB   |
| Gotham | 0.7.1      |    7869.12   |    3.84GB   |
| Rocket | 0.5.0-rc.3 |    6429.14   |    3.14GB   |

### Enter the following command to build:
```bash
docker build -t devraymondsh/rust-webframeworks-benchmark .
```
### Enter the following command to run:
```bash
docker run -it devraymondsh/rust-webframeworks-benchmark
```
### Enter the following command to run with custom options:
```bash
docker run -e="RATE=50000" -e="THREADS=8" -e="CLIENTS=200" -e="DURATION=10" -it devraymondsh/rust-webframeworks-benchmark
```
Feel free to replace values with your own desire.