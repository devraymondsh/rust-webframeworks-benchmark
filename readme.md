# Rust web frameworks benchmark

## **Requirements:**

### Docker: [Get Docker](https://docs.docker.com/get-docker/)

### Rust (+1.57): [Rustup](https://rustup.rs/)

## **Executing the benchmark:**

### **Macos/Linux**
#### Just run the following command in order to use the ready-to-use script:
```bash
bash scripts/run.sh
```

### **Windows**
#### Build the docker image:
```batch
docker build -t devraymondsh/rust_web_frameworks_benchmark .
```
#### Run the docker image:
##### Replace `<path_to_cargo_home>` with your CARGO_HOME directory and `<path_to_root>` with the path to root directory of this repository folder.
```batch
docker run --rm -it -e CARGO_HOME="<path_to_cargo_home>" -v="<path_to_cargo_home>:/.cargo" -v="<path_to_root>/scripts:/rust_web_frameworks_benchmark/scripts" -v="<path_to_root>/frameworks:/rust_web_frameworks_benchmark/frameworks" -v="<path_to_root>/benchmarking_log:/rust_web_frameworks_benchmark/benchmarking_log" devraymondsh/rust_web_frameworks_benchmark
```