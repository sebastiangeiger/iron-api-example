# Iron API Example [![Build Status](https://travis-ci.org/sebastiangeiger/iron-api-example.svg?branch=master)](https://travis-ci.org/sebastiangeiger/iron-api-example)

This repository demonstrates how to use iron to build a small restful API in rust.
It uses sqlite to store data. Acceptance tests were written in ruby.


## Getting started

You can use the usual `cargo` commands like `cargo build`, `cargo test` and `cargo run`.
On top of that there are acceptance level tests written in ruby.
To execute those make sure you have ruby installed, then execute `bundle install` and afterwards `bundle exec rspec`.
