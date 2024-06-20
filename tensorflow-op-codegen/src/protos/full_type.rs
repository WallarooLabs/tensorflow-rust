// This file is generated by rust-protobuf 2.28.0. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `tensorflow/core/framework/full_type.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_28_0;

#[derive(PartialEq,Clone,Default)]
pub struct FullTypeDef {
    // message fields
    pub type_id: FullTypeId,
    pub args: ::protobuf::RepeatedField<FullTypeDef>,
    // message oneof groups
    pub attr: ::std::option::Option<FullTypeDef_oneof_attr>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a FullTypeDef {
    fn default() -> &'a FullTypeDef {
        <FullTypeDef as ::protobuf::Message>::default_instance()
    }
}

#[derive(Clone,PartialEq,Debug)]
pub enum FullTypeDef_oneof_attr {
    s(::std::string::String),
    i(i64),
}

impl FullTypeDef {
    pub fn new() -> FullTypeDef {
        ::std::default::Default::default()
    }

    // .tensorflow.FullTypeId type_id = 1;


    pub fn get_type_id(&self) -> FullTypeId {
        self.type_id
    }
    pub fn clear_type_id(&mut self) {
        self.type_id = FullTypeId::TFT_UNSET;
    }

    // Param is passed by value, moved
    pub fn set_type_id(&mut self, v: FullTypeId) {
        self.type_id = v;
    }

    // repeated .tensorflow.FullTypeDef args = 2;


    pub fn get_args(&self) -> &[FullTypeDef] {
        &self.args
    }
    pub fn clear_args(&mut self) {
        self.args.clear();
    }

    // Param is passed by value, moved
    pub fn set_args(&mut self, v: ::protobuf::RepeatedField<FullTypeDef>) {
        self.args = v;
    }

    // Mutable pointer to the field.
    pub fn mut_args(&mut self) -> &mut ::protobuf::RepeatedField<FullTypeDef> {
        &mut self.args
    }

    // Take field
    pub fn take_args(&mut self) -> ::protobuf::RepeatedField<FullTypeDef> {
        ::std::mem::replace(&mut self.args, ::protobuf::RepeatedField::new())
    }

    // string s = 3;


    pub fn get_s(&self) -> &str {
        match self.attr {
            ::std::option::Option::Some(FullTypeDef_oneof_attr::s(ref v)) => v,
            _ => "",
        }
    }
    pub fn clear_s(&mut self) {
        self.attr = ::std::option::Option::None;
    }

    pub fn has_s(&self) -> bool {
        match self.attr {
            ::std::option::Option::Some(FullTypeDef_oneof_attr::s(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_s(&mut self, v: ::std::string::String) {
        self.attr = ::std::option::Option::Some(FullTypeDef_oneof_attr::s(v))
    }

    // Mutable pointer to the field.
    pub fn mut_s(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(FullTypeDef_oneof_attr::s(_)) = self.attr {
        } else {
            self.attr = ::std::option::Option::Some(FullTypeDef_oneof_attr::s(::std::string::String::new()));
        }
        match self.attr {
            ::std::option::Option::Some(FullTypeDef_oneof_attr::s(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_s(&mut self) -> ::std::string::String {
        if self.has_s() {
            match self.attr.take() {
                ::std::option::Option::Some(FullTypeDef_oneof_attr::s(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    // int64 i = 4;


    pub fn get_i(&self) -> i64 {
        match self.attr {
            ::std::option::Option::Some(FullTypeDef_oneof_attr::i(v)) => v,
            _ => 0,
        }
    }
    pub fn clear_i(&mut self) {
        self.attr = ::std::option::Option::None;
    }

    pub fn has_i(&self) -> bool {
        match self.attr {
            ::std::option::Option::Some(FullTypeDef_oneof_attr::i(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_i(&mut self, v: i64) {
        self.attr = ::std::option::Option::Some(FullTypeDef_oneof_attr::i(v))
    }
}

impl ::protobuf::Message for FullTypeDef {
    fn is_initialized(&self) -> bool {
        for v in &self.args {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.type_id, 1, &mut self.unknown_fields)?
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.args)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.attr = ::std::option::Option::Some(FullTypeDef_oneof_attr::s(is.read_string()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.attr = ::std::option::Option::Some(FullTypeDef_oneof_attr::i(is.read_int64()?));
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.type_id != FullTypeId::TFT_UNSET {
            my_size += ::protobuf::rt::enum_size(1, self.type_id);
        }
        for value in &self.args {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let ::std::option::Option::Some(ref v) = self.attr {
            match v {
                &FullTypeDef_oneof_attr::s(ref v) => {
                    my_size += ::protobuf::rt::string_size(3, &v);
                },
                &FullTypeDef_oneof_attr::i(v) => {
                    my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.type_id != FullTypeId::TFT_UNSET {
            os.write_enum(1, ::protobuf::ProtobufEnum::value(&self.type_id))?;
        }
        for v in &self.args {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let ::std::option::Option::Some(ref v) = self.attr {
            match v {
                &FullTypeDef_oneof_attr::s(ref v) => {
                    os.write_string(3, v)?;
                },
                &FullTypeDef_oneof_attr::i(v) => {
                    os.write_int64(4, v)?;
                },
            };
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> FullTypeDef {
        FullTypeDef::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<FullTypeId>>(
                "type_id",
                |m: &FullTypeDef| { &m.type_id },
                |m: &mut FullTypeDef| { &mut m.type_id },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<FullTypeDef>>(
                "args",
                |m: &FullTypeDef| { &m.args },
                |m: &mut FullTypeDef| { &mut m.args },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                "s",
                FullTypeDef::has_s,
                FullTypeDef::get_s,
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor::<_>(
                "i",
                FullTypeDef::has_i,
                FullTypeDef::get_i,
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<FullTypeDef>(
                "FullTypeDef",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static FullTypeDef {
        static instance: ::protobuf::rt::LazyV2<FullTypeDef> = ::protobuf::rt::LazyV2::INIT;
        instance.get(FullTypeDef::new)
    }
}

impl ::protobuf::Clear for FullTypeDef {
    fn clear(&mut self) {
        self.type_id = FullTypeId::TFT_UNSET;
        self.args.clear();
        self.attr = ::std::option::Option::None;
        self.attr = ::std::option::Option::None;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FullTypeDef {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FullTypeDef {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum FullTypeId {
    TFT_UNSET = 0,
    TFT_VAR = 1,
    TFT_ANY = 2,
    TFT_PRODUCT = 3,
    TFT_NAMED = 4,
    TFT_FOR_EACH = 20,
    TFT_CALLABLE = 100,
    TFT_TENSOR = 1000,
    TFT_ARRAY = 1001,
    TFT_OPTIONAL = 1002,
    TFT_LITERAL = 1003,
    TFT_ENCODED = 1004,
    TFT_SHAPE_TENSOR = 1005,
    TFT_BOOL = 200,
    TFT_UINT8 = 201,
    TFT_UINT16 = 202,
    TFT_UINT32 = 203,
    TFT_UINT64 = 204,
    TFT_INT8 = 205,
    TFT_INT16 = 206,
    TFT_INT32 = 207,
    TFT_INT64 = 208,
    TFT_HALF = 209,
    TFT_FLOAT = 210,
    TFT_DOUBLE = 211,
    TFT_BFLOAT16 = 215,
    TFT_COMPLEX64 = 212,
    TFT_COMPLEX128 = 213,
    TFT_STRING = 214,
    TFT_DATASET = 10102,
    TFT_RAGGED = 10103,
    TFT_ITERATOR = 10104,
    TFT_MUTEX_LOCK = 10202,
    TFT_LEGACY_VARIANT = 10203,
}

impl ::protobuf::ProtobufEnum for FullTypeId {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<FullTypeId> {
        match value {
            0 => ::std::option::Option::Some(FullTypeId::TFT_UNSET),
            1 => ::std::option::Option::Some(FullTypeId::TFT_VAR),
            2 => ::std::option::Option::Some(FullTypeId::TFT_ANY),
            3 => ::std::option::Option::Some(FullTypeId::TFT_PRODUCT),
            4 => ::std::option::Option::Some(FullTypeId::TFT_NAMED),
            20 => ::std::option::Option::Some(FullTypeId::TFT_FOR_EACH),
            100 => ::std::option::Option::Some(FullTypeId::TFT_CALLABLE),
            1000 => ::std::option::Option::Some(FullTypeId::TFT_TENSOR),
            1001 => ::std::option::Option::Some(FullTypeId::TFT_ARRAY),
            1002 => ::std::option::Option::Some(FullTypeId::TFT_OPTIONAL),
            1003 => ::std::option::Option::Some(FullTypeId::TFT_LITERAL),
            1004 => ::std::option::Option::Some(FullTypeId::TFT_ENCODED),
            1005 => ::std::option::Option::Some(FullTypeId::TFT_SHAPE_TENSOR),
            200 => ::std::option::Option::Some(FullTypeId::TFT_BOOL),
            201 => ::std::option::Option::Some(FullTypeId::TFT_UINT8),
            202 => ::std::option::Option::Some(FullTypeId::TFT_UINT16),
            203 => ::std::option::Option::Some(FullTypeId::TFT_UINT32),
            204 => ::std::option::Option::Some(FullTypeId::TFT_UINT64),
            205 => ::std::option::Option::Some(FullTypeId::TFT_INT8),
            206 => ::std::option::Option::Some(FullTypeId::TFT_INT16),
            207 => ::std::option::Option::Some(FullTypeId::TFT_INT32),
            208 => ::std::option::Option::Some(FullTypeId::TFT_INT64),
            209 => ::std::option::Option::Some(FullTypeId::TFT_HALF),
            210 => ::std::option::Option::Some(FullTypeId::TFT_FLOAT),
            211 => ::std::option::Option::Some(FullTypeId::TFT_DOUBLE),
            215 => ::std::option::Option::Some(FullTypeId::TFT_BFLOAT16),
            212 => ::std::option::Option::Some(FullTypeId::TFT_COMPLEX64),
            213 => ::std::option::Option::Some(FullTypeId::TFT_COMPLEX128),
            214 => ::std::option::Option::Some(FullTypeId::TFT_STRING),
            10102 => ::std::option::Option::Some(FullTypeId::TFT_DATASET),
            10103 => ::std::option::Option::Some(FullTypeId::TFT_RAGGED),
            10104 => ::std::option::Option::Some(FullTypeId::TFT_ITERATOR),
            10202 => ::std::option::Option::Some(FullTypeId::TFT_MUTEX_LOCK),
            10203 => ::std::option::Option::Some(FullTypeId::TFT_LEGACY_VARIANT),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [FullTypeId] = &[
            FullTypeId::TFT_UNSET,
            FullTypeId::TFT_VAR,
            FullTypeId::TFT_ANY,
            FullTypeId::TFT_PRODUCT,
            FullTypeId::TFT_NAMED,
            FullTypeId::TFT_FOR_EACH,
            FullTypeId::TFT_CALLABLE,
            FullTypeId::TFT_TENSOR,
            FullTypeId::TFT_ARRAY,
            FullTypeId::TFT_OPTIONAL,
            FullTypeId::TFT_LITERAL,
            FullTypeId::TFT_ENCODED,
            FullTypeId::TFT_SHAPE_TENSOR,
            FullTypeId::TFT_BOOL,
            FullTypeId::TFT_UINT8,
            FullTypeId::TFT_UINT16,
            FullTypeId::TFT_UINT32,
            FullTypeId::TFT_UINT64,
            FullTypeId::TFT_INT8,
            FullTypeId::TFT_INT16,
            FullTypeId::TFT_INT32,
            FullTypeId::TFT_INT64,
            FullTypeId::TFT_HALF,
            FullTypeId::TFT_FLOAT,
            FullTypeId::TFT_DOUBLE,
            FullTypeId::TFT_BFLOAT16,
            FullTypeId::TFT_COMPLEX64,
            FullTypeId::TFT_COMPLEX128,
            FullTypeId::TFT_STRING,
            FullTypeId::TFT_DATASET,
            FullTypeId::TFT_RAGGED,
            FullTypeId::TFT_ITERATOR,
            FullTypeId::TFT_MUTEX_LOCK,
            FullTypeId::TFT_LEGACY_VARIANT,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<FullTypeId>("FullTypeId", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for FullTypeId {
}

impl ::std::default::Default for FullTypeId {
    fn default() -> Self {
        FullTypeId::TFT_UNSET
    }
}

impl ::protobuf::reflect::ProtobufValue for FullTypeId {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n)tensorflow/core/framework/full_type.proto\x12\ntensorflow\"\x93\x01\n\
    \x0bFullTypeDef\x12/\n\x07type_id\x18\x01\x20\x01(\x0e2\x16.tensorflow.F\
    ullTypeIdR\x06typeId\x12+\n\x04args\x18\x02\x20\x03(\x0b2\x17.tensorflow\
    .FullTypeDefR\x04args\x12\x0e\n\x01s\x18\x03\x20\x01(\tH\0R\x01s\x12\x0e\
    \n\x01i\x18\x04\x20\x01(\x03H\0R\x01iB\x06\n\x04attr*\xda\x04\n\nFullTyp\
    eId\x12\r\n\tTFT_UNSET\x10\0\x12\x0b\n\x07TFT_VAR\x10\x01\x12\x0b\n\x07T\
    FT_ANY\x10\x02\x12\x0f\n\x0bTFT_PRODUCT\x10\x03\x12\r\n\tTFT_NAMED\x10\
    \x04\x12\x10\n\x0cTFT_FOR_EACH\x10\x14\x12\x10\n\x0cTFT_CALLABLE\x10d\
    \x12\x0f\n\nTFT_TENSOR\x10\xe8\x07\x12\x0e\n\tTFT_ARRAY\x10\xe9\x07\x12\
    \x11\n\x0cTFT_OPTIONAL\x10\xea\x07\x12\x10\n\x0bTFT_LITERAL\x10\xeb\x07\
    \x12\x10\n\x0bTFT_ENCODED\x10\xec\x07\x12\x15\n\x10TFT_SHAPE_TENSOR\x10\
    \xed\x07\x12\r\n\x08TFT_BOOL\x10\xc8\x01\x12\x0e\n\tTFT_UINT8\x10\xc9\
    \x01\x12\x0f\n\nTFT_UINT16\x10\xca\x01\x12\x0f\n\nTFT_UINT32\x10\xcb\x01\
    \x12\x0f\n\nTFT_UINT64\x10\xcc\x01\x12\r\n\x08TFT_INT8\x10\xcd\x01\x12\
    \x0e\n\tTFT_INT16\x10\xce\x01\x12\x0e\n\tTFT_INT32\x10\xcf\x01\x12\x0e\n\
    \tTFT_INT64\x10\xd0\x01\x12\r\n\x08TFT_HALF\x10\xd1\x01\x12\x0e\n\tTFT_F\
    LOAT\x10\xd2\x01\x12\x0f\n\nTFT_DOUBLE\x10\xd3\x01\x12\x11\n\x0cTFT_BFLO\
    AT16\x10\xd7\x01\x12\x12\n\rTFT_COMPLEX64\x10\xd4\x01\x12\x13\n\x0eTFT_C\
    OMPLEX128\x10\xd5\x01\x12\x0f\n\nTFT_STRING\x10\xd6\x01\x12\x10\n\x0bTFT\
    _DATASET\x10\xf6N\x12\x0f\n\nTFT_RAGGED\x10\xf7N\x12\x11\n\x0cTFT_ITERAT\
    OR\x10\xf8N\x12\x13\n\x0eTFT_MUTEX_LOCK\x10\xdaO\x12\x17\n\x12TFT_LEGACY\
    _VARIANT\x10\xdbOB\x81\x01\n\x18org.tensorflow.frameworkB\x0eFullTypePro\
    tosP\x01ZPgithub.com/tensorflow/tensorflow/tensorflow/go/core/framework/\
    full_type_go_proto\xf8\x01\x01b\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
