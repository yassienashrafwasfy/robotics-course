#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



#[link(name = "turtlebot3_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__turtlebot3_msgs__srv__Sound_Request() -> *const std::ffi::c_void;
}

#[link(name = "turtlebot3_msgs__rosidl_generator_c")]
extern "C" {
    fn turtlebot3_msgs__srv__Sound_Request__init(msg: *mut Sound_Request) -> bool;
    fn turtlebot3_msgs__srv__Sound_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Sound_Request>, size: usize) -> bool;
    fn turtlebot3_msgs__srv__Sound_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Sound_Request>);
    fn turtlebot3_msgs__srv__Sound_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Sound_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<Sound_Request>) -> bool;
}

// Corresponds to turtlebot3_msgs__srv__Sound_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Sound_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub value: u8,

}



impl Default for Sound_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !turtlebot3_msgs__srv__Sound_Request__init(&mut msg as *mut _) {
        panic!("Call to turtlebot3_msgs__srv__Sound_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Sound_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { turtlebot3_msgs__srv__Sound_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { turtlebot3_msgs__srv__Sound_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { turtlebot3_msgs__srv__Sound_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Sound_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Sound_Request where Self: Sized {
  const TYPE_NAME: &'static str = "turtlebot3_msgs/srv/Sound_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__turtlebot3_msgs__srv__Sound_Request() }
  }
}


#[link(name = "turtlebot3_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__turtlebot3_msgs__srv__Sound_Response() -> *const std::ffi::c_void;
}

#[link(name = "turtlebot3_msgs__rosidl_generator_c")]
extern "C" {
    fn turtlebot3_msgs__srv__Sound_Response__init(msg: *mut Sound_Response) -> bool;
    fn turtlebot3_msgs__srv__Sound_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Sound_Response>, size: usize) -> bool;
    fn turtlebot3_msgs__srv__Sound_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Sound_Response>);
    fn turtlebot3_msgs__srv__Sound_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Sound_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<Sound_Response>) -> bool;
}

// Corresponds to turtlebot3_msgs__srv__Sound_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Sound_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for Sound_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !turtlebot3_msgs__srv__Sound_Response__init(&mut msg as *mut _) {
        panic!("Call to turtlebot3_msgs__srv__Sound_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Sound_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { turtlebot3_msgs__srv__Sound_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { turtlebot3_msgs__srv__Sound_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { turtlebot3_msgs__srv__Sound_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Sound_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Sound_Response where Self: Sized {
  const TYPE_NAME: &'static str = "turtlebot3_msgs/srv/Sound_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__turtlebot3_msgs__srv__Sound_Response() }
  }
}


#[link(name = "turtlebot3_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__turtlebot3_msgs__srv__Dqn_Request() -> *const std::ffi::c_void;
}

#[link(name = "turtlebot3_msgs__rosidl_generator_c")]
extern "C" {
    fn turtlebot3_msgs__srv__Dqn_Request__init(msg: *mut Dqn_Request) -> bool;
    fn turtlebot3_msgs__srv__Dqn_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Dqn_Request>, size: usize) -> bool;
    fn turtlebot3_msgs__srv__Dqn_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Dqn_Request>);
    fn turtlebot3_msgs__srv__Dqn_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Dqn_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<Dqn_Request>) -> bool;
}

// Corresponds to turtlebot3_msgs__srv__Dqn_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Dqn_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub action: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub init: bool,

}



impl Default for Dqn_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !turtlebot3_msgs__srv__Dqn_Request__init(&mut msg as *mut _) {
        panic!("Call to turtlebot3_msgs__srv__Dqn_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Dqn_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { turtlebot3_msgs__srv__Dqn_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { turtlebot3_msgs__srv__Dqn_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { turtlebot3_msgs__srv__Dqn_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Dqn_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Dqn_Request where Self: Sized {
  const TYPE_NAME: &'static str = "turtlebot3_msgs/srv/Dqn_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__turtlebot3_msgs__srv__Dqn_Request() }
  }
}


#[link(name = "turtlebot3_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__turtlebot3_msgs__srv__Dqn_Response() -> *const std::ffi::c_void;
}

#[link(name = "turtlebot3_msgs__rosidl_generator_c")]
extern "C" {
    fn turtlebot3_msgs__srv__Dqn_Response__init(msg: *mut Dqn_Response) -> bool;
    fn turtlebot3_msgs__srv__Dqn_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Dqn_Response>, size: usize) -> bool;
    fn turtlebot3_msgs__srv__Dqn_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Dqn_Response>);
    fn turtlebot3_msgs__srv__Dqn_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Dqn_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<Dqn_Response>) -> bool;
}

// Corresponds to turtlebot3_msgs__srv__Dqn_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Dqn_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub state: rosidl_runtime_rs::Sequence<f32>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub reward: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub done: bool,

}



impl Default for Dqn_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !turtlebot3_msgs__srv__Dqn_Response__init(&mut msg as *mut _) {
        panic!("Call to turtlebot3_msgs__srv__Dqn_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Dqn_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { turtlebot3_msgs__srv__Dqn_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { turtlebot3_msgs__srv__Dqn_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { turtlebot3_msgs__srv__Dqn_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Dqn_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Dqn_Response where Self: Sized {
  const TYPE_NAME: &'static str = "turtlebot3_msgs/srv/Dqn_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__turtlebot3_msgs__srv__Dqn_Response() }
  }
}


#[link(name = "turtlebot3_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__turtlebot3_msgs__srv__DrlStep_Request() -> *const std::ffi::c_void;
}

#[link(name = "turtlebot3_msgs__rosidl_generator_c")]
extern "C" {
    fn turtlebot3_msgs__srv__DrlStep_Request__init(msg: *mut DrlStep_Request) -> bool;
    fn turtlebot3_msgs__srv__DrlStep_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DrlStep_Request>, size: usize) -> bool;
    fn turtlebot3_msgs__srv__DrlStep_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DrlStep_Request>);
    fn turtlebot3_msgs__srv__DrlStep_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DrlStep_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<DrlStep_Request>) -> bool;
}

// Corresponds to turtlebot3_msgs__srv__DrlStep_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DrlStep_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub action: rosidl_runtime_rs::Sequence<f32>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub previous_action: rosidl_runtime_rs::Sequence<f32>,

}



impl Default for DrlStep_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !turtlebot3_msgs__srv__DrlStep_Request__init(&mut msg as *mut _) {
        panic!("Call to turtlebot3_msgs__srv__DrlStep_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DrlStep_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { turtlebot3_msgs__srv__DrlStep_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { turtlebot3_msgs__srv__DrlStep_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { turtlebot3_msgs__srv__DrlStep_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DrlStep_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DrlStep_Request where Self: Sized {
  const TYPE_NAME: &'static str = "turtlebot3_msgs/srv/DrlStep_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__turtlebot3_msgs__srv__DrlStep_Request() }
  }
}


#[link(name = "turtlebot3_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__turtlebot3_msgs__srv__DrlStep_Response() -> *const std::ffi::c_void;
}

#[link(name = "turtlebot3_msgs__rosidl_generator_c")]
extern "C" {
    fn turtlebot3_msgs__srv__DrlStep_Response__init(msg: *mut DrlStep_Response) -> bool;
    fn turtlebot3_msgs__srv__DrlStep_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DrlStep_Response>, size: usize) -> bool;
    fn turtlebot3_msgs__srv__DrlStep_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DrlStep_Response>);
    fn turtlebot3_msgs__srv__DrlStep_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DrlStep_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<DrlStep_Response>) -> bool;
}

// Corresponds to turtlebot3_msgs__srv__DrlStep_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DrlStep_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub state: rosidl_runtime_rs::Sequence<f32>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub reward: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub done: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub success: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub distance_traveled: f32,

}



impl Default for DrlStep_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !turtlebot3_msgs__srv__DrlStep_Response__init(&mut msg as *mut _) {
        panic!("Call to turtlebot3_msgs__srv__DrlStep_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DrlStep_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { turtlebot3_msgs__srv__DrlStep_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { turtlebot3_msgs__srv__DrlStep_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { turtlebot3_msgs__srv__DrlStep_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DrlStep_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DrlStep_Response where Self: Sized {
  const TYPE_NAME: &'static str = "turtlebot3_msgs/srv/DrlStep_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__turtlebot3_msgs__srv__DrlStep_Response() }
  }
}


#[link(name = "turtlebot3_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__turtlebot3_msgs__srv__Goal_Request() -> *const std::ffi::c_void;
}

#[link(name = "turtlebot3_msgs__rosidl_generator_c")]
extern "C" {
    fn turtlebot3_msgs__srv__Goal_Request__init(msg: *mut Goal_Request) -> bool;
    fn turtlebot3_msgs__srv__Goal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Goal_Request>, size: usize) -> bool;
    fn turtlebot3_msgs__srv__Goal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Goal_Request>);
    fn turtlebot3_msgs__srv__Goal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Goal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<Goal_Request>) -> bool;
}

// Corresponds to turtlebot3_msgs__srv__Goal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Goal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub empty: bool,

}



impl Default for Goal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !turtlebot3_msgs__srv__Goal_Request__init(&mut msg as *mut _) {
        panic!("Call to turtlebot3_msgs__srv__Goal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Goal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { turtlebot3_msgs__srv__Goal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { turtlebot3_msgs__srv__Goal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { turtlebot3_msgs__srv__Goal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Goal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Goal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "turtlebot3_msgs/srv/Goal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__turtlebot3_msgs__srv__Goal_Request() }
  }
}


#[link(name = "turtlebot3_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__turtlebot3_msgs__srv__Goal_Response() -> *const std::ffi::c_void;
}

#[link(name = "turtlebot3_msgs__rosidl_generator_c")]
extern "C" {
    fn turtlebot3_msgs__srv__Goal_Response__init(msg: *mut Goal_Response) -> bool;
    fn turtlebot3_msgs__srv__Goal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Goal_Response>, size: usize) -> bool;
    fn turtlebot3_msgs__srv__Goal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Goal_Response>);
    fn turtlebot3_msgs__srv__Goal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Goal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<Goal_Response>) -> bool;
}

// Corresponds to turtlebot3_msgs__srv__Goal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Goal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub new_goal: bool,

}



impl Default for Goal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !turtlebot3_msgs__srv__Goal_Response__init(&mut msg as *mut _) {
        panic!("Call to turtlebot3_msgs__srv__Goal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Goal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { turtlebot3_msgs__srv__Goal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { turtlebot3_msgs__srv__Goal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { turtlebot3_msgs__srv__Goal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Goal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Goal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "turtlebot3_msgs/srv/Goal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__turtlebot3_msgs__srv__Goal_Response() }
  }
}


#[link(name = "turtlebot3_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__turtlebot3_msgs__srv__RingGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "turtlebot3_msgs__rosidl_generator_c")]
extern "C" {
    fn turtlebot3_msgs__srv__RingGoal_Request__init(msg: *mut RingGoal_Request) -> bool;
    fn turtlebot3_msgs__srv__RingGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RingGoal_Request>, size: usize) -> bool;
    fn turtlebot3_msgs__srv__RingGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RingGoal_Request>);
    fn turtlebot3_msgs__srv__RingGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RingGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<RingGoal_Request>) -> bool;
}

// Corresponds to turtlebot3_msgs__srv__RingGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RingGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_pose_x: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_pose_y: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub radius: f32,

}



impl Default for RingGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !turtlebot3_msgs__srv__RingGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to turtlebot3_msgs__srv__RingGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RingGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { turtlebot3_msgs__srv__RingGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { turtlebot3_msgs__srv__RingGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { turtlebot3_msgs__srv__RingGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RingGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RingGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "turtlebot3_msgs/srv/RingGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__turtlebot3_msgs__srv__RingGoal_Request() }
  }
}


#[link(name = "turtlebot3_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__turtlebot3_msgs__srv__RingGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "turtlebot3_msgs__rosidl_generator_c")]
extern "C" {
    fn turtlebot3_msgs__srv__RingGoal_Response__init(msg: *mut RingGoal_Response) -> bool;
    fn turtlebot3_msgs__srv__RingGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RingGoal_Response>, size: usize) -> bool;
    fn turtlebot3_msgs__srv__RingGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RingGoal_Response>);
    fn turtlebot3_msgs__srv__RingGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RingGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<RingGoal_Response>) -> bool;
}

// Corresponds to turtlebot3_msgs__srv__RingGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RingGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for RingGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !turtlebot3_msgs__srv__RingGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to turtlebot3_msgs__srv__RingGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RingGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { turtlebot3_msgs__srv__RingGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { turtlebot3_msgs__srv__RingGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { turtlebot3_msgs__srv__RingGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RingGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RingGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "turtlebot3_msgs/srv/RingGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__turtlebot3_msgs__srv__RingGoal_Response() }
  }
}






#[link(name = "turtlebot3_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__turtlebot3_msgs__srv__Sound() -> *const std::ffi::c_void;
}

// Corresponds to turtlebot3_msgs__srv__Sound
#[allow(missing_docs, non_camel_case_types)]
pub struct Sound;

impl rosidl_runtime_rs::Service for Sound {
    type Request = Sound_Request;
    type Response = Sound_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__turtlebot3_msgs__srv__Sound() }
    }
}




#[link(name = "turtlebot3_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__turtlebot3_msgs__srv__Dqn() -> *const std::ffi::c_void;
}

// Corresponds to turtlebot3_msgs__srv__Dqn
#[allow(missing_docs, non_camel_case_types)]
pub struct Dqn;

impl rosidl_runtime_rs::Service for Dqn {
    type Request = Dqn_Request;
    type Response = Dqn_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__turtlebot3_msgs__srv__Dqn() }
    }
}




#[link(name = "turtlebot3_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__turtlebot3_msgs__srv__DrlStep() -> *const std::ffi::c_void;
}

// Corresponds to turtlebot3_msgs__srv__DrlStep
#[allow(missing_docs, non_camel_case_types)]
pub struct DrlStep;

impl rosidl_runtime_rs::Service for DrlStep {
    type Request = DrlStep_Request;
    type Response = DrlStep_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__turtlebot3_msgs__srv__DrlStep() }
    }
}




#[link(name = "turtlebot3_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__turtlebot3_msgs__srv__Goal() -> *const std::ffi::c_void;
}

// Corresponds to turtlebot3_msgs__srv__Goal
#[allow(missing_docs, non_camel_case_types)]
pub struct Goal;

impl rosidl_runtime_rs::Service for Goal {
    type Request = Goal_Request;
    type Response = Goal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__turtlebot3_msgs__srv__Goal() }
    }
}




#[link(name = "turtlebot3_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__turtlebot3_msgs__srv__RingGoal() -> *const std::ffi::c_void;
}

// Corresponds to turtlebot3_msgs__srv__RingGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct RingGoal;

impl rosidl_runtime_rs::Service for RingGoal {
    type Request = RingGoal_Request;
    type Response = RingGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__turtlebot3_msgs__srv__RingGoal() }
    }
}


