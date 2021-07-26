use candid::candid_method;
use chrono::prelude::*;
use ic_cdk::{api, export::Principal, storage};
use ic_cdk_macros::*;
use std::collections::HashMap;

static mut TEST_VALUE: u64 = 0;
static mut TEST_VALUE_2: u64 = 0;
static mut OWNER: Principal = Principal::anonymous();

// timestamp call_principal,error_msg
type Errors = HashMap<u64, HashMap<Principal, String>>;
// The error expires in a 5 minutes
const ERROR_EXPIRED_IN_MINUTE: u32 = 5u32;

#[init]
#[candid_method(init)]
fn init() {
    unsafe {
        OWNER = api::caller();
    }
}

#[update(name = "raiseError")]
#[candid_method(update, rename = "raiseError")]
async fn test_raise_error(err_msg: String) -> bool {
    unsafe {
        TEST_VALUE += 1;
        let caller = api::caller();
        // MUST use ic_cdk::api::call::call(api::id(), "__save_error_hack", (caller, err_msg)) to save error
        // If call  __save_error_hack(caller, err_msg) directly, the stored message will be reverted

        let save_res: Result<(bool,), _> =
            ic_cdk::api::call::call(api::id(), "__save_error_hack", (caller, err_msg)).await;
        match save_res {
            Ok(_) => ic_cdk::print("save_error storage executed succeed"),
            _ => ic_cdk::print("save_error storage executed failed"),
        };

        TEST_VALUE_2 += 1;
    }

    assert!(false, "raise error");
    true
}

// must impl it for save error message
#[update(name = "__save_error_hack")]
#[candid_method(update, rename = "__save_error_hack")]
fn __save_error_hack(_caller: Principal, _err_msg: String) -> bool {
    let caller = api::caller();

    assert!(caller == api::id(), "can not call it out side");

    ic_cdk::print("save_error executing");
    let crt_time = api::time();
    let errors_read = storage::get::<Errors>();
    let storage_timestamp = get_storage_timestamp(crt_time);

    match errors_read.get(&storage_timestamp) {
        Some(inner) => {
            let mut temp = inner.clone();
            temp.insert(_caller, _err_msg);
            let errors = storage::get_mut::<Errors>();
            errors.insert(storage_timestamp, temp);
        }
        None => {
            let mut inner = HashMap::new();
            inner.insert(_caller, _err_msg);
            let errors = storage::get_mut::<Errors>();
            errors.insert(storage_timestamp, inner);
        }
    }
    clear_expired_error();
    true
}

#[query(name = "getLastError")]
#[candid_method(query, rename = "getLastError")]
fn get_last_error() -> String {
    let crt_time = api::time();
    let caller = api::caller();
    let crt_storage_timestamp = get_storage_timestamp(crt_time);
    let errors_read = storage::get::<Errors>();

    let crt_storge = errors_read.get(&crt_storage_timestamp);
    match crt_storge {
        Some(inner) => match inner.get(&caller) {
            Some(err) => err.to_string(),
            _ => "".to_string(),
        },
        _ => "".to_string(),
    }
}

#[query(name = "getValue")]
#[candid_method(query, rename = "getValue")]
async fn get_value() -> u64 {
    unsafe { TEST_VALUE }
}

#[query(name = "getValue2")]
#[candid_method(query, rename = "getValue2")]
async fn get_value2() -> u64 {
    unsafe { TEST_VALUE_2 }
}

fn get_storage_timestamp(crt_time: u64) -> u64 {
    let crt_datetime: DateTime<Utc> = Utc.timestamp_nanos(crt_time as i64);
    let naive_utc = crt_datetime.naive_utc();
    let naive_date = naive_utc.date();
    let naive_time = naive_utc.time();
    let naive_minute = naive_time.minute();
    let start_minute = (naive_minute / ERROR_EXPIRED_IN_MINUTE) * ERROR_EXPIRED_IN_MINUTE;
    let start_dt = Utc
        .ymd(naive_date.year(), naive_date.month(), naive_date.day())
        .and_hms(naive_time.hour(), start_minute, 0);
    start_dt.timestamp() as u64
}

fn clear_expired_error() {
    let crt_time = api::time();
    let crt_storage_timestamp = get_storage_timestamp(crt_time);
    let errors = storage::get_mut::<Errors>();
    errors.retain(|key, _| key >= &crt_storage_timestamp);
}
