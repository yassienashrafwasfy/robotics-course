#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};




// Corresponds to turtlebot3_msgs__srv__Sound_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Sound_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub value: u8,

}



impl Default for Sound_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::Sound_Request::default())
  }
}

impl rosidl_runtime_rs::Message for Sound_Request {
  type RmwMsg = super::srv::rmw::Sound_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        value: msg.value,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      value: msg.value,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      value: msg.value,
    }
  }
}


// Corresponds to turtlebot3_msgs__srv__Sound_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Sound_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,

}



impl Default for Sound_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::Sound_Response::default())
  }
}

impl rosidl_runtime_rs::Message for Sound_Response {
  type RmwMsg = super::srv::rmw::Sound_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
    }
  }
}


// Corresponds to turtlebot3_msgs__srv__Dqn_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::Dqn_Request::default())
  }
}

impl rosidl_runtime_rs::Message for Dqn_Request {
  type RmwMsg = super::srv::rmw::Dqn_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        action: msg.action,
        init: msg.init,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      action: msg.action,
      init: msg.init,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      action: msg.action,
      init: msg.init,
    }
  }
}


// Corresponds to turtlebot3_msgs__srv__Dqn_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Dqn_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub state: Vec<f32>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub reward: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub done: bool,

}



impl Default for Dqn_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::Dqn_Response::default())
  }
}

impl rosidl_runtime_rs::Message for Dqn_Response {
  type RmwMsg = super::srv::rmw::Dqn_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        state: msg.state.into(),
        reward: msg.reward,
        done: msg.done,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        state: msg.state.as_slice().into(),
      reward: msg.reward,
      done: msg.done,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      state: msg.state
          .into_iter()
          .collect(),
      reward: msg.reward,
      done: msg.done,
    }
  }
}


// Corresponds to turtlebot3_msgs__srv__DrlStep_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DrlStep_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub action: Vec<f32>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub previous_action: Vec<f32>,

}



impl Default for DrlStep_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::DrlStep_Request::default())
  }
}

impl rosidl_runtime_rs::Message for DrlStep_Request {
  type RmwMsg = super::srv::rmw::DrlStep_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        action: msg.action.into(),
        previous_action: msg.previous_action.into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        action: msg.action.as_slice().into(),
        previous_action: msg.previous_action.as_slice().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      action: msg.action
          .into_iter()
          .collect(),
      previous_action: msg.previous_action
          .into_iter()
          .collect(),
    }
  }
}


// Corresponds to turtlebot3_msgs__srv__DrlStep_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DrlStep_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub state: Vec<f32>,


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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::DrlStep_Response::default())
  }
}

impl rosidl_runtime_rs::Message for DrlStep_Response {
  type RmwMsg = super::srv::rmw::DrlStep_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        state: msg.state.into(),
        reward: msg.reward,
        done: msg.done,
        success: msg.success,
        distance_traveled: msg.distance_traveled,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        state: msg.state.as_slice().into(),
      reward: msg.reward,
      done: msg.done,
      success: msg.success,
      distance_traveled: msg.distance_traveled,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      state: msg.state
          .into_iter()
          .collect(),
      reward: msg.reward,
      done: msg.done,
      success: msg.success,
      distance_traveled: msg.distance_traveled,
    }
  }
}


// Corresponds to turtlebot3_msgs__srv__Goal_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Goal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub empty: bool,

}



impl Default for Goal_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::Goal_Request::default())
  }
}

impl rosidl_runtime_rs::Message for Goal_Request {
  type RmwMsg = super::srv::rmw::Goal_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        empty: msg.empty,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      empty: msg.empty,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      empty: msg.empty,
    }
  }
}


// Corresponds to turtlebot3_msgs__srv__Goal_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Goal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub new_goal: bool,

}



impl Default for Goal_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::Goal_Response::default())
  }
}

impl rosidl_runtime_rs::Message for Goal_Response {
  type RmwMsg = super::srv::rmw::Goal_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        new_goal: msg.new_goal,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      new_goal: msg.new_goal,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      new_goal: msg.new_goal,
    }
  }
}


// Corresponds to turtlebot3_msgs__srv__RingGoal_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::RingGoal_Request::default())
  }
}

impl rosidl_runtime_rs::Message for RingGoal_Request {
  type RmwMsg = super::srv::rmw::RingGoal_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_pose_x: msg.robot_pose_x,
        robot_pose_y: msg.robot_pose_y,
        radius: msg.radius,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      robot_pose_x: msg.robot_pose_x,
      robot_pose_y: msg.robot_pose_y,
      radius: msg.radius,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      robot_pose_x: msg.robot_pose_x,
      robot_pose_y: msg.robot_pose_y,
      radius: msg.radius,
    }
  }
}


// Corresponds to turtlebot3_msgs__srv__RingGoal_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RingGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for RingGoal_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::RingGoal_Response::default())
  }
}

impl rosidl_runtime_rs::Message for RingGoal_Response {
  type RmwMsg = super::srv::rmw::RingGoal_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
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


