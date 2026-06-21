pub trait Readable {}
pub trait Writeable {}

pub struct ReadOnly;
pub struct WriteOnly;
pub struct ReadWrite;

impl Readable for ReadOnly {}
impl Writeable for WriteOnly {}
impl Readable for ReadWrite {}
impl Writeable for ReadWrite {}
