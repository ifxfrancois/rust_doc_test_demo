
/// check if the add function returns the correct value when both input parameters are 2
///
/// # test steps
///
/// * call add with 2 and 2
/// * check result
/// * ...
///
/// requirement (req_math_add_ID)[link to req.]
#[test]
fn it_works() {
    let result = rust_doc_test_demo::math::add(2, 2);
    assert_eq!(result, 4);
}
