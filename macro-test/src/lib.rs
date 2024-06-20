use proc_macro::my_macro;

#[my_macro]
pub fn my_func() {
    let my_field = String::default();
    let _this_is_fine = MyEnum::MyVariant { my_field };
    let my_var = String::default();
    let _this_produces_a_parse_error = MyEnum::MyVariant { my_field: my_var };
}

#[allow(dead_code)]
enum MyEnum {
    MyVariant { my_field: String },
}
