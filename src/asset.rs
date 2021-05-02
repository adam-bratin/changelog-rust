use packer::Packer;

#[derive(Packer)]
#[packer(source = "src/assets/", prefixed = false)]
pub struct Assets;
