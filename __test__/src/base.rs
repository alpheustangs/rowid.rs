use std::{io, time::SystemTime};

use rowid::{
    base::{
        GenerateResult, RowIDError, VerifyResult, decode, encode, generate,
        get_randomness, rowid, verify,
    },
    time::{system_time_to_timestamp, timestamp_to_system_time},
};

// get_randomness

#[test]
fn test_get_randomness_0() {
    let rds: String = get_randomness(0);
    assert!(rds.len() == 0);
}

#[test]
fn test_get_randomness_6() {
    let rds: String = get_randomness(6);
    assert!(rds.len() == 6);
}

#[test]
fn test_get_randomness_10() {
    let rds: String = get_randomness(10);
    assert!(rds.len() == 10);
}

#[test]
fn test_get_randomness_20() {
    let rds: String = get_randomness(20);
    assert!(rds.len() == 20);
}

// encode

#[test]
fn test_encode_now() {
    let encoded: String = encode(SystemTime::now()).unwrap();
    assert!(encoded.len() == 10);
}

#[test]
fn test_encode_0() {
    let encoded: String = encode(timestamp_to_system_time(0)).unwrap();
    assert!(encoded == "0000000000");
}

// rowid

#[test]
fn test_rowid() {
    let id: String = rowid();
    assert!(id.len() == 32);
}

// decode

#[test]
fn test_decode() {
    let current: SystemTime = SystemTime::now();
    let decoded: SystemTime = decode(&encode(current).unwrap()).unwrap();
    assert!(
        system_time_to_timestamp(decoded) == system_time_to_timestamp(current)
    );
}

#[test]
fn test_decode_length_error() {
    let result: io::Error = match decode("ABC123") {
        | Ok(_) => return assert!(false),
        | Err(e) => e,
    };

    assert!(result.kind() == io::ErrorKind::InvalidInput);
    assert!(result.to_string() == RowIDError::EncodedLength.as_str());
}

#[test]
fn test_decode_invalid_input_error() {
    let result: io::Error = match decode("ab^!@#$agastgyaSER") {
        | Ok(_) => return assert!(false),
        | Err(e) => e,
    };

    assert!(result.kind() == io::ErrorKind::InvalidInput);
    assert!(result.to_string() == RowIDError::InvalidEncoded.as_str());
}

// generate

#[test]
fn test_generate() {
    let current: SystemTime = SystemTime::now();
    let generated: GenerateResult = generate(current, Some(6));
    let id: String = generated.result.unwrap();
    assert!(generated.success == true);
    assert!(
        system_time_to_timestamp(decode(&id).unwrap())
            == system_time_to_timestamp(current)
    );
    assert!(id.len() == 16);
}

// verify

#[test]
fn test_verify() {
    let current: SystemTime = SystemTime::now();
    let id: String = generate(current, Some(6)).result.unwrap();
    let verified: VerifyResult = verify(&id);
    assert!(verified.success == true);
    assert!(match verified.result {
        | Some(r) =>
            system_time_to_timestamp(r) == system_time_to_timestamp(current),
        | None => false,
    });
    assert!(verified.natural == Some(true));
}

#[test]
fn test_verify_length_error() {
    let verified: VerifyResult = verify("ABC123");

    assert!(verified.success == false);

    let error: io::Error = match verified.error {
        | None => return assert!(false),
        | Some(e) => e,
    };

    assert!(error.kind() == io::ErrorKind::InvalidInput);
    assert!(error.to_string() == RowIDError::EncodedLength.as_str());
}

#[test]
fn test_verify_invalid_input_error() {
    let verified: VerifyResult = verify("ab^!@#$agastgyaSER");

    assert!(verified.success == false);

    let error: io::Error = match verified.error {
        | None => return assert!(false),
        | Some(e) => e,
    };

    assert!(error.kind() == io::ErrorKind::InvalidInput);
    assert!(error.to_string() == RowIDError::InvalidEncoded.as_str());
}
