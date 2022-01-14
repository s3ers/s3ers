use bytes::BufMut;
use serde::Serialize;

/// Converts a byte slice to a buffer by copying.
pub fn slice_to_buf<B: Default + BufMut>(s: &[u8]) -> B {
    let mut buf = B::default();
    buf.put_slice(s);
    buf
}

/// Creates a buffer and writes a serializable value to it.
pub fn xml_to_buf<B: Default + BufMut, T: Serialize>(
    val: &T,
) -> Result<B, quick_xml::de::DeError> {
    let mut buf = B::default().writer();
    quick_xml::se::to_writer(&mut buf, val)?;
    Ok(buf.into_inner())
}
