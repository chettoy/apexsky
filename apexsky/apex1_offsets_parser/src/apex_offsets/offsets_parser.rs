macro_rules! def_offsets {
    ($name:ident: [$section_name:expr] {$($field:ident: $alias:expr,)*}) => {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub(super) struct $name {
            $(
                pub(super) $field: String
            ),*
        }

        impl TryFrom<&ini::Properties> for $name {
            type Error = anyhow::Error;
            fn try_from(value: &ini::Properties) -> Result<Self, Self::Error> {
                Ok(Self {
                    $(
                        $field: Self::read_value(value, obfstr::obfstr!($alias))?
                    ),*
                })
            }
        }

        impl $name {
            fn read_value(section: &ini::Properties, key: &str) -> anyhow::Result<String> {
                section.get(key).map(|v| v.to_string()).ok_or_else(||{
                    // println!("{:?}", section);
                    anyhow::anyhow!(format!(
                        "{}{}{}{}{}",
                        obfstr::obfstr!("Failed to read value `"),
                        key,
                        obfstr::obfstr!("` in section `"),
                        obfstr::obfstr!($section_name),
                        obfstr::obfstr!("` in offsets file.")
                    ))
                })
            }

            pub(super) fn from_ini(conf: &ini::Ini) -> anyhow::Result<Self> {
                conf.section_all(Some(obfstr::obfstr!($section_name))).fold(
                    Err(anyhow::anyhow!(format!(
                        "{}{}{}",
                        obfstr::obfstr!("Failed to read section `"),
                        obfstr::obfstr!($section_name),
                        obfstr::obfstr!("` in offsets file.")
                    ))),
                    |acc, x| match acc {
                        Ok(_) => acc,
                        Err(_) => x.try_into(),
                    },
                )
            }
        }
    }
}

macro_rules! export_custom_offsets {
    (~$data_type:ident~(^_^) ~$target_type:ident(~_~)=> $ref_name:ident{$($field:ident: $value:expr,)*}) => {
        paste::paste! {
            impl From<$data_type> for $target_type {
                fn from($ref_name: $data_type) -> Self {
                    Self {
                        $(
                            $field: ({ $value }),
                        )*
                    }
                }
            }
        }
    }
}

macro_rules! parse_u64 {
    ($value:expr) => {
        if ($value).starts_with(obfstr::obfstr!("0x")) {
            u64::from_str_radix(($value).trim_start_matches(obfstr::obfstr!("0x")), 16).unwrap()
        } else {
            ($value).parse::<u64>().unwrap()
        }
    };
}

macro_rules! assert_offsets_eq {
    ($a:expr,$b:expr) => {
        if ($a) != ($b) {
            // panic with name and value
            assert_eq!(
                format!("{}=0x{:x}={}", stringify!($a), ($a), ($a)),
                format!("{}=0x{:x}={}", stringify!($b), ($b), ($b))
            );
        }
    };
}

pub(super) use assert_offsets_eq;
pub(super) use def_offsets;
pub(super) use export_custom_offsets;
pub(super) use parse_u64;
