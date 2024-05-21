use std::env;

extern crate prediction_guard as pg_client;
use pg_client::{client, factuality};

#[tokio::main]
async fn main() {
    let key = env::var("PGKEY").expect("PG Api Key");
    let host = env::var("PGHOST").expect("PG Host");

    let clt = client::Client::new(&host, &key).expect("client value");

    let req = factuality::Request {
            reference: "The President shall receive in full for his services during the term for which he shall have been elected compensation in the aggregate amount of 400,000 a year, to be paid monthly, and in addition an expense allowance of 50,000 to assist in defraying expenses relating to or resulting from the discharge of his official duties. Any unused amount of such expense allowance shall revert to the Treasury pursuant to section 1552 of title 31, United States Code. No amount of such expense allowance shall be included in the gross income of the President. He shall be entitled also to the use of the furniture and other effects belonging to the United States and kept in the Executive Residence at the White House.".to_string(),
		    text: "The president of the united states can take a salary of one million dollars".to_string(),
    };

    let result = clt
        .check_factuality(&req)
        .await
        .expect("error from factuality");

    println!("\n\nfactuality response:\n{:?}\n\n", result);
}
