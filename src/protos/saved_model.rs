// This file is generated by rust-protobuf 2.27.1. Do not edit
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
//! Generated file from `tensorflow/core/protobuf/saved_model.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_27_1;

#[derive(PartialEq,Clone,Default)]
pub struct SavedModel {
    // message fields
    pub saved_model_schema_version: i64,
    pub meta_graphs: ::protobuf::RepeatedField<super::meta_graph::MetaGraphDef>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a SavedModel {
    fn default() -> &'a SavedModel {
        <SavedModel as ::protobuf::Message>::default_instance()
    }
}

impl SavedModel {
    pub fn new() -> SavedModel {
        ::std::default::Default::default()
    }

    // int64 saved_model_schema_version = 1;


    pub fn get_saved_model_schema_version(&self) -> i64 {
        self.saved_model_schema_version
    }
    pub fn clear_saved_model_schema_version(&mut self) {
        self.saved_model_schema_version = 0;
    }

    // Param is passed by value, moved
    pub fn set_saved_model_schema_version(&mut self, v: i64) {
        self.saved_model_schema_version = v;
    }

    // repeated .tensorflow.MetaGraphDef meta_graphs = 2;


    pub fn get_meta_graphs(&self) -> &[super::meta_graph::MetaGraphDef] {
        &self.meta_graphs
    }
    pub fn clear_meta_graphs(&mut self) {
        self.meta_graphs.clear();
    }

    // Param is passed by value, moved
    pub fn set_meta_graphs(&mut self, v: ::protobuf::RepeatedField<super::meta_graph::MetaGraphDef>) {
        self.meta_graphs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_meta_graphs(&mut self) -> &mut ::protobuf::RepeatedField<super::meta_graph::MetaGraphDef> {
        &mut self.meta_graphs
    }

    // Take field
    pub fn take_meta_graphs(&mut self) -> ::protobuf::RepeatedField<super::meta_graph::MetaGraphDef> {
        ::std::mem::replace(&mut self.meta_graphs, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for SavedModel {
    fn is_initialized(&self) -> bool {
        for v in &self.meta_graphs {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.saved_model_schema_version = tmp;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.meta_graphs)?;
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
        if self.saved_model_schema_version != 0 {
            my_size += ::protobuf::rt::value_size(1, self.saved_model_schema_version, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.meta_graphs {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.saved_model_schema_version != 0 {
            os.write_int64(1, self.saved_model_schema_version)?;
        }
        for v in &self.meta_graphs {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

    fn new() -> SavedModel {
        SavedModel::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "saved_model_schema_version",
                |m: &SavedModel| { &m.saved_model_schema_version },
                |m: &mut SavedModel| { &mut m.saved_model_schema_version },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::meta_graph::MetaGraphDef>>(
                "meta_graphs",
                |m: &SavedModel| { &m.meta_graphs },
                |m: &mut SavedModel| { &mut m.meta_graphs },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<SavedModel>(
                "SavedModel",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static SavedModel {
        static instance: ::protobuf::rt::LazyV2<SavedModel> = ::protobuf::rt::LazyV2::INIT;
        instance.get(SavedModel::new)
    }
}

impl ::protobuf::Clear for SavedModel {
    fn clear(&mut self) {
        self.saved_model_schema_version = 0;
        self.meta_graphs.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SavedModel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SavedModel {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n*tensorflow/core/protobuf/saved_model.proto\x12\ntensorflow\x1a)tensor\
    flow/core/protobuf/meta_graph.proto\"\x84\x01\n\nSavedModel\x12;\n\x1asa\
    ved_model_schema_version\x18\x01\x20\x01(\x03R\x17savedModelSchemaVersio\
    n\x129\n\x0bmeta_graphs\x18\x02\x20\x03(\x0b2\x18.tensorflow.MetaGraphDe\
    fR\nmetaGraphsB\x88\x01\n\x18org.tensorflow.frameworkB\x10SavedModelProt\
    osP\x01ZUgithub.com/tensorflow/tensorflow/tensorflow/go/core/protobuf/fo\
    r_core_protos_go_proto\xf8\x01\x01b\x06proto3\
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
