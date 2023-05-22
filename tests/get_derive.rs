use std::cell::OnceCell;

use cell_derive::GetDerive;

#[test]
fn test_get_derive() {
    #[derive(GetDerive)]
    struct TestStruct {
        #[init(init_val)]
        val: OnceCell<String>,
    }

    fn init_val() -> String {
        "Value".to_string()
    }

    let test = TestStruct {
        val: OnceCell::new(),
    };

    assert_eq!(test.val(), &"Value".to_string())
}
