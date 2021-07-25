use candid::candid_method;
use ic_cdk::{api, export::Principal};
use ic_cdk_macros::*;

static mut TEST_VALUE: u64 = 0;
static mut OWNER: Principal = Principal::anonymous();
static mut STORAGE_CANISTER_ID: Principal = Principal::anonymous();

#[init]
#[candid_method(init)]
fn init() {
    unsafe {
        OWNER = api::caller();
    }
}

#[update(name = "raiseError")]
#[candid_method(update, rename = "raiseError")]
async fn raise_error(err_msg: String) -> bool {
    unsafe {
        assert!(
            STORAGE_CANISTER_ID != Principal::anonymous(),
            "storage canister not set"
        );

        TEST_VALUE += 1;
        let caller = api::caller();
        let save_res: Result<(bool,), _> =
            ic_cdk::api::call::call(STORAGE_CANISTER_ID, "saveError", (caller, err_msg)).await;
        match save_res {
            Ok(_) => ic_cdk::print("save_error storage executed succeed"),
            _ => ic_cdk::print("save_error storage executed failed"),
        };
    }

    assert!(false, "raise error");
    true
}

#[update(name = "setStorage")]
#[candid_method(update, rename = "setStorage")]
fn set_storage(storage_canister_id: Principal) -> bool {
    unsafe {
        ic_cdk::print(format!("OWNER IS {}", OWNER));
        ic_cdk::print(format!("CALLER IS {}", api::caller()));
    }
    if !is_owner() {
        return false;
    }

    unsafe {
        STORAGE_CANISTER_ID = storage_canister_id;
    }
    true
}

#[query(name = "getLastError")]
#[candid_method(query, rename = "getLastError")]
async fn get_last_error() -> String {
    unsafe {
        assert!(
            STORAGE_CANISTER_ID != Principal::anonymous(),
            "storage canister not set"
        );
        let caller = api::caller();
        let get_res: Result<(String,), _> =
            ic_cdk::api::call::call(STORAGE_CANISTER_ID, "getLastError", (caller,)).await;
        match get_res {
            Ok(msg) => msg.0,
            _ => "".to_string(),
        }
    }
}

fn is_owner() -> bool {
    let caller = api::caller();
    unsafe { caller == OWNER }
}
