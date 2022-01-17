use chrono::Utc;
use quick_xml::de::from_str;
use reqwest;
use serde::Deserialize;
use anyhow::Result;
use sha2::{Sha256, Digest};
use hmac::{Hmac, Mac};

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Owner {
    #[serde(rename = "ID")]
    id: String,
    display_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Contents {
    key: String,
    last_modified: String,
    e_tag: String,
    owner: Owner,
    storage_class: String,
    size: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ListBucketResult {
    name: String,
    delimiter: String,
    marker: String,
    max_keys: usize,
    prefix: String,
    is_truncated: bool,
    contents: Vec<Contents>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Credentials {
    access_key: String,
    secret_key: String,
}

fn main() -> Result<()> {
    let endpoint = "http://localhost:9000";
    let access_key = "AKIAIOSFODNN7EXAMPLE";
    let secret_key = "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY";
    let creds = Credentials {
        access_key: access_key.to_string(),
        secret_key: secret_key.to_string(),
    };
    dbg!(&creds);

    let client = reqwest::blocking::Client::new();

    let request = client.get(format!("{}/public-read/", endpoint)).build()?;
    dbg!(&request);
    let response = client.execute(request)?.text()?;
    dbg!(&response);

    let result: ListBucketResult = from_str(&response)?;
    dbg!(result);


    let mut request = client.get(format!("{}/private", endpoint)).build()?;
    add_host_header(&mut request)?;
    add_amz_headers(&mut request)?;
    sign_request(&mut request, &creds)?;
    dbg!(&request);
    let response = client.execute(request)?.text()?;
    dbg!(&response);

    let result: ListBucketResult = from_str(&response)?;
    dbg!(result);

    Ok(())
}


fn add_host_header(request: &mut reqwest::blocking::Request) -> Result<()> {
    let host = request.url().host_str().unwrap();
    let port = request.url().port_or_known_default().unwrap();
    let value = format!("{host}:{port}").parse()?;
    let headers = request.headers_mut();
    headers.insert(reqwest::header::HOST, value);
    Ok(())
}

fn add_amz_headers(request: &mut reqwest::blocking::Request) -> Result<()> {
    let x_amz_content_sha256 = reqwest::header::HeaderName::from_static("x-amz-content-sha256");
    let content_sha256 = Sha256::digest(match request.body() {
        Some(body) => body.as_bytes().unwrap(),
        None => b"",
    });
    let content_sha256 = format!("{:x}", content_sha256);

    let x_amz_date = reqwest::header::HeaderName::from_static("x-amz-date");
    let date = Utc::now();
    let date = date.format("%Y%m%dT%H%M%SZ").to_string();

    let headers = request.headers_mut();
    headers.insert(x_amz_content_sha256, reqwest::header::HeaderValue::from_str(content_sha256.as_str())?);
    headers.insert(x_amz_date, reqwest::header::HeaderValue::from_str(date.as_str())?);

    Ok(())
}

fn sign_request(request: &mut reqwest::blocking::Request, creds: &Credentials) -> Result<()> {
    let mut canonical_request_query = String::new();
    // TODO: this will add an extra `&` at the beginning. find a better way to do this
    for (arg, value) in request.url().query_pairs() {
        canonical_request_query += format!("&{}={}", arg, value).as_str();
    }

    let canonical_request_signed_headers = request.headers().iter().map(|(header, _)| header.as_str()).collect::<Vec<_>>().join(";");

    let mut canonical_request_headers = String::new();
    // TODO: sort this by header name
    // TODO: only get Host, Content-Type, and x-amz-* headers
    for (header, value) in request.headers().iter() {
        // TODO: trim value
        canonical_request_headers += format!("{}:{}\n", header, value.to_str()?).as_str();
    }

    let canonical_request_hashed_payload = request.headers().get("x-amz-content-sha256").unwrap().to_str()?;

    let canonical_request = format!(
        "{}\n{}\n{}\n{}\n{}\n{}",
        request.method(),
        request.url().path(),
        canonical_request_query,
        canonical_request_headers,
        canonical_request_signed_headers,
        canonical_request_hashed_payload,
    );
    dbg!(&canonical_request);

    let algo = "AWS4-HMAC-SHA256";

    // TODO: share this date with add_amz_headers
    let date = Utc::now();
    let date = date.format("%Y%m%d").to_string();
    let region = "us-east-1";
    let service = "s3";
    let aws4_request = "aws4_request";

    let canonical_request_sha256 = Sha256::digest(canonical_request);
    let canonical_request_sha256 = format!("{:x}", canonical_request_sha256);

    let scope = format!("{}/{}/{}/{}", date, region, service, aws4_request);

    let string_to_sign = format!(
        "{}\n{}\n{}\n{}",
        &algo,
        &request.headers().get("x-amz-date").unwrap().to_str()?,
        &scope,
        canonical_request_sha256,
    );
    dbg!(&string_to_sign);


    let mut date_key = HmacSha256::new_from_slice(format!("AWS4{}", creds.secret_key).as_bytes())?;
    date_key.update(&date.as_bytes());
    let date_key = date_key.finalize().into_bytes();

    let mut date_region_key = HmacSha256::new_from_slice(&date_key[..])?;
    date_region_key.update(&region.as_bytes());
    let date_region_key = date_region_key.finalize().into_bytes();

    let mut date_region_service_key = HmacSha256::new_from_slice(&date_region_key[..])?;
    date_region_service_key.update(&service.as_bytes());
    let date_region_service_key = date_region_service_key.finalize().into_bytes();

    let mut signing_key = HmacSha256::new_from_slice(&date_region_service_key[..])?;
    signing_key.update(&aws4_request.as_bytes());
    let signing_key = signing_key.finalize().into_bytes();

    let signing_key = format!("{:x}", signing_key);
    dbg!(&signing_key);

    let mut signature = HmacSha256::new_from_slice(&signing_key.as_bytes())?;
    signature.update(&string_to_sign.as_bytes());
    let signature = signature.finalize().into_bytes();

    let signature = format!("{:x}", signature);
    dbg!(&signature);

    let authorization = format!(
        "{} Credential={}/{}, SignedHeaders={}, Signature={}",
        &algo,
        &creds.access_key,
        &scope,
        &canonical_request_signed_headers,
        &signature,
    );
    dbg!(&authorization);

    let headers = request.headers_mut();
    headers.insert(reqwest::header::AUTHORIZATION, reqwest::header::HeaderValue::from_str(authorization.as_str())?);

    Ok(())
}
