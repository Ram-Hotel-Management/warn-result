use warn_result::{WOK, WarnResult};
fn _test_compilation() -> WarnResult<(), String, String> {
    let warning: WarnResult<(), &'static str, String> =
        warn_result::WarnResult::Warning("This is a warning");
    let res = warn_result::warn_try_sc_error!(warning);

    let _: () = res.unwrap_or_default();
    WarnResult::Ok(())
}

fn _test_compilation2() -> Result<WOK<(), &'static str>, String> {
    let warning: WarnResult<(), &'static str, String> =
        warn_result::WarnResult::Warning("This is another warning");
    let res = warn_result::warn_try_sc_error_result!(warning);

    let a = res?;
    Ok(a)
}
