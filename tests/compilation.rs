use warn_result::WarnResult;
fn _test_compilation() -> WarnResult<(), String, String> {
    let warning: WarnResult<(), &'static str, String> =
        warn_result::WarnResult::Warning("This is a warning");
    let res = warn_result::warn_try_sc_error!(warning);

    let _: () = res.unwrap_or_default();
    WarnResult::Ok(())
}
