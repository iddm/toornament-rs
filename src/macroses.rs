macro_rules! builder {
    ($field:ident, $field_type:ty) => {
        /// A builder method for $field with `$field_type` type.
        pub fn $field(mut self, $field: $field_type) -> Self {
            self.$field = $field;
            self
        }
    };
}

macro_rules! builder_s {
    ($field:ident) => {
        /// A builder method for $field with `String` type.
        pub fn $field<S: Into<String>>(mut self, $field: S) -> Self {
            self.$field = $field.into();
            self
        }
    };
}

macro_rules! builder_o {
    ($field:ident, $field_type:ty) => {
        /// A builder method for $field with `Option` type.
        pub fn $field(mut self, $field: $field_type) -> Self {
            self.$field = Some($field.into());
            self
        }
    };
}

macro_rules! builder_so {
    ($field:ident) => {
        builder!($field, Option<String>);
    };
}
