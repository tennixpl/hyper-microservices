# Hands-On Microservices with Rust

## Build, test, and deploy scalable and reactive microservices with Rust 2018




## Mac Kill on port

 ```sudo lsof -ti tcp:8080 | xargs kill```

### Hyper-service

[hyper-service](/hyper-service)

### random-service

[random-service](/random-service)






## Debug fun

Print out only debug or higher logs
```RUST_LOG=random_service=debug cargo run```