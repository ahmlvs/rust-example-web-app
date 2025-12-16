pub trait IDProvider {
    fn provide(&self) -> String;
}

pub struct NanoIDProvider;

impl IDProvider for NanoIDProvider {
    fn provide(&self) -> String {
        nanoid::nanoid!(7)
    }
}

pub struct FakeIDProvider {
    id: String,
}

impl FakeIDProvider {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

impl IDProvider for FakeIDProvider {
    fn provide(&self) -> String {
        self.id.clone()
    }
}
