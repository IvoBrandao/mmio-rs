/// Marker trait for register types that support reading.
pub trait Readable {}
/// Marker trait for register types that support writing.
pub trait Writeable {}

/// Access marker: register can only be read.
pub struct ReadOnly;
/// Access marker: register can only be written.
pub struct WriteOnly;
/// Access marker: register can be both read and written.
pub struct ReadWrite;

impl Readable for ReadOnly {}
impl Writeable for WriteOnly {}
impl Readable for ReadWrite {}
impl Writeable for ReadWrite {}
