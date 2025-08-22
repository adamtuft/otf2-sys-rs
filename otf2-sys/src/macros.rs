macro_rules! zipmap {
    ($first:expr, $second:expr, $closure:expr) => {
        $first
            .zip($second)
            .map($closure)
            .collect()
    };
}

macro_rules! slice_from_raw {
    ($ptr:expr, $size:expr) => {
        std::slice::from_raw_parts($ptr, $size as usize)
    }
}

macro_rules! parse_ident_or_underscore {
    ( _ ) => {
        _
    };
    ( $ident:ident ) => {
        $ident
    };
}

macro_rules! map_optional_union_access {
    ($variant_name:ident from $union_field:expr ) => {
        $variant_name(unsafe { $union_field })
    };
    ($variant_name:ident) => {
        $variant_name(())
    };
}

macro_rules! declare_enum_union_wrapper {
    (
        $(#[$enum_attr:meta])*
        $vis:vis enum $enum_name:ident (union: $mapped_union_newtype:ty) {
            $(
                $(#[$variant_attr:meta])*
                $type_variant:tt => $variant_name:ident ( $contained_type:ty ) $( from $union_field:ident )?
            ),* $(,)?
        }
    ) => {
        $(#[$enum_attr])*
        $vis enum $enum_name {
            $(
                $(#[$variant_attr])*
                $variant_name ( $contained_type ),
            )*
        }

        impl $enum_name {
            pub fn new(kind: OTF2_Type, value: $mapped_union_newtype) -> Self {
                use OTF2_Type_enum::*;
                use $enum_name::*;
                match kind.to_enum() {
                    $(
                        crate::internal::parse_ident_or_underscore!($type_variant) => crate::internal::map_optional_union_access!($variant_name $(from value.0.$union_field)?),
                    )*
                }
            }

            pub fn type_name(&self) -> &'static str {
                match self {
                    $(
                        Self::$variant_name(_) => stringify!($contained_type),
                    )*
                }
            }
        }

        $(
            impl TryFrom<$enum_name> for $contained_type {
                type Error = String;

                fn try_from(value: $enum_name) -> Result<Self, Self::Error> {
                    use $enum_name::*;
                    if let $variant_name(v) = value {
                        Ok(v)
                    } else {
                        Err(format!("{} expected {} but got {}", stringify!(TryFrom<$enum_name>), stringify!($contained_type), value.type_name()))
                    }
                }
            }
        )*
    };
}

pub(crate) use {
    zipmap, slice_from_raw, parse_ident_or_underscore, map_optional_union_access, declare_enum_union_wrapper
};
