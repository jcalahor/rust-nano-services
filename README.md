# rust-nano-services
Samples of nanoservices. 

# Overview
This project shows the usage of Nanoservices. As sample we built a simple calculator that can do Multiplication, Sum, Division and Substraction

Nanoservice documentation is located here:
https://nanoservices.io/docs/Docs/iomessaging/introduction/

# References
https://github.com/nanoservicesforge

https://github.com/nanoservicesforge/example-tcp-messaging


# Running the client and server
Running the server

```cd server && cargo run```

Runing the client

```cd client && cargo run```

Sample log of Server

```
Server listening on port 8080
Received: CalcWork(CalcWork { input_data1: 10, input_data2: 10, work_type: Mult, result: None, error: None })
Received: CalcWork(CalcWork { input_data1: 10, input_data2: 10, work_type: Sum, result: None, error: None })
```



Sample log of Client
```
CalcWork { input_data1: 10, input_data2: 10, work_type: Mult, result: Some(100), error: None }
CalcWork { input_data1: 10, input_data2: 10, work_type: Sum, result: Some(20), error: None }
```
