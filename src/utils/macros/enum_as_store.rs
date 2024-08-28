#[macro_export]
macro_rules! enum_as_store {
    (
        $(
            $(#[ $($meta:meta),+ ])*
            $name:ident: $kind:ty {
                $( $key:ident => $val:expr ),+
            }
        ),* 
    ) => {
        $(
            $(#[ $($meta),+ ])*
            pub enum $name {
                $( $key ),+
            }
        )*

        $(
            impl $name {
                pub fn value(&self) -> $kind {
                    match self {
                        $( $name::$key => $val ),+
                    }
                }
            }
        )*
    };
}