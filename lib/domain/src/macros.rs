macro_rules! get {
    ($name:ident, $type:ty) => {
        paste::paste! {
            #[allow(dead_code)]
            pub fn [<get_ $name>](&self) -> $type {
                self.$name.clone()
            }
        }
    };
}

macro_rules! currency {
    ($code:literal, $name:literal, $symbol:literal) => {
        paste::paste! {
            // TODO: does inlining affect static?

            #[inline]
            #[allow(non_snake_case)]
            pub fn [<$code:upper>]() -> &'static Self {
                static [< $code:upper _CURRENCY>]: Lazy<Currency>
                    = Lazy::new(|| Currency::new($code, $name, $symbol));

                &[< $code:upper _CURRENCY>]
            }
        }
    };
}

pub(crate) use currency;
pub(crate) use get;
