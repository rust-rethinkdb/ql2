use std::string::String as StdString;

use proto::{
    Term, Datum,
    Term_TermType as TermType,
    Term_AssocPair as TermPair,
    Datum_DatumType as DatumType,
    Datum_AssocPair as DatumPair,
};

use protobuf::repeated::RepeatedField;
use serde_json::value::{ToJson, Value};

include!(concat!(env!("OUT_DIR"), "/serde_types.rs"));

macro_rules! implement {
    (Selection<$dt:ident>) => {
        impl DataType for Selection<$dt> {}

        impl Selection<$dt> {
            pub fn new() -> Selection<$dt> {
                Selection::<$dt>($dt(Term::new()))
            }
        }

        impl From<Selection<$dt>> for Term {
            fn from(t: Selection<$dt>) -> Term {
                From::from(t.0)
            }
        }

        impl From<Term> for Selection<$dt> {
            fn from(t: Term) -> Selection<$dt> {
                Selection::<$dt>($dt(t))
            }
        }
    };

    ($(#[$attr:meta])* pub struct Command) => {
        $(
            #[$attr]
        )*
        #[derive(Debug, Clone)]
        pub struct Command(Term);

        impl DataType for Command {}

        impl From<Command> for Term {
            fn from(t: Command) -> Term {
                t.0
            }
        }

        impl From<Term> for Command {
            fn from(t: Term) -> Command {
                Command(t)
            }
        }
    };

    ($(#[$attr:meta])* pub struct $dt:ident) => {
        $(
            #[$attr]
        )*
        #[derive(Debug, Clone)]
        pub struct $dt(Term);

        impl DataType for $dt {}

        impl $dt {
            pub fn new() -> $dt {
                $dt(Term::new())
            }
        }

        impl From<$dt> for Term {
            fn from(t: $dt) -> Term {
                t.0
            }
        }

        impl From<Term> for $dt {
            fn from(t: Term) -> $dt {
                $dt(t)
            }
        }
    };
}

pub trait DataType : From<Term> + Into<Term> + Clone {}

implement! {
    /// **Arrays** are lists of zero or more elements.
    ///
    /// ```json
    /// [1, 2, 3]
    /// []
    /// [{user: 'Bob', posts: 23}, {user: 'Jason', posts: 10}]
    /// ```
    /// Anything valid in a JSON array is valid in RethinkDB.
    /// The elements may be any of the basic values, objects,
    /// or other arrays. Arrays in RethinkDB are loaded fully
    /// into memory before they’re returned to the user, so
    /// they’re inefficient at large sizes. RethinkDB defaults
    /// to supporting arrays of up to 100,000 elements; this
    /// may be set to a different value at runtime for reading
    /// by using the `array_limit` option to run.
    pub struct Array
}

implement! {
    /// **Booleans** are `true` and `false`
    pub struct Bool
}

implement! {
    /// **Databases** are RethinkDB databases.
    ///
    /// This is the return type of `db`.
    pub struct Db
}

implement! {
    /// **Functions** can be passed as parameters to certain ReQL
    /// commands.
    pub struct Function
}

implement! {
    /// **Grouped data** is created by the `group` command.
    ///
    /// The command partitions a stream into multiple groups
    /// based on specified fields or functions. ReQL commands
    /// called on GroupedData operate on each group
    /// individually. For more details, read the group
    /// documentation. Depending on the input to group,
    /// grouped data may have the type of GroupedStream.
    pub struct GroupedData
}

implement! {
    pub struct GroupedStream
}

implement! {
    /// **Minval** and **maxval** are used with some commands such
    /// as `between` to specify absolute lower and upper bounds
    /// (e.g., `between(r.minval, 1000)` would return all
    /// documents in a table whose primary key is less than
    /// 1000).
    pub struct MaxVal
}

implement! {
    pub struct MinVal
}

implement! {
    /// **Null** is a value distinct from the number zero, an
    /// empty set, or a zero-length string.
    ///
    /// Natively this is
    /// `None`. It is often used to explicitly denote the
    /// absence of any other value. The root node of a tree
    /// structure might have a parent of `null`, or a required
    /// but as yet non-initialized key might be given a value
    /// of `null`.
    pub struct Null
}

implement! {
    /// **Numbers** are any real number: `5`, `3.14159`, `-42`.
    ///
    /// RethinkDB uses double precision (64-bit) floating point
    /// numbers internally. Neither infinity nor NaN are allowed.
    pub struct Number
}

implement! {
    /// **Objects** are JSON data objects, standard key-value pairs.
    ///
    /// ```json
    /// {
    ///     username: 'bob',
    ///     posts: 23,
    ///     favorites: {
    ///         color: 'blue',
    ///         food: 'tacos'
    ///     },
    ///     friends: ['agatha', 'jason']
    /// }
    /// ```
    ///
    /// Any valid JSON object is a valid RethinkDB object, so
    /// values can be any of the basic values, arrays, or
    /// other objects. Documents in a RethinkDB database are
    /// objects. Like JSON, key names must be strings, not
    /// integers.
    pub struct Object
}

implement! {
    /// **Binary** objects are similar to BLOBs in SQL databases:
    /// files, images and other binary data.
    ///
    /// See Storing binary objects for details.
    pub struct Binary
}

implement! {
    /// **Geometry** data types for geospatial support, including
    /// points, lines, and polygons.
    pub struct Geometry
}

implement! {
    /// **Times** are RethinkDB’s native date/time type, stored
    /// with millisecond precision.
    ///
    /// You can use native date/time types in supported
    /// languages, as the conversion will be done by the driver.
    /// See Dates and times in RethinkDB for details.
    pub struct Time
}

implement! {
    /// **Streams** are lists like arrays, but they’re loaded in
    /// a lazy fashion.
    ///
    /// Operations that return streams return a `cursor`.
    /// A cursor is a pointer into the result set. Instead of
    /// reading the results all at once like an array, you
    /// loop over the results, retrieving the next member of the
    /// set with each iteration. This makes it possible to
    /// efficiently work with large result sets.
    /// 
    /// (See “Working with Streams,” below, for some tips.)
    /// Streams are read-only; you can’t pass one as an input
    /// to an ReQL command meant to modify its input like
    /// `update` or `delete`.
    pub struct Stream
}

implement! {
    /// **Strings** are any valid UTF-8 string: `"superhero"`,
    /// `"ünnëcëssärÿ ümläüts"`. Strings may include the null
    /// code point (U+0000).
    pub struct String
}

implement! {
    /// **Tables** are RethinkDB database tables.
    ///
    /// They behave like selections—they’re writable, as you can
    /// insert and delete documents in them. ReQL methods that
    /// use an index, like `get_all`, are only available on
    /// tables.
    pub struct Table
}

implement! {
    pub struct TableSlice
}

/// **Selections** represent subsets of tables, for example,
/// the return values of `filter` or `get`.
///
/// There are three kinds of selections: `Selection<Object>`,
/// `Selection<Array>` and `Selection<Stream>`. The
/// difference between selections and their non-selection
/// counterparts is that selections are writable—their
/// return values can be passed as inputs to ReQL commands
/// that modify the database. For instance, the get command
/// will return a Selection<Object> that could then be
/// passed to an update or delete command.
///
/// (Note: _singleSelection_ is an older term for
/// Selection<Object>; they mean the same thing.)
/// Some commands (`order_by` and `between`) return a data
/// type similar to a selection called a `table_slice`.
/// In most cases a table_slice behaves identically to a
/// selection, but `between` can only be called on a table
/// or a table_slice, not any other kind of selection.
#[derive(Debug, Clone)]
pub struct Selection<T>(T);
implement! { Selection<Object> }
implement! { Selection<Array> }
implement! { Selection<Stream> }

pub type ObjectSelection = Selection<Object>;
pub type ArraySelection = Selection<Array>;
pub type StreamSelection = Selection<Stream>;

implement! {
    pub struct Command
}

#[derive(Debug, Clone)]
pub struct WithOpts<T, O>(T, O);

impl<T, O> DataType for WithOpts<T, O> where T: DataType, O: Default + ToJson + Clone { }

impl<T, O> WithOpts<T, O>
    where T: DataType, O: Default + ToJson + Clone
{
    pub fn new(cmd: T, opts: O) -> WithOpts<T, O>
    {
        WithOpts(cmd, opts)
    }
}

impl<T, O> From<WithOpts<T, O>> for Term
    where T: DataType, O: Default + ToJson + Clone
{
    fn from(t: WithOpts<T, O>) -> Term {
        let obj = Object::from(t.1);
        Command(t.0.into())
            .with_opts(obj)
            .into()
    }
}

impl<T, O> From<Term> for WithOpts<T, O>
    where T: DataType, O: Default + ToJson + Clone
{
    fn from(t: Term) -> WithOpts<T, O> {
        WithOpts(t.into(), Default::default())
    }
}

impl<T> From<T> for Object
    where T: ToJson
{
    fn from(t: T) -> Object {
        let term = Term::from_json(t);
        From::from(term)
    }
}

impl<T> From<T> for String
    where T: Into<StdString>
{
    fn from(t: T) -> String {
        let mut datum = Datum::new();
        datum.set_field_type(DatumType::R_STR);
        datum.set_r_str(t.into());
        let mut output = String::new();
        output.0.set_field_type(TermType::DATUM);
        output.0.set_datum(datum);
        output
    }
}

impl Command {
    pub fn new(cmd_type: TermType, prev_cmd: Option<Term>) -> Command
        {
            let mut term = Term::new();
            term.set_field_type(cmd_type);
            if let Some(cmd) = prev_cmd {
                let args = RepeatedField::from_vec(vec![cmd]);
                term.set_args(args);
            }
            Command(term)
        }

    pub fn with_args(mut self, args: Term) -> Self
        {
            self.0.mut_args().push(args);
            self
        }

    pub fn with_opts(mut self, opts: Object) -> Self
        {
            let mut opts: Term = opts.into();
            if opts.has_datum() {
                let mut datum = opts.take_datum();
                let pairs = datum.take_r_object().into_vec();
                for mut pair in pairs {
                    if pair.has_key() {
                        let mut term_pair = TermPair::new();
                        term_pair.set_key(pair.take_key());
                        let mut val = Term::new();
                        val.set_field_type(TermType::DATUM);
                        val.set_datum(pair.take_val());
                        term_pair.set_val(val);
                        self.0.mut_optargs().push(term_pair);
                    }
                }
            }
            self
        }

    pub fn into<O>(self) -> O
        where O: From<Term>
    {
        From::from(self.0)
    }
}

impl Term {
    fn from_json<T: ToJson>(t: T) -> Term {
        // Datum
        let mut datum = Datum::new();
        match t.to_json() {
            Value::String(val) => {
                datum.set_field_type(DatumType::R_STR);
                datum.set_r_str(val);
            },
            Value::Bool(val) => {
                datum.set_field_type(DatumType::R_BOOL);
                datum.set_r_bool(val);
            },
            Value::I64(val) => {
                datum.set_field_type(DatumType::R_NUM);
                datum.set_r_num(val as f64);
            },
            Value::U64(val) => {
                datum.set_field_type(DatumType::R_NUM);
                datum.set_r_num(val as f64);
            },
            Value::F64(val) => {
                datum.set_field_type(DatumType::R_NUM);
                datum.set_r_num(val);
            },
            Value::Array(val) => {
                datum.set_field_type(DatumType::R_ARRAY);
                let args: Vec<Datum> = val.iter()
                    .map(|a| Term::from_json(a).take_datum())
                    .collect();
                let arr = RepeatedField::from_vec(args);
                datum.set_r_array(arr);
            },
            Value::Object(val) => {
                datum.set_field_type(DatumType::R_OBJECT);
                let args: Vec<DatumPair> = val.into_iter()
                    .map(|(name, arg)| {
                        let mut obj = DatumPair::new();
                        obj.set_key(name.into());
                        obj.set_val(Term::from_json(arg).take_datum());
                        obj
                    })
                    .collect();
                let obj = RepeatedField::from_vec(args);
                datum.set_r_object(obj);
            },
            Value::Null => {
                datum.set_field_type(DatumType::R_NULL);
            },
        }
        // Term
        let mut term = Term::new();
        term.set_field_type(TermType::DATUM);
        term.set_datum(datum);
        term
    }
}

pub trait WithTableOpts : DataType {
    fn read_mode(mut self, arg: ReadMode) -> Self;
    fn identifier_format(mut self, arg: IdentifierFormat) -> Self;
}

impl<T> WithTableOpts for WithOpts<T, TableOpts>
    where T: DataType
{
    fn read_mode(mut self, arg: ReadMode) -> Self {
        self.1.read_mode = arg;
        self
    }

    fn identifier_format(mut self, arg: IdentifierFormat) -> Self {
        self.1.identifier_format = arg;
        self
    }
}

pub trait WithChangesOpts<T: DataType, A: SquashArg> : DataType {
    fn squash<B>(mut self, arg: B) -> WithOpts<T, ChangesOpts<B>>
        where B: SquashArg, ChangesOpts<B>: Default + ToJson + Clone;
    fn changefeed_queue_size(mut self, arg: u64) -> Self;
    fn include_initial(mut self, arg: bool) -> Self;
    fn include_states(mut self, arg: bool) -> Self;
    fn include_offsets(mut self, arg: bool) -> Self;
    fn include_types(mut self, arg: bool) -> Self;
}

pub trait SquashArg where Self: ToJson + Clone {}

impl SquashArg for bool {}
impl SquashArg for f32 {}

impl<T, A> WithChangesOpts<T, A> for WithOpts<T, ChangesOpts<A>>
    where T: DataType, A: SquashArg, ChangesOpts<A>: Default + ToJson + Clone
{
    fn squash<B>(self, arg: B) -> WithOpts<T, ChangesOpts<B>>
        where B: SquashArg, ChangesOpts<B>: Default + ToJson + Clone
    {
        let opts = ChangesOpts {
            squash: arg,
            changefeed_queue_size: self.1.changefeed_queue_size,
            include_initial: self.1.include_initial,
            include_states: self.1.include_states,
            include_offsets: self.1.include_offsets,
            include_types: self.1.include_types,
        };
        WithOpts(self.0, opts)
    }

    fn changefeed_queue_size(mut self, arg: u64) -> Self {
        self.1.changefeed_queue_size = arg;
        self
    }

    fn include_initial(mut self, arg: bool) -> Self {
        self.1.include_initial = arg;
        self
    }

    fn include_states(mut self, arg: bool) -> Self {
        self.1.include_states = arg;
        self
    }

    fn include_offsets(mut self, arg: bool) -> Self {
        self.1.include_offsets = arg;
        self
    }

    fn include_types(mut self, arg: bool) -> Self {
        self.1.include_types = arg;
        self
    }
}
