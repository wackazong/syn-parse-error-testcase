use proc_macro::my_macro;

#[my_macro]
pub fn my_func() {
    MyEnum::MyVariant {
        my_field: "".into(),
    };
}

enum MyEnum {
    MyVariant { my_field: String },
}
