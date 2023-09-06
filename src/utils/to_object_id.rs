use bson::oid::{ObjectId, Error};

pub fn to_object_id<S: AsRef<str>>(id: S) -> Result<ObjectId, Error> {
  ObjectId::parse_str(id.as_ref())
}