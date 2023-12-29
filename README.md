# Rust web frameworks benchmark

In this project we benchmark different Rust web frameworks to see which framework can handle more requests/second. We have a 512kb HTML file that each candidate should serve the directory in which the file is. The gap between these frameworks are not much and may change based on the hardware and after time with newer framework updates.

### Benchmarks on Intel core i5 12400:
| Name   |   Version   | Requests/sec | Transfer/sec |
|:------:|------------:|:------------:|:------------|
| Warp   |   0.3.6     |    8809.19   |    4.30GB   |
| Hyper  |   1.1.0     |    8699.47   |    4.25GB   |
| Axum   |   0.7.2     |    8663.31   |    4.23GB   |
| Gotham |   0.7.2     |    8613.04   |    4.21GB   |
| Actix  |   4.4.1     |    7617.35   |    3.72GB   |
| Rocket |   0.5.0     |    6595.12   |    3.22GB   |
| Salvo  |   0.63.1    |    6233.79   |    3.04GB   |

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