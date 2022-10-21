# OpenFaaS Rust HTTP Templates

## Usage
```sh
faas-cli template pull https://github.com/JadKHaddad/OpenFaaS-Rust-HTTP-Templates

faas-cli new mycfunction-r      --lang rust
faas-cli new mycfunction-r-http --lang rust-http
faas-cli new mycfunction-r-ac   --lang rust-actix
```
## Notes
Mdified versions of **rust** and **rust-http** templates.
[Original templates](https://github.com/openfaas-incubator/rust-http-template)