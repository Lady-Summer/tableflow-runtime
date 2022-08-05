// This file is generated by rust-protobuf 3.1.0. Do not edit
// .proto file is parsed by protoc --rust-out=...
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
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `coordinator/coordinator.proto`

use crate::common::{common, probe, stream};

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_1_0;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:coordinator.CreateStreamGraphResponse)
pub struct CreateStreamGraphResponse {
    // message fields
    // @@protoc_insertion_point(field:coordinator.CreateStreamGraphResponse.status)
    pub status: ::protobuf::EnumOrUnknown<stream::DataflowStatus>,
    // special fields
    // @@protoc_insertion_point(special_field:coordinator.CreateStreamGraphResponse.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CreateStreamGraphResponse {
    fn default() -> &'a CreateStreamGraphResponse {
        <CreateStreamGraphResponse as ::protobuf::Message>::default_instance()
    }
}

impl CreateStreamGraphResponse {
    pub fn new() -> CreateStreamGraphResponse {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "status",
            |m: &CreateStreamGraphResponse| { &m.status },
            |m: &mut CreateStreamGraphResponse| { &mut m.status },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<CreateStreamGraphResponse>(
            "CreateStreamGraphResponse",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for CreateStreamGraphResponse {
    const NAME: &'static str = "CreateStreamGraphResponse";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.status = is.read_enum_or_unknown()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.status != ::protobuf::EnumOrUnknown::new(common::proto::stream::DataflowStatus::INITIALIZED) {
            os.write_enum(1, ::protobuf::EnumOrUnknown::value(&self.status))?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.status != ::protobuf::EnumOrUnknown::new(common::proto::stream::DataflowStatus::INITIALIZED) {
            my_size += ::protobuf::rt::int32_size(1, self.status.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> CreateStreamGraphResponse {
        CreateStreamGraphResponse::new()
    }

    fn clear(&mut self) {
        self.status = ::protobuf::EnumOrUnknown::new(common::proto::stream::DataflowStatus::INITIALIZED);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CreateStreamGraphResponse {
        static instance: CreateStreamGraphResponse = CreateStreamGraphResponse {
            status: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for CreateStreamGraphResponse {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("CreateStreamGraphResponse").unwrap()).clone()
    }
}

impl ::std::fmt::Display for CreateStreamGraphResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CreateStreamGraphResponse {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:coordinator.TerminateDataflowRequest)
pub struct TerminateDataflowRequest {
    // message fields
    // @@protoc_insertion_point(field:coordinator.TerminateDataflowRequest.job_id)
    pub job_id: ::protobuf::MessageField<common::proto::common::JobId>,
    // special fields
    // @@protoc_insertion_point(special_field:coordinator.TerminateDataflowRequest.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a TerminateDataflowRequest {
    fn default() -> &'a TerminateDataflowRequest {
        <TerminateDataflowRequest as ::protobuf::Message>::default_instance()
    }
}

impl TerminateDataflowRequest {
    pub fn new() -> TerminateDataflowRequest {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, common::proto::common::JobId>(
            "job_id",
            |m: &TerminateDataflowRequest| { &m.job_id },
            |m: &mut TerminateDataflowRequest| { &mut m.job_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<TerminateDataflowRequest>(
            "TerminateDataflowRequest",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for TerminateDataflowRequest {
    const NAME: &'static str = "TerminateDataflowRequest";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.job_id)?;
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
        if let Some(v) = self.job_id.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.job_id.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> TerminateDataflowRequest {
        TerminateDataflowRequest::new()
    }

    fn clear(&mut self) {
        self.job_id.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static TerminateDataflowRequest {
        static instance: TerminateDataflowRequest = TerminateDataflowRequest {
            job_id: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for TerminateDataflowRequest {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("TerminateDataflowRequest").unwrap()).clone()
    }
}

impl ::std::fmt::Display for TerminateDataflowRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TerminateDataflowRequest {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:coordinator.TerminateDataflowResponse)
pub struct TerminateDataflowResponse {
    // message fields
    // @@protoc_insertion_point(field:coordinator.TerminateDataflowResponse.status)
    pub status: ::protobuf::EnumOrUnknown<stream::DataflowStatus>,
    // special fields
    // @@protoc_insertion_point(special_field:coordinator.TerminateDataflowResponse.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a TerminateDataflowResponse {
    fn default() -> &'a TerminateDataflowResponse {
        <TerminateDataflowResponse as ::protobuf::Message>::default_instance()
    }
}

impl TerminateDataflowResponse {
    pub fn new() -> TerminateDataflowResponse {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "status",
            |m: &TerminateDataflowResponse| { &m.status },
            |m: &mut TerminateDataflowResponse| { &mut m.status },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<TerminateDataflowResponse>(
            "TerminateDataflowResponse",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for TerminateDataflowResponse {
    const NAME: &'static str = "TerminateDataflowResponse";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.status = is.read_enum_or_unknown()?;
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
        if self.status != ::protobuf::EnumOrUnknown::new(stream::DataflowStatus::INITIALIZED) {
            my_size += ::protobuf::rt::int32_size(1, self.status.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.status != ::protobuf::EnumOrUnknown::new(stream::DataflowStatus::INITIALIZED) {
            os.write_enum(1, ::protobuf::EnumOrUnknown::value(&self.status))?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> TerminateDataflowResponse {
        TerminateDataflowResponse::new()
    }

    fn clear(&mut self) {
        self.status = ::protobuf::EnumOrUnknown::new(stream::DataflowStatus::INITIALIZED);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static TerminateDataflowResponse {
        static instance: TerminateDataflowResponse = TerminateDataflowResponse {
            status: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for TerminateDataflowResponse {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("TerminateDataflowResponse").unwrap()).clone()
    }
}

impl ::std::fmt::Display for TerminateDataflowResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TerminateDataflowResponse {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:coordinator.GetDataflowRequest)
pub struct GetDataflowRequest {
    // message fields
    // @@protoc_insertion_point(field:coordinator.GetDataflowRequest.job_id)
    pub job_id: ::protobuf::MessageField<common::JobId>,
    // special fields
    // @@protoc_insertion_point(special_field:coordinator.GetDataflowRequest.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetDataflowRequest {
    fn default() -> &'a GetDataflowRequest {
        <GetDataflowRequest as ::protobuf::Message>::default_instance()
    }
}

impl GetDataflowRequest {
    pub fn new() -> GetDataflowRequest {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, common::JobId>(
            "job_id",
            |m: &GetDataflowRequest| { &m.job_id },
            |m: &mut GetDataflowRequest| { &mut m.job_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetDataflowRequest>(
            "GetDataflowRequest",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetDataflowRequest {
    const NAME: &'static str = "GetDataflowRequest";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.job_id)?;
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
        if let Some(v) = self.job_id.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.job_id.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> GetDataflowRequest {
        GetDataflowRequest::new()
    }

    fn clear(&mut self) {
        self.job_id.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetDataflowRequest {
        static instance: GetDataflowRequest = GetDataflowRequest {
            job_id: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GetDataflowRequest {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetDataflowRequest").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetDataflowRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetDataflowRequest {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:coordinator.GetDataflowResponse)
pub struct GetDataflowResponse {
    // message fields
    // @@protoc_insertion_point(field:coordinator.GetDataflowResponse.status)
    pub status: ::protobuf::EnumOrUnknown<stream::DataflowStatus>,
    // @@protoc_insertion_point(field:coordinator.GetDataflowResponse.graph)
    pub graph: ::protobuf::MessageField<stream::Dataflow>,
    // special fields
    // @@protoc_insertion_point(special_field:coordinator.GetDataflowResponse.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetDataflowResponse {
    fn default() -> &'a GetDataflowResponse {
        <GetDataflowResponse as ::protobuf::Message>::default_instance()
    }
}

impl GetDataflowResponse {
    pub fn new() -> GetDataflowResponse {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "status",
            |m: &GetDataflowResponse| { &m.status },
            |m: &mut GetDataflowResponse| { &mut m.status },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, stream::Dataflow>(
            "graph",
            |m: &GetDataflowResponse| { &m.graph },
            |m: &mut GetDataflowResponse| { &mut m.graph },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetDataflowResponse>(
            "GetDataflowResponse",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetDataflowResponse {
    const NAME: &'static str = "GetDataflowResponse";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.status = is.read_enum_or_unknown()?;
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.graph)?;
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
        if self.status != ::protobuf::EnumOrUnknown::new(stream::DataflowStatus::INITIALIZED) {
            my_size += ::protobuf::rt::int32_size(1, self.status.value());
        }
        if let Some(v) = self.graph.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.status != ::protobuf::EnumOrUnknown::new(stream::DataflowStatus::INITIALIZED) {
            os.write_enum(1, ::protobuf::EnumOrUnknown::value(&self.status))?;
        }
        if let Some(v) = self.graph.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> GetDataflowResponse {
        GetDataflowResponse::new()
    }

    fn clear(&mut self) {
        self.status = ::protobuf::EnumOrUnknown::new(stream::DataflowStatus::INITIALIZED);
        self.graph.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetDataflowResponse {
        static instance: GetDataflowResponse = GetDataflowResponse {
            status: ::protobuf::EnumOrUnknown::from_i32(0),
            graph: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GetDataflowResponse {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetDataflowResponse").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetDataflowResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetDataflowResponse {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1dcoordinator/coordinator.proto\x12\x0bcoordinator\x1a\x12common/pro\
    be.proto\x1a\x13common/stream.proto\x1a\x13common/common.proto\"K\n\x19C\
    reateStreamGraphResponse\x12.\n\x06status\x18\x01\x20\x01(\x0e2\x16.comm\
    on.DataflowStatusR\x06status\"@\n\x18TerminateDataflowRequest\x12$\n\x06\
    job_id\x18\x01\x20\x01(\x0b2\r.common.JobIdR\x05jobId\"K\n\x19TerminateD\
    ataflowResponse\x12.\n\x06status\x18\x01\x20\x01(\x0e2\x16.common.Datafl\
    owStatusR\x06status\":\n\x12GetDataflowRequest\x12$\n\x06job_id\x18\x01\
    \x20\x01(\x0b2\r.common.JobIdR\x05jobId\"m\n\x13GetDataflowResponse\x12.\
    \n\x06status\x18\x01\x20\x01(\x0e2\x16.common.DataflowStatusR\x06status\
    \x12&\n\x05graph\x18\x02\x20\x01(\x0b2\x10.common.DataflowR\x05graph2\
    \xd0\x02\n\x0eCoordinatorApi\x126\n\x05Probe\x12\x14.common.ProbeRequest\
    \x1a\x15.common.ProbeResponse\"\0\x12L\n\x0eCreateDataflow\x12\x10.commo\
    n.Dataflow\x1a&.coordinator.CreateStreamGraphResponse\"\0\x12d\n\x11Term\
    inateDataflow\x12%.coordinator.TerminateDataflowRequest\x1a&.coordinator\
    .TerminateDataflowResponse\"\0\x12R\n\x0bGetDataflow\x12\x1f.coordinator\
    .GetDataflowRequest\x1a\x20.coordinator.GetDataflowResponse\"\0B\x07Z\
    \x05protoJ\x88\x06\n\x06\x12\x04\0\0#\x01\n\x08\n\x01\x0c\x12\x03\0\0\
    \x12\n\x08\n\x01\x02\x12\x03\x02\0\x14\n\t\n\x02\x03\0\x12\x03\x03\0\x1c\
    \n\t\n\x02\x03\x01\x12\x03\x04\0\x1d\n\t\n\x02\x03\x02\x12\x03\x05\0\x1d\
    \n\x08\n\x01\x08\x12\x03\x07\0\x1c\n\t\n\x02\x08\x0b\x12\x03\x07\0\x1c\n\
    \n\n\x02\x06\0\x12\x04\t\0\x0e\x01\n\n\n\x03\x06\0\x01\x12\x03\t\x08\x16\
    \n\x0b\n\x04\x06\0\x02\0\x12\x03\n\x02B\n\x0c\n\x05\x06\0\x02\0\x01\x12\
    \x03\n\x06\x0b\n\x0c\n\x05\x06\0\x02\0\x02\x12\x03\n\x0c\x1f\n\x0c\n\x05\
    \x06\0\x02\0\x03\x12\x03\n*>\n\x0b\n\x04\x06\0\x02\x01\x12\x03\x0b\x02L\
    \n\x0c\n\x05\x06\0\x02\x01\x01\x12\x03\x0b\x06\x14\n\x0c\n\x05\x06\0\x02\
    \x01\x02\x12\x03\x0b\x15$\n\x0c\n\x05\x06\0\x02\x01\x03\x12\x03\x0b/H\n\
    \x0b\n\x04\x06\0\x02\x02\x12\x03\x0c\x02X\n\x0c\n\x05\x06\0\x02\x02\x01\
    \x12\x03\x0c\x06\x17\n\x0c\n\x05\x06\0\x02\x02\x02\x12\x03\x0c\x180\n\
    \x0c\n\x05\x06\0\x02\x02\x03\x12\x03\x0c;T\n\x0b\n\x04\x06\0\x02\x03\x12\
    \x03\r\x02F\n\x0c\n\x05\x06\0\x02\x03\x01\x12\x03\r\x06\x11\n\x0c\n\x05\
    \x06\0\x02\x03\x02\x12\x03\r\x12$\n\x0c\n\x05\x06\0\x02\x03\x03\x12\x03\
    \r/B\n\n\n\x02\x04\0\x12\x04\x10\0\x12\x01\n\n\n\x03\x04\0\x01\x12\x03\
    \x10\x08!\n\x0b\n\x04\x04\0\x02\0\x12\x03\x11\x02#\n\x0c\n\x05\x04\0\x02\
    \0\x06\x12\x03\x11\x02\x17\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x11\x18\
    \x1e\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x11!\"\n\n\n\x02\x04\x01\x12\
    \x04\x14\0\x16\x01\n\n\n\x03\x04\x01\x01\x12\x03\x14\x08\x20\n\x0b\n\x04\
    \x04\x01\x02\0\x12\x03\x15\x02\x1a\n\x0c\n\x05\x04\x01\x02\0\x06\x12\x03\
    \x15\x02\x0e\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x15\x0f\x15\n\x0c\n\
    \x05\x04\x01\x02\0\x03\x12\x03\x15\x18\x19\n\n\n\x02\x04\x02\x12\x04\x18\
    \0\x1a\x01\n\n\n\x03\x04\x02\x01\x12\x03\x18\x08!\n\x0b\n\x04\x04\x02\
    \x02\0\x12\x03\x19\x02#\n\x0c\n\x05\x04\x02\x02\0\x06\x12\x03\x19\x02\
    \x17\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03\x19\x18\x1e\n\x0c\n\x05\x04\
    \x02\x02\0\x03\x12\x03\x19!\"\n\n\n\x02\x04\x03\x12\x04\x1c\0\x1e\x01\n\
    \n\n\x03\x04\x03\x01\x12\x03\x1c\x08\x1a\n\x0b\n\x04\x04\x03\x02\0\x12\
    \x03\x1d\x02\x1a\n\x0c\n\x05\x04\x03\x02\0\x06\x12\x03\x1d\x02\x0e\n\x0c\
    \n\x05\x04\x03\x02\0\x01\x12\x03\x1d\x0f\x15\n\x0c\n\x05\x04\x03\x02\0\
    \x03\x12\x03\x1d\x18\x19\n\n\n\x02\x04\x04\x12\x04\x20\0#\x01\n\n\n\x03\
    \x04\x04\x01\x12\x03\x20\x08\x1b\n\x0b\n\x04\x04\x04\x02\0\x12\x03!\x02#\
    \n\x0c\n\x05\x04\x04\x02\0\x06\x12\x03!\x02\x17\n\x0c\n\x05\x04\x04\x02\
    \0\x01\x12\x03!\x18\x1e\n\x0c\n\x05\x04\x04\x02\0\x03\x12\x03!!\"\n\x0b\
    \n\x04\x04\x04\x02\x01\x12\x03\"\x02\x1c\n\x0c\n\x05\x04\x04\x02\x01\x06\
    \x12\x03\"\x02\x11\n\x0c\n\x05\x04\x04\x02\x01\x01\x12\x03\"\x12\x17\n\
    \x0c\n\x05\x04\x04\x02\x01\x03\x12\x03\"\x1a\x1bb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(3);
            deps.push(probe::file_descriptor().clone());
            deps.push(stream::file_descriptor().clone());
            deps.push(common::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(5);
            messages.push(CreateStreamGraphResponse::generated_message_descriptor_data());
            messages.push(TerminateDataflowRequest::generated_message_descriptor_data());
            messages.push(TerminateDataflowResponse::generated_message_descriptor_data());
            messages.push(GetDataflowRequest::generated_message_descriptor_data());
            messages.push(GetDataflowResponse::generated_message_descriptor_data());
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
