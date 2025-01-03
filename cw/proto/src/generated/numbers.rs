// This file is generated by rust-protobuf 3.7.1. Do not edit
// .proto file is parsed by protoc 28.3
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#[allow(unknown_lints)]
#[allow(clippy::all)]
#[allow(unused_attributes)]
#[cfg_attr(rustfmt, rustfmt::skip)]
#[allow(dead_code)]
#[allow(missing_docs)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
#[allow(trivial_casts)]
#[allow(unused_results)]
#[allow(unused_mut)]



/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:numbers.NumberArray)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct NumberArray {
    // message fields
    // @@protoc_insertion_point(field:numbers.NumberArray.values)
    pub values: ::std::vec::Vec<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:numbers.NumberArray.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a NumberArray {
    fn default() -> &'a NumberArray {
        <NumberArray as ::protobuf::Message>::default_instance()
    }
}

impl NumberArray {
    pub fn new() -> NumberArray {
        ::std::default::Default::default()
    }

    // repeated fixed32 values = 1;

    pub fn values(&self) -> &[u32] {
        &self.values
    }

    pub fn clear_values(&mut self) {
        self.values.clear();
    }

    // Param is passed by value, moved
    pub fn set_values(&mut self, v: ::std::vec::Vec<u32>) {
        self.values = v;
    }

    // Mutable pointer to the field.
    pub fn mut_values(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.values
    }

    // Take field
    pub fn take_values(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.values, ::std::vec::Vec::new())
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "values",
            |m: &NumberArray| { &m.values },
            |m: &mut NumberArray| { &mut m.values },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<NumberArray>(
            "NumberArray",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for NumberArray {
    const NAME: &'static str = "NumberArray";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    is.read_repeated_packed_fixed32_into(&mut self.values)?;
                },
                13 => {
                    self.values.push(is.read_fixed32()?);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::vec_packed_fixed32_size(1, &self.values);
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        os.write_repeated_packed_fixed32(1, &self.values)?;
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> NumberArray {
        NumberArray::new()
    }

    fn clear(&mut self) {
        self.values.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static NumberArray {
        static instance: NumberArray = NumberArray {
            values: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for NumberArray {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("NumberArray").unwrap()).clone()
    }
}

impl ::std::fmt::Display for NumberArray {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NumberArray {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\rnumbers.proto\x12\x07numbers\")\n\x0bNumberArray\x12\x1a\n\x06values\
    \x18\x01\x20\x03(\x07R\x06valuesB\x02\x10\x01b\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(NumberArray::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
