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

//! Generated file from `worker/worker.proto`

use crate::common;
use crate::common::{event, probe, stream};

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_1_0;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:dataflow.DispatchDataEventsRequest)
pub struct DispatchDataEventsRequest {
    // message fields
    // @@protoc_insertion_point(field:dataflow.DispatchDataEventsRequest.events)
    pub events: ::std::vec::Vec<common::event::DataEvent>,
    // special fields
    // @@protoc_insertion_point(special_field:dataflow.DispatchDataEventsRequest.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a DispatchDataEventsRequest {
    fn default() -> &'a DispatchDataEventsRequest {
        <DispatchDataEventsRequest as ::protobuf::Message>::default_instance()
    }
}

impl DispatchDataEventsRequest {
    pub fn new() -> DispatchDataEventsRequest {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "events",
            |m: &DispatchDataEventsRequest| { &m.events },
            |m: &mut DispatchDataEventsRequest| { &mut m.events },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<DispatchDataEventsRequest>(
            "DispatchDataEventsRequest",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for DispatchDataEventsRequest {
    const NAME: &'static str = "DispatchDataEventsRequest";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.events.push(is.read_message()?);
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
        for value in &self.events {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.events {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> DispatchDataEventsRequest {
        DispatchDataEventsRequest::new()
    }

    fn clear(&mut self) {
        self.events.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static DispatchDataEventsRequest {
        static instance: DispatchDataEventsRequest = DispatchDataEventsRequest {
            events: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for DispatchDataEventsRequest {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("DispatchDataEventsRequest").unwrap()).clone()
    }
}

impl ::std::fmt::Display for DispatchDataEventsRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DispatchDataEventsRequest {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:dataflow.DispatchDataEventsResponse)
pub struct DispatchDataEventsResponse {
    // message fields
    // @@protoc_insertion_point(field:dataflow.DispatchDataEventsResponse.statusSet)
    pub statusSet: ::std::collections::HashMap<::std::string::String, ::protobuf::EnumOrUnknown<DispatchDataEventStatusEnum>>,
    // special fields
    // @@protoc_insertion_point(special_field:dataflow.DispatchDataEventsResponse.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a DispatchDataEventsResponse {
    fn default() -> &'a DispatchDataEventsResponse {
        <DispatchDataEventsResponse as ::protobuf::Message>::default_instance()
    }
}

impl DispatchDataEventsResponse {
    pub fn new() -> DispatchDataEventsResponse {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor::<_, _, _>(
            "statusSet",
            |m: &DispatchDataEventsResponse| { &m.statusSet },
            |m: &mut DispatchDataEventsResponse| { &mut m.statusSet },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<DispatchDataEventsResponse>(
            "DispatchDataEventsResponse",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for DispatchDataEventsResponse {
    const NAME: &'static str = "DispatchDataEventsResponse";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            10 => key = is.read_string()?,
                            16 => value = is.read_enum_or_unknown()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.statusSet.insert(key, value);
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
        for (k, v) in &self.statusSet {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::string_size(1, &k);
            entry_size += ::protobuf::rt::int32_size(2, v.value());
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for (k, v) in &self.statusSet {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::string_size(1, &k);
            entry_size += ::protobuf::rt::int32_size(2, v.value());
            os.write_raw_varint32(10)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_string(1, &k)?;
            os.write_enum(2, ::protobuf::EnumOrUnknown::value(v))?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> DispatchDataEventsResponse {
        DispatchDataEventsResponse::new()
    }

    fn clear(&mut self) {
        self.statusSet.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static DispatchDataEventsResponse {
        static instance: ::protobuf::rt::Lazy<DispatchDataEventsResponse> = ::protobuf::rt::Lazy::new();
        instance.get(DispatchDataEventsResponse::new)
    }
}

impl ::protobuf::MessageFull for DispatchDataEventsResponse {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("DispatchDataEventsResponse").unwrap()).clone()
    }
}

impl ::std::fmt::Display for DispatchDataEventsResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DispatchDataEventsResponse {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:dataflow.StopDataflowRequest)
pub struct StopDataflowRequest {
    // message fields
    // @@protoc_insertion_point(field:dataflow.StopDataflowRequest.job_id)
    pub job_id: ::protobuf::MessageField<super::super::common::common::JobId>,
    // special fields
    // @@protoc_insertion_point(special_field:dataflow.StopDataflowRequest.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a StopDataflowRequest {
    fn default() -> &'a StopDataflowRequest {
        <StopDataflowRequest as ::protobuf::Message>::default_instance()
    }
}

impl StopDataflowRequest {
    pub fn new() -> StopDataflowRequest {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::common::JobId>(
            "job_id",
            |m: &StopDataflowRequest| { &m.job_id },
            |m: &mut StopDataflowRequest| { &mut m.job_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<StopDataflowRequest>(
            "StopDataflowRequest",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for StopDataflowRequest {
    const NAME: &'static str = "StopDataflowRequest";

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

    fn new() -> StopDataflowRequest {
        StopDataflowRequest::new()
    }

    fn clear(&mut self) {
        self.job_id.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static StopDataflowRequest {
        static instance: StopDataflowRequest = StopDataflowRequest {
            job_id: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for StopDataflowRequest {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("StopDataflowRequest").unwrap()).clone()
    }
}

impl ::std::fmt::Display for StopDataflowRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StopDataflowRequest {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:dataflow.StopDataflowResponse)
pub struct StopDataflowResponse {
    // message fields
    // @@protoc_insertion_point(field:dataflow.StopDataflowResponse.resp)
    pub resp: ::protobuf::MessageField<super::common::Response>,
    // special fields
    // @@protoc_insertion_point(special_field:dataflow.StopDataflowResponse.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a StopDataflowResponse {
    fn default() -> &'a StopDataflowResponse {
        <StopDataflowResponse as ::protobuf::Message>::default_instance()
    }
}

impl StopDataflowResponse {
    pub fn new() -> StopDataflowResponse {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::common::Response>(
            "resp",
            |m: &StopDataflowResponse| { &m.resp },
            |m: &mut StopDataflowResponse| { &mut m.resp },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<StopDataflowResponse>(
            "StopDataflowResponse",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for StopDataflowResponse {
    const NAME: &'static str = "StopDataflowResponse";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.resp)?;
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
        if let Some(v) = self.resp.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.resp.as_ref() {
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

    fn new() -> StopDataflowResponse {
        StopDataflowResponse::new()
    }

    fn clear(&mut self) {
        self.resp.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static StopDataflowResponse {
        static instance: StopDataflowResponse = StopDataflowResponse {
            resp: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for StopDataflowResponse {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("StopDataflowResponse").unwrap()).clone()
    }
}

impl ::std::fmt::Display for StopDataflowResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StopDataflowResponse {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:dataflow.CreateDataflowRequest)
pub struct CreateDataflowRequest {
    // message fields
    // @@protoc_insertion_point(field:dataflow.CreateDataflowRequest.job_id)
    pub job_id: ::protobuf::MessageField<common::common::JobId>,
    // @@protoc_insertion_point(field:dataflow.CreateDataflowRequest.dataflow)
    pub dataflow: ::protobuf::MessageField<stream::Dataflow>,
    // special fields
    // @@protoc_insertion_point(special_field:dataflow.CreateDataflowRequest.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CreateDataflowRequest {
    fn default() -> &'a CreateDataflowRequest {
        <CreateDataflowRequest as ::protobuf::Message>::default_instance()
    }
}

impl CreateDataflowRequest {
    pub fn new() -> CreateDataflowRequest {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::common::JobId>(
            "job_id",
            |m: &CreateDataflowRequest| { &m.job_id },
            |m: &mut CreateDataflowRequest| { &mut m.job_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::stream::Dataflow>(
            "dataflow",
            |m: &CreateDataflowRequest| { &m.dataflow },
            |m: &mut CreateDataflowRequest| { &mut m.dataflow },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<CreateDataflowRequest>(
            "CreateDataflowRequest",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for CreateDataflowRequest {
    const NAME: &'static str = "CreateDataflowRequest";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.job_id)?;
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.dataflow)?;
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
        if let Some(v) = self.dataflow.as_ref() {
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
        if let Some(v) = self.dataflow.as_ref() {
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

    fn new() -> CreateDataflowRequest {
        CreateDataflowRequest::new()
    }

    fn clear(&mut self) {
        self.job_id.clear();
        self.dataflow.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CreateDataflowRequest {
        static instance: CreateDataflowRequest = CreateDataflowRequest {
            job_id: ::protobuf::MessageField::none(),
            dataflow: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for CreateDataflowRequest {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("CreateDataflowRequest").unwrap()).clone()
    }
}

impl ::std::fmt::Display for CreateDataflowRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CreateDataflowRequest {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:dataflow.CreateDataflowResponse)
pub struct CreateDataflowResponse {
    // message fields
    // @@protoc_insertion_point(field:dataflow.CreateDataflowResponse.resp)
    pub resp: ::protobuf::MessageField<super::super::common::common::Response>,
    // special fields
    // @@protoc_insertion_point(special_field:dataflow.CreateDataflowResponse.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CreateDataflowResponse {
    fn default() -> &'a CreateDataflowResponse {
        <CreateDataflowResponse as ::protobuf::Message>::default_instance()
    }
}

impl CreateDataflowResponse {
    pub fn new() -> CreateDataflowResponse {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::common::Response>(
            "resp",
            |m: &CreateDataflowResponse| { &m.resp },
            |m: &mut CreateDataflowResponse| { &mut m.resp },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<CreateDataflowResponse>(
            "CreateDataflowResponse",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for CreateDataflowResponse {
    const NAME: &'static str = "CreateDataflowResponse";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.resp)?;
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
        if let Some(v) = self.resp.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.resp.as_ref() {
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

    fn new() -> CreateDataflowResponse {
        CreateDataflowResponse::new()
    }

    fn clear(&mut self) {
        self.resp.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CreateDataflowResponse {
        static instance: CreateDataflowResponse = CreateDataflowResponse {
            resp: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for CreateDataflowResponse {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("CreateDataflowResponse").unwrap()).clone()
    }
}

impl ::std::fmt::Display for CreateDataflowResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CreateDataflowResponse {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:dataflow.DispatchDataEventStatusEnum)
pub enum DispatchDataEventStatusEnum {
    // @@protoc_insertion_point(enum_value:dataflow.DispatchDataEventStatusEnum.DISPATCHING)
    DISPATCHING = 0,
    // @@protoc_insertion_point(enum_value:dataflow.DispatchDataEventStatusEnum.DONE)
    DONE = 1,
    // @@protoc_insertion_point(enum_value:dataflow.DispatchDataEventStatusEnum.FAILURE)
    FAILURE = 2,
}

impl ::protobuf::Enum for DispatchDataEventStatusEnum {
    const NAME: &'static str = "DispatchDataEventStatusEnum";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DispatchDataEventStatusEnum> {
        match value {
            0 => ::std::option::Option::Some(DispatchDataEventStatusEnum::DISPATCHING),
            1 => ::std::option::Option::Some(DispatchDataEventStatusEnum::DONE),
            2 => ::std::option::Option::Some(DispatchDataEventStatusEnum::FAILURE),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [DispatchDataEventStatusEnum] = &[
        DispatchDataEventStatusEnum::DISPATCHING,
        DispatchDataEventStatusEnum::DONE,
        DispatchDataEventStatusEnum::FAILURE,
    ];
}

impl ::protobuf::EnumFull for DispatchDataEventStatusEnum {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("DispatchDataEventStatusEnum").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for DispatchDataEventStatusEnum {
    fn default() -> Self {
        DispatchDataEventStatusEnum::DISPATCHING
    }
}

impl DispatchDataEventStatusEnum {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<DispatchDataEventStatusEnum>("DispatchDataEventStatusEnum")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x13worker/worker.proto\x12\x08dataflow\x1a\x12common/probe.proto\x1a\
    \x12common/event.proto\x1a\x13common/common.proto\x1a\x13common/stream.p\
    roto\"F\n\x19DispatchDataEventsRequest\x12)\n\x06events\x18\x01\x20\x03(\
    \x0b2\x11.common.DataEventR\x06events\"\xd4\x01\n\x1aDispatchDataEventsR\
    esponse\x12Q\n\tstatusSet\x18\x01\x20\x03(\x0b23.dataflow.DispatchDataEv\
    entsResponse.StatusSetEntryR\tstatusSet\x1ac\n\x0eStatusSetEntry\x12\x10\
    \n\x03key\x18\x01\x20\x01(\tR\x03key\x12;\n\x05value\x18\x02\x20\x01(\
    \x0e2%.dataflow.DispatchDataEventStatusEnumR\x05value:\x028\x01\";\n\x13\
    StopDataflowRequest\x12$\n\x06job_id\x18\x01\x20\x01(\x0b2\r.common.JobI\
    dR\x05jobId\"<\n\x14StopDataflowResponse\x12$\n\x04resp\x18\x01\x20\x01(\
    \x0b2\x10.common.ResponseR\x04resp\"k\n\x15CreateDataflowRequest\x12$\n\
    \x06job_id\x18\x01\x20\x01(\x0b2\r.common.JobIdR\x05jobId\x12,\n\x08data\
    flow\x18\x02\x20\x01(\x0b2\x10.common.DataflowR\x08dataflow\">\n\x16Crea\
    teDataflowResponse\x12$\n\x04resp\x18\x01\x20\x01(\x0b2\x10.common.Respo\
    nseR\x04resp*E\n\x1bDispatchDataEventStatusEnum\x12\x0f\n\x0bDISPATCHING\
    \x10\0\x12\x08\n\x04DONE\x10\x01\x12\x0b\n\x07FAILURE\x10\x022\xd2\x02\n\
    \rTaskWorkerApi\x126\n\x05Probe\x12\x14.common.ProbeRequest\x1a\x15.comm\
    on.ProbeResponse\"\0\x12a\n\x12DispatchDataEvents\x12#.dataflow.Dispatch\
    DataEventsRequest\x1a$.dataflow.DispatchDataEventsResponse\"\0\x12O\n\
    \x0cStopDataflow\x12\x1d.dataflow.StopDataflowRequest\x1a\x1e.dataflow.S\
    topDataflowResponse\"\0\x12U\n\x0eCreateDataflow\x12\x1f.dataflow.Create\
    DataflowRequest\x1a\x20.dataflow.CreateDataflowResponse\"\0B\x07Z\x05pro\
    toJ\x83\x08\n\x06\x12\x04\0\0.\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\
    \n\x01\x02\x12\x03\x02\0\x11\n\t\n\x02\x03\0\x12\x03\x03\0\x1c\n\t\n\x02\
    \x03\x01\x12\x03\x04\0\x1c\n\t\n\x02\x03\x02\x12\x03\x05\0\x1d\n\t\n\x02\
    \x03\x03\x12\x03\x06\0\x1d\n\x08\n\x01\x08\x12\x03\x08\0\x1c\n\t\n\x02\
    \x08\x0b\x12\x03\x08\0\x1c\n\n\n\x02\x06\0\x12\x04\n\0\x0f\x01\n\n\n\x03\
    \x06\0\x01\x12\x03\n\x08\x15\n\x0b\n\x04\x06\0\x02\0\x12\x03\x0b\x02B\n\
    \x0c\n\x05\x06\0\x02\0\x01\x12\x03\x0b\x06\x0b\n\x0c\n\x05\x06\0\x02\0\
    \x02\x12\x03\x0b\x0c\x1f\n\x0c\n\x05\x06\0\x02\0\x03\x12\x03\x0b*>\n\x0b\
    \n\x04\x06\0\x02\x01\x12\x03\x0c\x02Z\n\x0c\n\x05\x06\0\x02\x01\x01\x12\
    \x03\x0c\x06\x18\n\x0c\n\x05\x06\0\x02\x01\x02\x12\x03\x0c\x192\n\x0c\n\
    \x05\x06\0\x02\x01\x03\x12\x03\x0c=W\n\x0b\n\x04\x06\0\x02\x02\x12\x03\r\
    \x02I\n\x0c\n\x05\x06\0\x02\x02\x01\x12\x03\r\x06\x12\n\x0c\n\x05\x06\0\
    \x02\x02\x02\x12\x03\r\x13&\n\x0c\n\x05\x06\0\x02\x02\x03\x12\x03\r1E\n\
    \x0b\n\x04\x06\0\x02\x03\x12\x03\x0e\x02N\n\x0c\n\x05\x06\0\x02\x03\x01\
    \x12\x03\x0e\x06\x14\n\x0c\n\x05\x06\0\x02\x03\x02\x12\x03\x0e\x15*\n\
    \x0c\n\x05\x06\0\x02\x03\x03\x12\x03\x0e4J\n\n\n\x02\x04\0\x12\x04\x11\0\
    \x13\x01\n\n\n\x03\x04\0\x01\x12\x03\x11\x08!\n\x0b\n\x04\x04\0\x02\0\
    \x12\x03\x12\x02'\n\x0c\n\x05\x04\0\x02\0\x04\x12\x03\x12\x02\n\n\x0c\n\
    \x05\x04\0\x02\0\x06\x12\x03\x12\x0b\x1b\n\x0c\n\x05\x04\0\x02\0\x01\x12\
    \x03\x12\x1c\"\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x12%&\n\n\n\x02\x04\
    \x01\x12\x04\x15\0\x17\x01\n\n\n\x03\x04\x01\x01\x12\x03\x15\x08\"\n\x0b\
    \n\x04\x04\x01\x02\0\x12\x03\x16\x029\n\x0c\n\x05\x04\x01\x02\0\x06\x12\
    \x03\x16\x02*\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x16+4\n\x0c\n\x05\
    \x04\x01\x02\0\x03\x12\x03\x1678\n\n\n\x02\x05\0\x12\x04\x19\0\x1d\x01\n\
    \n\n\x03\x05\0\x01\x12\x03\x19\x05\x20\n\x0b\n\x04\x05\0\x02\0\x12\x03\
    \x1a\x02\x12\n\x0c\n\x05\x05\0\x02\0\x01\x12\x03\x1a\x02\r\n\x0c\n\x05\
    \x05\0\x02\0\x02\x12\x03\x1a\x10\x11\n\x0b\n\x04\x05\0\x02\x01\x12\x03\
    \x1b\x02\x0b\n\x0c\n\x05\x05\0\x02\x01\x01\x12\x03\x1b\x02\x06\n\x0c\n\
    \x05\x05\0\x02\x01\x02\x12\x03\x1b\t\n\n\x0b\n\x04\x05\0\x02\x02\x12\x03\
    \x1c\x02\x0e\n\x0c\n\x05\x05\0\x02\x02\x01\x12\x03\x1c\x02\t\n\x0c\n\x05\
    \x05\0\x02\x02\x02\x12\x03\x1c\x0c\r\n\n\n\x02\x04\x02\x12\x04\x1f\0!\
    \x01\n\n\n\x03\x04\x02\x01\x12\x03\x1f\x08\x1b\n\x0b\n\x04\x04\x02\x02\0\
    \x12\x03\x20\x02\x1a\n\x0c\n\x05\x04\x02\x02\0\x06\x12\x03\x20\x02\x0e\n\
    \x0c\n\x05\x04\x02\x02\0\x01\x12\x03\x20\x0f\x15\n\x0c\n\x05\x04\x02\x02\
    \0\x03\x12\x03\x20\x18\x19\n\n\n\x02\x04\x03\x12\x04#\0%\x01\n\n\n\x03\
    \x04\x03\x01\x12\x03#\x08\x1c\n\x0b\n\x04\x04\x03\x02\0\x12\x03$\x02\x1b\
    \n\x0c\n\x05\x04\x03\x02\0\x06\x12\x03$\x02\x11\n\x0c\n\x05\x04\x03\x02\
    \0\x01\x12\x03$\x12\x16\n\x0c\n\x05\x04\x03\x02\0\x03\x12\x03$\x19\x1a\n\
    \n\n\x02\x04\x04\x12\x04'\0*\x01\n\n\n\x03\x04\x04\x01\x12\x03'\x08\x1d\
    \n\x0b\n\x04\x04\x04\x02\0\x12\x03(\x02\x1a\n\x0c\n\x05\x04\x04\x02\0\
    \x06\x12\x03(\x02\x0e\n\x0c\n\x05\x04\x04\x02\0\x01\x12\x03(\x0f\x15\n\
    \x0c\n\x05\x04\x04\x02\0\x03\x12\x03(\x18\x19\n\x0b\n\x04\x04\x04\x02\
    \x01\x12\x03)\x02\x1f\n\x0c\n\x05\x04\x04\x02\x01\x06\x12\x03)\x02\x11\n\
    \x0c\n\x05\x04\x04\x02\x01\x01\x12\x03)\x12\x1a\n\x0c\n\x05\x04\x04\x02\
    \x01\x03\x12\x03)\x1d\x1e\n\n\n\x02\x04\x05\x12\x04,\0.\x01\n\n\n\x03\
    \x04\x05\x01\x12\x03,\x08\x1e\n\x0b\n\x04\x04\x05\x02\0\x12\x03-\x02\x1b\
    \n\x0c\n\x05\x04\x05\x02\0\x06\x12\x03-\x02\x11\n\x0c\n\x05\x04\x05\x02\
    \0\x01\x12\x03-\x12\x16\n\x0c\n\x05\x04\x05\x02\0\x03\x12\x03-\x19\x1ab\
    \x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(4);
            deps.push(probe::file_descriptor().clone());
            deps.push(event::file_descriptor().clone());
            deps.push(common::common::file_descriptor().clone());
            deps.push(stream::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(6);
            messages.push(DispatchDataEventsRequest::generated_message_descriptor_data());
            messages.push(DispatchDataEventsResponse::generated_message_descriptor_data());
            messages.push(StopDataflowRequest::generated_message_descriptor_data());
            messages.push(StopDataflowResponse::generated_message_descriptor_data());
            messages.push(CreateDataflowRequest::generated_message_descriptor_data());
            messages.push(CreateDataflowResponse::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(DispatchDataEventStatusEnum::generated_enum_descriptor_data());
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
