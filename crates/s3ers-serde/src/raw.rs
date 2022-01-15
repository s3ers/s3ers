use std::{
    clone::Clone,
    fmt::{self, Debug, Formatter},
    marker::PhantomData,
};

use serde::{
    de::{Deserialize, Deserializer, IgnoredAny, MapAccess, Visitor},
    ser::{Serialize, Serializer},
};
use serde_json::value::{to_raw_value as to_raw_json_value, RawValue as RawJsonValue};

use crate::cow::MyCowStr;

/// A wrapper around `Box<RawValue>`, to be used in place of any type in the S3 endpoint
/// definition to allow request and response types to contain that said type represented by
/// the generic argument `Ev`.
///
/// s3ers offers the `Raw` wrapper to enable passing around JSON text that is only partially
/// validated. This is useful when a client receives events that do not follow the spec perfectly
/// or a server needs to generate reference hashes with the original canonical JSON string.
/// All event structs and enums implement `Serialize` / `Deserialize`, `Raw` should be used
/// to pass around events in a lossless way.
///
/// ```no_run
/// # use serde::Deserialize;
/// # use s3ers_serde::Raw;
/// # #[derive(Deserialize)]
/// # struct AnyRoomEvent;
///
/// let json = r#"{ "type": "imagine a full event", "content": {...} }"#;
///
/// let deser = serde_json::from_str::<Raw<AnyRoomEvent>>(json)
///     .unwrap() // the first Result from serde_json::from_str, will not fail
///     .deserialize() // deserialize to the inner type
///     .unwrap(); // finally get to the AnyRoomEvent
/// ```
pub struct Raw<T> {
    json: Box<RawJsonValue>,
    _ev: PhantomData<T>,
}

impl<T> Raw<T> {
    /// Create a `Raw` by serializing the given `T`.
    ///
    /// Shorthand for `serde_json::value::to_raw_value(val).map(Raw::from_json)`, but specialized to
    /// `T`.
    ///
    /// # Errors
    ///
    /// Fails if `T`s [`Serialize`] implementation fails.
    pub fn new(val: &T) -> serde_json::Result<Self>
    where
        T: Serialize,
    {
        to_raw_json_value(val).map(Self::from_json)
    }

    /// Create a `Raw` from a boxed `RawValue`.
    pub fn from_json(json: Box<RawJsonValue>) -> Self {
        Self { json, _ev: PhantomData }
    }

    /// Access the underlying json value.
    pub fn json(&self) -> &RawJsonValue {
        &self.json
    }

    /// Convert `self` into the underlying json value.
    pub fn into_json(self) -> Box<RawJsonValue> {
        self.json
    }

    /// Try to access a given field inside this `Raw`, assuming it contains an object.
    ///
    /// Returns `Err(_)` when the contained value is not an object, or the field exists but is fails
    /// to deserialize to the expected type.
    ///
    /// Returns `Ok(None)` when the field doesn't exist or is `null`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # type CustomS3Event = ();
    /// # fn foo() -> serde_json::Result<()> {
    /// # let raw_event: s3ers_serde::Raw<()> = todo!();
    /// if raw_event.get_field::<String>("type")?.as_deref() == Some("org.custom.s3.event") {
    ///     let event = raw_event.deserialize_as::<CustomS3Event>()?;
    ///     // ...
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_field<'a, U>(&'a self, field_name: &str) -> serde_json::Result<Option<U>>
    where
        U: Deserialize<'a>,
    {
        struct SingleFieldVisitor<'b, T> {
            field_name: &'b str,
            _phantom: PhantomData<T>,
        }

        impl<'b, T> SingleFieldVisitor<'b, T> {
            fn new(field_name: &'b str) -> Self {
                Self { field_name, _phantom: PhantomData }
            }
        }

        impl<'b, 'de, T> Visitor<'de> for SingleFieldVisitor<'b, T>
        where
            T: Deserialize<'de>,
        {
            type Value = Option<T>;

            fn expecting(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("a string")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut res = None;
                while let Some(key) = map.next_key::<MyCowStr<'_>>()? {
                    if key.get() == self.field_name {
                        res = Some(map.next_value()?);
                    } else {
                        map.next_value::<IgnoredAny>()?;
                    }
                }

                Ok(res)
            }
        }

        let mut deserializer = serde_json::Deserializer::from_str(self.json().get());
        deserializer.deserialize_map(SingleFieldVisitor::new(field_name))
    }

    /// Try to deserialize the JSON as the expected type.
    pub fn deserialize<'a>(&'a self) -> serde_json::Result<T>
    where
        T: Deserialize<'a>,
    {
        serde_json::from_str(self.json.get())
    }

    /// Try to deserialize the JSON as a custom type.
    pub fn deserialize_as<'a, U>(&'a self) -> serde_json::Result<U>
    where
        U: Deserialize<'a>,
    {
        serde_json::from_str(self.json.get())
    }
}

impl<T> Clone for Raw<T> {
    fn clone(&self) -> Self {
        Self::from_json(self.json.clone())
    }
}

impl<T> Debug for Raw<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use std::any::type_name;
        f.debug_struct(&format!("Raw::<{}>", type_name::<T>())).field("json", &self.json).finish()
    }
}

impl<'de, T> Deserialize<'de> for Raw<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Box::<RawJsonValue>::deserialize(deserializer).map(Self::from_json)
    }
}

impl<T> Serialize for Raw<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.json.serialize(serializer)
    }
}
