extern crate hyper;
extern crate base64;

use docker;

use serde_json;

use errors::*;

use server::{InvokeRequestBody, InvokeResponseBody};

use std::io::Read;

pub fn run(container_id: &str, port: u16, program: &str, stdin: &[u8]) -> Result<(String, String, i32, u64)> {
    let container_id = match try!(docker::get_container_id(container_id)) {
        Some(container_id) => container_id,
        None => bail!("unable to find a running container with id {}. is the cds server running (automatically)?", container_id),
    };

    let addr = match try!(docker::get_public_addr(container_id.as_str(), port)) {
        Some(addr) => addr,
        None => bail!("unable to find public address of cds server running in container {}. is the port exposed?", container_id),
    };

    let req_body = InvokeRequestBody{
        stdin: base64::encode(stdin),
    };

    let url = format!("http://{}/run/{}", addr, program);
    let url = try!(hyper::Url::parse(&url)
        .chain_err(|| format!("unable to parse url {}", url)));
    let req_body = try!(serde_json::to_string(&req_body)
        .chain_err(|| "unable to serialize invocation request body"));

    let client = hyper::client::Client::new();
    let mut response = try!(client.post(url)
        .body(hyper::client::Body::SizedBody(&mut req_body.as_bytes(), req_body.len() as u64))
        .send()
        .chain_err(|| "HTTP client is unable to communicate with server"));

    let mut resp_string = String::new();
    try!(response.read_to_string(&mut resp_string)
        .chain_err(|| "unable to read server response body"));

    let resp_body: InvokeResponseBody = try!(serde_json::from_str(&resp_string)
        .chain_err(|| format!("unable to deserialize json response ({})", &resp_string)));

    if let Some(error) = resp_body.error {
        bail!("On error occurred in the server: {}", error);
    }

    Ok((resp_body.stdout, resp_body.stderr, resp_body.exit_status, resp_body.duration))
}
