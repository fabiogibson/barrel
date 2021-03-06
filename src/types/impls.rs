//! Implementation specifics for the type system

/// Core type enum, describing the basic type
#[derive(PartialEq, Debug, Clone)]
pub(crate) enum BaseType {
    /// Strings
    Text,
    /// Like a String but worse
    Varchar,
    /// Primary key (utility for incrementing integer – postgres supports this, we just mirror it)
    Primary,
    /// Simple integer
    Integer,
    /// Floating point number
    Float,
    /// Like Float but ~~double precision~~
    Double,
    /// True or False
    Boolean,
    /// <inconceivable jibberish>
    Binary,
    /// Foreign key to other table
    Foreign(&'static str),
    /// I have no idea what you are – but I *like* it
    Custom(&'static str),
    /// Any of the above, but **many** of them
    Array(Box<BaseType>),
}

/// A database column type and all the metadata attached to it
///
/// Using this struct directly is not recommended. Instead, you should be
/// using the constructor APIs in the `types` module.
///
/// ```norun
/// use barrel::types::*;
///
/// let column = varchar()
///                 .size(255)
///                 .nullable(true)
///                 .indexed(true)
///                 .unique(true);
/// ```
///
/// Please see the **default vaulues** section in the `types` module docs!
pub struct Type<T> {
    pub nullable: bool,
    pub unique: bool,
    pub increments: bool,
    pub indexed: bool,
    pub default: Option<T>,
    pub size: Option<usize>,
    inner: BaseType,
}

/// This is a public API, be considered about breaking thigns
#[cfg_attr(rustfmt, rustfmt_skip)]
impl<T> Type<T> {
    pub(crate) fn new(inner: BaseType) -> Self {
        Self {
            nullable: false,
            unique: false,
            increments: false,
            indexed: false,
            default: None,
            size: None,
            inner,
        }
    }

    /// Validate provided metadata against
    pub(crate) fn validate(&self) -> bool {
        true
    }

    /// Function used to hide the inner type to outside users (sneaky, I know)
    pub(crate) fn get_inner(&self) -> BaseType {
        self.inner.clone()
    }
    
    /// Set the nullability of this type
    pub fn nullable(self, arg: bool) -> Self {
        Self { nullable: arg, ..self }
    }
    
    /// Set the uniqueness of this type
    pub fn unique(self, arg: bool) -> Self {
        Self { unique: arg, ..self }
    }
    
    /// Specify if this type should auto-increment
    pub fn increments(self, arg: bool) -> Self {
        Self { increments: arg, ..self }
    }
    
    /// Specify if this type should be indexed by your SQL implementation
    pub fn indexed(self, arg: bool) -> Self {
        Self { indexed: arg, ..self }
    }
    
    /// Provide a default value for a type column
    pub fn default(self, arg: impl Into<T>) -> Self {
        Self { default: Some(arg.into()), ..self }
    }
    
    /// Specify a size limit (important or varchar & similar)
    pub fn size(self, arg: usize) -> Self {
        Self { size: Some(arg), ..self }
    }
}


