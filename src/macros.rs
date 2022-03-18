#[macro_export]
macro_rules! declare_env {
    ($env:ident: $env_var:ident, $def:expr, $type:ty) => {
        pub fn $env() -> $type {
            match env::var("TDT_$env_var") {
                Ok(res) => res.into(),
                _ => $def.into()
            }
        }
    };
    ($env:ident: $env_var:ident, $def:expr) => {
        pub fn $env() -> String {
            match env::var("TDT_$env_var") {
                Ok(res) => res,
                _ => $def.to_string()
            }
        }
    }
}
#[macro_export]
macro_rules! define_container {
    (@add_type $type:ty => $enum:ident::$variant:ident) => {
        impl From<$type> for $enum {
            fn from(v: $type) -> Self {
                Self::$variant(v)
            }
        }

        impl From<$enum> for $type {
            fn from(v: $enum) -> Self {
                match v {
                    $enum::$variant(r) => r,
                    _ => panic!("Expected $enum::$variant, got {:#?}", v),
                }
            }
        }

        // I'm not sure if it's possible to just do all of this as referrences
        impl From<&$type> for $enum {
            fn from(v: &$type) -> Self {
                Self::$variant(v.clone())
            }
        }

        impl From<&$enum> for $type {
            fn from(v: &$enum) -> Self {
                match v {
                    $enum::$variant(r) => r.clone(),
                    _ => panic!("Expected $enum::$variant, got {:#?}", v),
                }
            }
        }
    };
    ($enum:ident: {$($variant:ident: $type:ty),+ $(,)?}) => {

        #[derive(Debug)]
        pub enum $enum {
            $($variant($type),)*
        }
        
        $(define_container!(@add_type $type => $enum::$variant);)*
    }
}
