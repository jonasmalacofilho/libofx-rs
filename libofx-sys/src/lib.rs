extern crate libc;

use libc::{c_char, c_double, c_int, c_long, c_void, time_t};

#[link(name = "ofx")]
extern "C" {
    /// May try to unwind from C++.
    pub fn libofx_get_new_context() -> LibofxContextPtr;

    /// May try to unwind from C++.
    pub fn libofx_free_context(libofx_context: LibofxContextPtr) -> c_int;

    pub fn libofx_set_dtd_dir(libofx_context: LibofxContextPtr, s: *const c_char) -> ();

    // TODO libofx_get_file_format_from_str
    // TODO libofx_get_file_format_description

    /// May try to unwind from C++.
    pub fn libofx_proc_file(
        libofx_context: LibofxContextPtr,
        p_filename: *const c_char,
        ftype: LibofxFileFormat,
    ) -> c_int;

    // libofx_proc_buffer hidden, has no implementation (libofx/libofx#34)

    pub fn ofx_set_status_cb(
        libofx_context: LibofxContextPtr,
        cb: LibofxProcStatusCallback,
        user_data: *mut c_void,
    ) -> ();

    pub fn ofx_set_account_cb(
        libofx_context: LibofxContextPtr,
        cb: LibofxProcAccountCallback,
        user_data: *mut c_void,
    ) -> ();

    pub fn ofx_set_security_cb(
        libofx_context: LibofxContextPtr,
        cb: LibofxProcSecurityCallback,
        user_data: *mut c_void,
    ) -> ();

    pub fn ofx_set_transaction_cb(
        libofx_context: LibofxContextPtr,
        cb: LibofxProcTransactionCallback,
        user_data: *mut c_void,
    ) -> ();

    pub fn ofx_set_statement_cb(
        libofx_context: LibofxContextPtr,
        cb: LibofxProcStatementCallback,
        user_data: *mut c_void,
    ) -> ();

    // TODO libofx_request_statement
    // TODO libofx_request_accountinfo
    // TODO libofx_request_payment
    // TODO libofx_request_payment_status

    pub static mut ofx_PARSER_msg: c_int;
    pub static mut ofx_DEBUG_msg: c_int;
    pub static mut ofx_DEBUG1_msg: c_int;
    pub static mut ofx_DEBUG2_msg: c_int;
    pub static mut ofx_DEBUG3_msg: c_int;
    pub static mut ofx_DEBUG4_msg: c_int;
    pub static mut ofx_DEBUG5_msg: c_int;
    pub static mut ofx_STATUS_msg: c_int;
    pub static mut ofx_INFO_msg: c_int;
    pub static mut ofx_WARNING_msg: c_int;
    pub static mut ofx_ERROR_msg: c_int;
    pub static mut ofx_show_position: c_int;
}

pub type LibofxContextPtr = *mut c_void;
pub type LibofxProcStatusCallback = extern "C" fn(OfxStatusData, *mut c_void) -> c_int;
pub type LibofxProcAccountCallback = extern "C" fn(OfxAccountData, *mut c_void) -> c_int;
pub type LibofxProcSecurityCallback = extern "C" fn(OfxSecurityData, *mut c_void) -> c_int;
pub type LibofxProcTransactionCallback = extern "C" fn(OfxTransactionData, *mut c_void) -> c_int;
pub type LibofxProcStatementCallback = extern "C" fn(OfxStatementData, *mut c_void) -> c_int;

#[repr(C)]
pub enum LibofxFileFormat {
    AUTODETECT,
    OFX,
    OFC,
    QIF,
    UNKNOWN,
    LAST,
}

#[repr(C)]
pub struct LibofxFileFormatInfo {
    format: LibofxFileFormat,
    format_name: *const c_char,
    description: *const c_char,
}

// TODO LibofxImportFormatList
// TODO LibofxExportFormatList

#[repr(C)]
pub enum Severity {
    INFO,
    WARN,
    ERROR,
}

#[repr(C)]
pub struct OfxStatusData {
    pub ofx_element_name: [c_char; OFX_ELEMENT_NAME_LENGTH],
    pub ofx_element_name_valid: c_int,
    pub code: c_int,
    pub name: *const c_char,
    pub description: *const c_char,
    pub code_valid: c_int,
    pub severity: Severity,
    pub severity_valid: c_int,
    pub server_message: *mut c_char,
    pub server_message_valid: c_int,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub enum AccountType {
    OFX_CHECKING,
    OFX_SAVINGS,
    OFX_MONEYMRKT,
    OFX_CREDITLINE,
    OFX_CMA,
    OFX_CREDITCARD,
    OFX_INVESTMENT,
}

#[repr(C)]
pub struct OfxAccountData {
    pub account_id: [c_char; OFX_ACCOUNT_ID_LENGTH],
    pub account_name: [c_char; OFX_ACCOUNT_NAME_LENGTH],
    pub account_id_valid: c_int,
    pub account_type: AccountType,
    pub account_type_valid: c_int,
    pub currency: [c_char; OFX_CURRENCY_LENGTH],
    pub currency_valid: c_int,
    pub account_number: [c_char; OFX_ACCTID_LENGTH],
    pub account_number_valid: c_int,
    pub bank_id: [c_char; OFX_BANKID_LENGTH],
    pub bank_id_valid: c_int,
    pub broker_id: [c_char; OFX_BROKERID_LENGTH],
    pub broker_id_valid: c_int,
    pub branch_id: [c_char; OFX_BRANCHID_LENGTH],
    pub branch_id_valid: c_int,
}

#[repr(C)]
pub struct OfxSecurityData {
    pub unique_id: [c_char; OFX_UNIQUE_ID_LENGTH],
    pub unique_id_valid: c_int,
    pub unique_id_type: [c_char; OFX_UNIQUE_ID_TYPE_LENGTH],
    pub unique_id_type_valid: c_int,
    pub secname: [c_char; OFX_SECNAME_LENGTH],
    pub secname_valid: c_int,
    pub ticker: [c_char; OFX_TICKER_LENGTH],
    pub ticker_valid: c_int,
    pub unitprice: c_double,
    pub unitprice_valid: c_int,
    pub date_unitprice: time_t,
    pub date_unitprice_valid: c_int,
    pub currency: [c_char; OFX_CURRENCY_LENGTH],
    pub currency_valid: c_int,
    pub memo: [c_char; OFX_MEMO2_LENGTH],
    pub memo_valid: c_int,
    pub fiid: [c_char; OFX_FIID_LENGTH],
    pub fiid_valid: c_int,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub enum TransactionType {
    OFX_CREDIT,
    OFX_DEBIT,
    OFX_INT,
    OFX_DIV,
    OFX_FEE,
    OFX_SRVCHG,
    OFX_DEP,
    OFX_ATM,
    OFX_POS,
    OFX_XFER,
    OFX_CHECK,
    OFX_PAYMENT,
    OFX_CASH,
    OFX_DIRECTDEP,
    OFX_DIRECTDEBIT,
    OFX_REPEATPMT,
    OFX_OTHER,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub enum InvTransactionType {
    OFX_BUYDEBT,
    OFX_BUYMF,
    OFX_BUYOPT,
    OFX_BUYOTHER,
    OFX_BUYSTOCK,
    OFX_CLOSUREOPT,
    OFX_INCOME,
    OFX_INVEXPENSE,
    OFX_JRNLFUND,
    OFX_JRNLSEC,
    OFX_MARGININTEREST,
    OFX_REINVEST,
    OFX_RETOFCAP,
    OFX_SELLDEBT,
    OFX_SELLMF,
    OFX_SELLOPT,
    OFX_SELLOTHER,
    OFX_SELLSTOCK,
    OFX_SPLIT,
    OFX_TRANSFER,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub enum FiIdCorrectionAction {
    DELETE,
    REPLACE,
}

#[repr(C)]
pub struct OfxTransactionData {
    pub account_id: [c_char; OFX_ACCOUNT_ID_LENGTH],
    pub account_ptr: *mut OfxAccountData,
    pub account_id_valid: c_int,
    pub transactiontype: TransactionType,
    pub transactiontype_valid: c_int,
    pub invtransactiontype: InvTransactionType,
    pub invtransactiontype_valid: c_int,
    pub units: c_double,
    pub units_valid: c_int,
    pub unitprice: c_double,
    pub unitprice_valid: c_int,
    pub amount: c_double,
    pub amount_valid: c_int,
    pub fi_id: [c_char; 256],
    pub fi_id_valid: c_int,
    pub unique_id: [c_char; OFX_UNIQUE_ID_LENGTH],
    pub unique_id_valid: c_int,
    pub unique_id_type: [c_char; OFX_UNIQUE_ID_TYPE_LENGTH],
    pub unique_id_type_valid: c_int,
    pub security_data_ptr: *mut OfxSecurityData,
    pub security_data_valid: c_int,
    pub date_posted: time_t,
    pub date_posted_valid: c_int,
    pub data_initiated: time_t,
    pub data_initiated_valid: c_int,
    pub date_funds_available: time_t,
    pub date_funds_available_valid: c_int,
    pub fi_id_corrected: [c_char; 256],
    pub fi_id_corrected_valid: c_int,
    pub fi_id_correction_action: FiIdCorrectionAction,
    pub fi_id_correction_action_valid: c_int,
    pub server_transaction_id: [c_char; OFX_SVRTID2_LENGTH],
    pub server_transaction_id_valid: c_int,
    pub check_number: [c_char; OFX_CHECK_NUMBER_LENGTH],
    pub check_number_valid: c_int,
    pub reference_number: [c_char; OFX_REFERENCE_NUMBER_LENGTH],
    pub reference_number_valid: c_int,
    pub standard_industrial_code: c_long,
    pub standard_industrial_code_valid: c_int,
    pub payee_id: [c_char; OFX_SVRTID2_LENGTH],
    pub payee_id_valid: c_int,
    pub name: [c_char; OFX_TRANSACTION_NAME_LENGTH],
    pub name_valid: c_int,
    pub memo: [c_char; OFX_MEMO2_LENGTH],
    pub memo_valid: c_int,
    pub comission: c_double,
    pub comission_valid: c_int,
    pub fees: c_double,
    pub fees_valid: c_int,
    pub oldunits: c_double,
    pub oldunits_valid: c_int,
    pub newunits: c_double,
    pub newunits_valid: c_int,
}

#[repr(C)]
pub struct OfxStatementData {
    pub currency: [c_char; OFX_CURRENCY_LENGTH],
    pub currency_valid: c_int,
    pub account_id: [c_char; OFX_ACCOUNT_ID_LENGTH],
    pub account_ptr: *mut OfxAccountData,
    pub account_id_valid: c_int,
    pub ledger_balance: c_double,
    pub ledger_balance_valid: c_int,
    pub ledger_balance_date: time_t,
    pub ledger_balance_date_valid: c_int,
    pub available_balance: c_double,
    pub available_balance_valid: c_int,
    pub available_balance_date: time_t,
    pub available_balance_date_valid: c_int,
    pub date_start: time_t,
    pub date_start_valid: c_int,
    pub date_end: time_t,
    pub date_end_valid: c_int,
    pub marketing_info: [c_char; OFX_MARKETING_INFO_LENGTH],
    pub marketing_info_valid: c_int,
}

// TODO OfxFiServiceInfo (unused)
// TODO OfxfiLogin
// TODO OfxPayment
// TODO OfxPayee

pub const OFX_ELEMENT_NAME_LENGTH: usize = 100;
pub const OFX_SVRTID2_LENGTH: usize = 36 + 1;
pub const OFX_CHECK_NUMBER_LENGTH: usize = 12 + 1;
pub const OFX_REFERENCE_NUMBER_LENGTH: usize = 32 + 1;
pub const OFX_FITID_LENGTH: usize = 255 + 1;
pub const OFX_TOKEN2_LENGTH: usize = 36 + 1;
pub const OFX_MEMO_LENGTH: usize = 255 + 1;
pub const OFX_FIID_LENGTH: usize = 32 + 1;
pub const OFX_MEMO2_LENGTH: usize = 390 + 1;
pub const OFX_BALANCE_NAME_LENGTH: usize = 32 + 1;
pub const OFX_BALANCE_DESCRIPTION_LENGTH: usize = 80 + 1;
pub const OFX_CURRENCY_LENGTH: usize = 3 + 1;
pub const OFX_BANKID_LENGTH: usize = 9 + 1;
pub const OFX_BRANCHID_LENGTH: usize = 22 + 1;
pub const OFX_ACCTID_LENGTH: usize = 22 + 1;
pub const OFX_ACCTKEY_LENGTH: usize = 22 + 1;
pub const OFX_BROKERID_LENGTH: usize = 22 + 1;
pub const OFX_ACCOUNT_ID_LENGTH: usize =
    OFX_BANKID_LENGTH + OFX_BRANCHID_LENGTH + OFX_ACCTID_LENGTH + 1;
pub const OFX_ACCOUNT_NAME_LENGTH: usize = 255;
pub const OFX_MARKETING_INFO_LENGTH: usize = 360 + 1;
pub const OFX_TRANSACTION_NAME_LENGTH: usize = 96 + 1;
pub const OFX_UNIQUE_ID_LENGTH: usize = 32 + 1;
pub const OFX_UNIQUE_ID_TYPE_LENGTH: usize = 10 + 1;
pub const OFX_SECNAME_LENGTH: usize = 32 + 1;
pub const OFX_TICKER_LENGTH: usize = 32 + 1;
pub const OFX_ORG_LENGTH: usize = 32 + 1;
pub const OFX_FID_LENGTH: usize = 32 + 1;
pub const OFX_USERID_LENGTH: usize = 32 + 1;
pub const OFX_USERPASS_LENGTH: usize = 32 + 1;
pub const OFX_URL_LENGTH: usize = 500 + 1;
pub const OFX_APPID_LENGTH: usize = 32;
pub const OFX_APPVER_LENGTH: usize = 32;
pub const OFX_HEADERVERSION_LENGTH: usize = 32;
pub const OFX_CLIENTUID_LENGTH: usize = 36 + 1;
pub const OFX_AMOUNT_LENGTH: usize = 32 + 1;
pub const OFX_PAYACCT_LENGTH: usize = 32 + 1;
pub const OFX_STATE_LENGTH: usize = 5 + 1;
pub const OFX_POSTALCODE_LENGTH: usize = 11 + 1;
pub const OFX_NAME_LENGTH: usize = 32 + 1;

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn it_works() {
        let sample = CString::new("../docs/samples/santander.ofx").unwrap();
        unsafe {
            ofx_show_position = 0;
            ofx_ERROR_msg = 1;
            ofx_INFO_msg = 1;
            let ctx = libofx_get_new_context();
            let ret = libofx_proc_file(ctx, sample.as_ptr(), LibofxFileFormat::AUTODETECT);
            libofx_free_context(ctx);
            assert_eq!(ret, 0);
        }
    }
}
