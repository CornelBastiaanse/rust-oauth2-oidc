use uuid::Uuid;

pub trait Entity {
    fn id(&self) -> Uuid;
}