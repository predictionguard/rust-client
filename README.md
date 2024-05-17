## Prediction Guard Rust Client



### Description

This crate provides functionality developed to simplify interfacing with [Prediction Guard API](https://www.predictionguard.com/) in Rust.

### Requirements

To access the API, contact us [here](https://www.predictionguard.com/getting-started) to get an enterprise access token. You will need this access token to continue.

### Usage

```rust
use std::env;

use pg_rust_client as pg_client;
use pg_client::{client, completion};

#[tokio::main]
async fn main() {
    let key = env::var("PGKEY").expect("PG Api Key");
    let host = env::var("PGHOST").expect("PG Host");

    let clt = client::Client::new(&host, &key).expect("client value");

    let req = completion::ChatRequest {
        model: completion::Models::NeuralChat7B,
        messages: vec![completion::Message {
            role: completion::Roles::User,
            content: "How do you feel about the world in general?".to_string(),
        }],
        max_tokens: 1000,
        temperature: 1.1,
    };

    let result = clt
        .generate_chat_completion(&req)
        .await
        .expect("error from generate chat completion");

    println!("\nchat completion response:\n\n {:?}", result);
}
```

### Docs

You can find the Prediction Guard API docs on the Prediction Guard website.

[API Docs](https://docs.predictionguard.com/docs/getting-started/welcome)

[API Reference](https://docs.predictionguard.com/api-reference/api-reference/check-api-health)

### Getting started

The example below shows you can get started connecting to the api:



#### Licensing

```
Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```
Copyright 2024 Prediction Guard
