## Prediction Guard Rust Client

[![CircleCI](https://dl.circleci.com/status-badge/img/circleci/Cy6tWW4wpE69Ftb8vdTAN9/NrVgbqg2cCEGyBjGRJcNhf/tree/main.svg?style=svg)](https://dl.circleci.com/status-badge/redirect/circleci/Cy6tWW4wpE69Ftb8vdTAN9/NrVgbqg2cCEGyBjGRJcNhf/tree/main)
[![crates.io](https://img.shields.io/crates/v/prediction-guard.svg)](https://crates.io/crates/prediction-guard)

### Description

This crate provides functionality developed to simplify interfacing
with [Prediction Guard API](https://www.predictionguard.com/) in Rust.

### Requirements

To access the API, contact us [here](https://www.predictionguard.com/getting-started) to get an enterprise access token.
You will need this access token to continue.

### Usage

```rust
extern crate prediction_guard as pg_client;

use pg_client::{chat, client, models};

#[tokio::main]
async fn main() {
    let pg_env = client::PgEnvironment::from_env().expect("env keys");

    let clt = client::Client::new(pg_env).expect("client value");

    let req = chat::Request::<chat::Message>::new(models::Model::NeuralChat7B)
        .add_message(
            chat::Roles::User,
            "How do you feel about the world in general?".to_string(),
        )
        .max_tokens(1000)
        .temperature(0.85);

    let result = clt
        .generate_chat_completion(&req)
        .await
        .expect("error from generate chat completion");

    println!("\nchat completion response:\n\n {:?}", result);
}
```

Take a look at the `examples` directory for more examples.

### Docs

You can find the Prediction Guard API docs on the Prediction Guard website.

[API Docs](https://docs.predictionguard.com/docs/getting-started/welcome)

[API Reference](https://docs.predictionguard.com/api-reference/api-reference/check-api-health)

### Getting started

Once you have your api key you can use the `makefile` to run curl commands
for the different api endpoints. For example, `make curl-injection` will connect to
the injection endpoint and return the injection response. The `makefile` also allows you to run the different examples
such as `make run-injection` to run the injection example.

#### Licensing

```
Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    https://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```

Copyright 2024 Prediction Guard
