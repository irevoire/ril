use std::borrow::Cow;

use heed::BoxedError;
use roaring::RoaringBitmap;

pub struct RoaringBitmapCodec;

impl heed::BytesDecode<'_> for RoaringBitmapCodec {
    type DItem = RoaringBitmap;

    fn bytes_decode(bytes: &[u8]) -> Result<Self::DItem, BoxedError> {
        RoaringBitmap::deserialize_from(bytes).map_err(|e| e.into())
    }
}

impl heed::BytesEncode<'_> for RoaringBitmapCodec {
    type EItem = RoaringBitmap;

    fn bytes_encode(item: &Self::EItem) -> Result<Cow<[u8]>, BoxedError> {
        let mut bytes = Vec::with_capacity(item.serialized_size());
        item.serialize_into(&mut bytes)
            .map_err(|e| Box::new(e) as BoxedError)?;
        Ok(Cow::Owned(bytes))
    }
}
