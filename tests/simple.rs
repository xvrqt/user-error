use user_error::UserError;

#[test]
fn simple_constructor() {
    let s  = "Too gay to function";
    let _ue = UserError::simple(s);
    // Strip color control characters
 //   let summary = ue.summary().replace(|c: char| !c.is_ascii(), "");
  //  assert_eq!(String::from(s), summary);
}
