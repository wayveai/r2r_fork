  pub mod msg {
    use super::super::*;

                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Color {

                              pub r: u8,
pub g: u8,
pub b: u8,

                          }

                          impl WrappedTypesupport for Color { 

            type CStruct = turtlesim__msg__Color; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__turtlesim__msg__Color() }
            }

            fn create_msg() -> *mut turtlesim__msg__Color {

                unsafe { turtlesim__msg__Color__create() }

            }

            fn destroy_msg(msg: *mut turtlesim__msg__Color) -> () {

                unsafe { turtlesim__msg__Color__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Color {
  Color {
r: msg.r,
g: msg.g,
b: msg.b,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.r = self.r;
msg.g = self.g;
msg.b = self.b;
}



        }


                          
                          impl Default for Color {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Color>::new();
                                  Color::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Pose {

                              pub x: f32,
pub y: f32,
pub theta: f32,
pub linear_velocity: f32,
pub angular_velocity: f32,

                          }

                          impl WrappedTypesupport for Pose { 

            type CStruct = turtlesim__msg__Pose; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__turtlesim__msg__Pose() }
            }

            fn create_msg() -> *mut turtlesim__msg__Pose {

                unsafe { turtlesim__msg__Pose__create() }

            }

            fn destroy_msg(msg: *mut turtlesim__msg__Pose) -> () {

                unsafe { turtlesim__msg__Pose__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Pose {
  Pose {
x: msg.x,
y: msg.y,
theta: msg.theta,
linear_velocity: msg.linear_velocity,
angular_velocity: msg.angular_velocity,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.x = self.x;
msg.y = self.y;
msg.theta = self.theta;
msg.linear_velocity = self.linear_velocity;
msg.angular_velocity = self.angular_velocity;
}



        }


                          
                          impl Default for Pose {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Pose>::new();
                                  Pose::from_native(&msg_native)
                              }
                          }
             


                      }
  pub mod srv {
#[allow(non_snake_case)]
    pub mod Kill {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__turtlesim__srv__Kill()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              pub name: std::string::String,

                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = turtlesim__srv__Kill_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__turtlesim__srv__Kill_Request() }
            }

            fn create_msg() -> *mut turtlesim__srv__Kill_Request {

                unsafe { turtlesim__srv__Kill_Request__create() }

            }

            fn destroy_msg(msg: *mut turtlesim__srv__Kill_Request) -> () {

                unsafe { turtlesim__srv__Kill_Request__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Request {
  Request {
name: msg.name.to_str().to_owned(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.name.assign(&self.name);
}



        }


                          
                          impl Default for Request {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Request>::new();
                                  Request::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Response {

                              
                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = turtlesim__srv__Kill_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__turtlesim__srv__Kill_Response() }
            }

            fn create_msg() -> *mut turtlesim__srv__Kill_Response {

                unsafe { turtlesim__srv__Kill_Response__create() }

            }

            fn destroy_msg(msg: *mut turtlesim__srv__Kill_Response) -> () {

                unsafe { turtlesim__srv__Kill_Response__destroy(msg) };

            }

            fn from_native(_msg: &Self::CStruct) -> Response {
  Response {
      }
    }



            fn copy_to_native(&self, _msg: &mut Self::CStruct) {}



        }


                          
                          impl Default for Response {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Response>::new();
                                  Response::from_native(&msg_native)
                              }
                          }
             


                        }
#[allow(non_snake_case)]
    pub mod SetPen {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__turtlesim__srv__SetPen()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              pub r: u8,
pub g: u8,
pub b: u8,
pub width: u8,
pub off: u8,

                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = turtlesim__srv__SetPen_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__turtlesim__srv__SetPen_Request() }
            }

            fn create_msg() -> *mut turtlesim__srv__SetPen_Request {

                unsafe { turtlesim__srv__SetPen_Request__create() }

            }

            fn destroy_msg(msg: *mut turtlesim__srv__SetPen_Request) -> () {

                unsafe { turtlesim__srv__SetPen_Request__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Request {
  Request {
r: msg.r,
g: msg.g,
b: msg.b,
width: msg.width,
off: msg.off,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.r = self.r;
msg.g = self.g;
msg.b = self.b;
msg.width = self.width;
msg.off = self.off;
}



        }


                          
                          impl Default for Request {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Request>::new();
                                  Request::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Response {

                              
                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = turtlesim__srv__SetPen_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__turtlesim__srv__SetPen_Response() }
            }

            fn create_msg() -> *mut turtlesim__srv__SetPen_Response {

                unsafe { turtlesim__srv__SetPen_Response__create() }

            }

            fn destroy_msg(msg: *mut turtlesim__srv__SetPen_Response) -> () {

                unsafe { turtlesim__srv__SetPen_Response__destroy(msg) };

            }

            fn from_native(_msg: &Self::CStruct) -> Response {
  Response {
      }
    }



            fn copy_to_native(&self, _msg: &mut Self::CStruct) {}



        }


                          
                          impl Default for Response {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Response>::new();
                                  Response::from_native(&msg_native)
                              }
                          }
             


                        }
#[allow(non_snake_case)]
    pub mod Spawn {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__turtlesim__srv__Spawn()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              pub x: f32,
pub y: f32,
pub theta: f32,
pub name: std::string::String,

                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = turtlesim__srv__Spawn_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__turtlesim__srv__Spawn_Request() }
            }

            fn create_msg() -> *mut turtlesim__srv__Spawn_Request {

                unsafe { turtlesim__srv__Spawn_Request__create() }

            }

            fn destroy_msg(msg: *mut turtlesim__srv__Spawn_Request) -> () {

                unsafe { turtlesim__srv__Spawn_Request__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Request {
  Request {
x: msg.x,
y: msg.y,
theta: msg.theta,
name: msg.name.to_str().to_owned(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.x = self.x;
msg.y = self.y;
msg.theta = self.theta;
msg.name.assign(&self.name);
}



        }


                          
                          impl Default for Request {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Request>::new();
                                  Request::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Response {

                              pub name: std::string::String,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = turtlesim__srv__Spawn_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__turtlesim__srv__Spawn_Response() }
            }

            fn create_msg() -> *mut turtlesim__srv__Spawn_Response {

                unsafe { turtlesim__srv__Spawn_Response__create() }

            }

            fn destroy_msg(msg: *mut turtlesim__srv__Spawn_Response) -> () {

                unsafe { turtlesim__srv__Spawn_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
name: msg.name.to_str().to_owned(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.name.assign(&self.name);
}



        }


                          
                          impl Default for Response {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Response>::new();
                                  Response::from_native(&msg_native)
                              }
                          }
             


                        }
#[allow(non_snake_case)]
    pub mod TeleportAbsolute {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__turtlesim__srv__TeleportAbsolute()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              pub x: f32,
pub y: f32,
pub theta: f32,

                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = turtlesim__srv__TeleportAbsolute_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__turtlesim__srv__TeleportAbsolute_Request() }
            }

            fn create_msg() -> *mut turtlesim__srv__TeleportAbsolute_Request {

                unsafe { turtlesim__srv__TeleportAbsolute_Request__create() }

            }

            fn destroy_msg(msg: *mut turtlesim__srv__TeleportAbsolute_Request) -> () {

                unsafe { turtlesim__srv__TeleportAbsolute_Request__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Request {
  Request {
x: msg.x,
y: msg.y,
theta: msg.theta,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.x = self.x;
msg.y = self.y;
msg.theta = self.theta;
}



        }


                          
                          impl Default for Request {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Request>::new();
                                  Request::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Response {

                              
                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = turtlesim__srv__TeleportAbsolute_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__turtlesim__srv__TeleportAbsolute_Response() }
            }

            fn create_msg() -> *mut turtlesim__srv__TeleportAbsolute_Response {

                unsafe { turtlesim__srv__TeleportAbsolute_Response__create() }

            }

            fn destroy_msg(msg: *mut turtlesim__srv__TeleportAbsolute_Response) -> () {

                unsafe { turtlesim__srv__TeleportAbsolute_Response__destroy(msg) };

            }

            fn from_native(_msg: &Self::CStruct) -> Response {
  Response {
      }
    }



            fn copy_to_native(&self, _msg: &mut Self::CStruct) {}



        }


                          
                          impl Default for Response {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Response>::new();
                                  Response::from_native(&msg_native)
                              }
                          }
             


                        }
#[allow(non_snake_case)]
    pub mod TeleportRelative {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__turtlesim__srv__TeleportRelative()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              pub linear: f32,
pub angular: f32,

                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = turtlesim__srv__TeleportRelative_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__turtlesim__srv__TeleportRelative_Request() }
            }

            fn create_msg() -> *mut turtlesim__srv__TeleportRelative_Request {

                unsafe { turtlesim__srv__TeleportRelative_Request__create() }

            }

            fn destroy_msg(msg: *mut turtlesim__srv__TeleportRelative_Request) -> () {

                unsafe { turtlesim__srv__TeleportRelative_Request__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Request {
  Request {
linear: msg.linear,
angular: msg.angular,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.linear = self.linear;
msg.angular = self.angular;
}



        }


                          
                          impl Default for Request {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Request>::new();
                                  Request::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Response {

                              
                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = turtlesim__srv__TeleportRelative_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__turtlesim__srv__TeleportRelative_Response() }
            }

            fn create_msg() -> *mut turtlesim__srv__TeleportRelative_Response {

                unsafe { turtlesim__srv__TeleportRelative_Response__create() }

            }

            fn destroy_msg(msg: *mut turtlesim__srv__TeleportRelative_Response) -> () {

                unsafe { turtlesim__srv__TeleportRelative_Response__destroy(msg) };

            }

            fn from_native(_msg: &Self::CStruct) -> Response {
  Response {
      }
    }



            fn copy_to_native(&self, _msg: &mut Self::CStruct) {}



        }


                          
                          impl Default for Response {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Response>::new();
                                  Response::from_native(&msg_native)
                              }
                          }
             


                        }
  }
  pub mod action {
#[allow(non_snake_case)]
    pub mod RotateAbsolute {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Action();
        impl WrappedActionTypeSupport for Action {
            type Goal = Goal;
            type Result = Result;
            type Feedback = Feedback;

            // internal structs
            type FeedbackMessage = FeedbackMessage;
            type SendGoal = SendGoal::Service;
            type GetResult = GetResult::Service;

            fn get_ts() -> &'static rosidl_action_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_action_type_support_handle__turtlesim__action__RotateAbsolute()
                }
            }

            fn make_goal_request_msg(goal_id: unique_identifier_msgs::msg::UUID, goal: Goal) -> SendGoal::Request {
                SendGoal::Request {
                     goal_id,
                     goal
                }
            }

            fn make_goal_response_msg(accepted: bool, stamp: builtin_interfaces::msg::Time) -> SendGoal::Response {
                SendGoal::Response {
                     accepted,
                     stamp
                }
            }

            fn make_feedback_msg(goal_id: unique_identifier_msgs::msg::UUID, feedback: Feedback) -> FeedbackMessage {
                FeedbackMessage {
                     goal_id,
                     feedback
                }
            }

            fn make_result_request_msg(goal_id: unique_identifier_msgs::msg::UUID) -> GetResult::Request {
                GetResult::Request {
                     goal_id,
                }
            }

            fn make_result_response_msg(status: i8, result: Result) -> GetResult::Response {
                GetResult::Response {
                     status,
                     result,
                }
            }

            fn destructure_goal_request_msg(msg: SendGoal::Request) -> (unique_identifier_msgs::msg::UUID, Goal) {
                (msg.goal_id, msg.goal)
            }

            fn destructure_goal_response_msg(msg: SendGoal::Response) -> (bool, builtin_interfaces::msg::Time) {
                (msg.accepted, msg.stamp)
            }

            fn destructure_feedback_msg(msg: FeedbackMessage) -> (unique_identifier_msgs::msg::UUID, Feedback) {
                (msg.goal_id, msg.feedback)
            }

            fn destructure_result_response_msg(msg: GetResult::Response) -> (i8, Result) {
                (msg.status, msg.result)
            }

            fn destructure_result_request_msg(msg: GetResult::Request) -> unique_identifier_msgs::msg::UUID {
                msg.goal_id
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Goal {

                              pub theta: f32,

                          }

                          impl WrappedTypesupport for Goal { 

            type CStruct = turtlesim__action__RotateAbsolute_Goal; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__turtlesim__action__RotateAbsolute_Goal() }
            }

            fn create_msg() -> *mut turtlesim__action__RotateAbsolute_Goal {

                unsafe { turtlesim__action__RotateAbsolute_Goal__create() }

            }

            fn destroy_msg(msg: *mut turtlesim__action__RotateAbsolute_Goal) -> () {

                unsafe { turtlesim__action__RotateAbsolute_Goal__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Goal {
  Goal {
theta: msg.theta,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.theta = self.theta;
}



        }


                          
                          impl Default for Goal {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Goal>::new();
                                  Goal::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Result {

                              pub delta: f32,

                          }

                          impl WrappedTypesupport for Result { 

            type CStruct = turtlesim__action__RotateAbsolute_Result; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__turtlesim__action__RotateAbsolute_Result() }
            }

            fn create_msg() -> *mut turtlesim__action__RotateAbsolute_Result {

                unsafe { turtlesim__action__RotateAbsolute_Result__create() }

            }

            fn destroy_msg(msg: *mut turtlesim__action__RotateAbsolute_Result) -> () {

                unsafe { turtlesim__action__RotateAbsolute_Result__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Result {
  Result {
delta: msg.delta,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.delta = self.delta;
}



        }


                          
                          impl Default for Result {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Result>::new();
                                  Result::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Feedback {

                              pub remaining: f32,

                          }

                          impl WrappedTypesupport for Feedback { 

            type CStruct = turtlesim__action__RotateAbsolute_Feedback; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__turtlesim__action__RotateAbsolute_Feedback() }
            }

            fn create_msg() -> *mut turtlesim__action__RotateAbsolute_Feedback {

                unsafe { turtlesim__action__RotateAbsolute_Feedback__create() }

            }

            fn destroy_msg(msg: *mut turtlesim__action__RotateAbsolute_Feedback) -> () {

                unsafe { turtlesim__action__RotateAbsolute_Feedback__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Feedback {
  Feedback {
remaining: msg.remaining,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.remaining = self.remaining;
}



        }


                          
                          impl Default for Feedback {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Feedback>::new();
                                  Feedback::from_native(&msg_native)
                              }
                          }
             


                    #[allow(non_snake_case)]
    pub mod SendGoal {
    use super::super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__turtlesim__action__RotateAbsolute_SendGoal()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              pub goal_id: unique_identifier_msgs::msg::UUID,
pub goal: turtlesim::action::RotateAbsolute::Goal,

                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = turtlesim__action__RotateAbsolute_SendGoal_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__turtlesim__action__RotateAbsolute_SendGoal_Request() }
            }

            fn create_msg() -> *mut turtlesim__action__RotateAbsolute_SendGoal_Request {

                unsafe { turtlesim__action__RotateAbsolute_SendGoal_Request__create() }

            }

            fn destroy_msg(msg: *mut turtlesim__action__RotateAbsolute_SendGoal_Request) -> () {

                unsafe { turtlesim__action__RotateAbsolute_SendGoal_Request__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Request {
  Request {
goal_id: unique_identifier_msgs::msg::UUID::from_native(&msg.goal_id),
goal: turtlesim::action::RotateAbsolute::Goal::from_native(&msg.goal),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.goal_id.copy_to_native(&mut msg.goal_id);
self.goal.copy_to_native(&mut msg.goal);
}



        }


                          
                          impl Default for Request {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Request>::new();
                                  Request::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Response {

                              pub accepted: bool,
pub stamp: builtin_interfaces::msg::Time,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = turtlesim__action__RotateAbsolute_SendGoal_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__turtlesim__action__RotateAbsolute_SendGoal_Response() }
            }

            fn create_msg() -> *mut turtlesim__action__RotateAbsolute_SendGoal_Response {

                unsafe { turtlesim__action__RotateAbsolute_SendGoal_Response__create() }

            }

            fn destroy_msg(msg: *mut turtlesim__action__RotateAbsolute_SendGoal_Response) -> () {

                unsafe { turtlesim__action__RotateAbsolute_SendGoal_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
accepted: msg.accepted,
stamp: builtin_interfaces::msg::Time::from_native(&msg.stamp),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.accepted = self.accepted;
self.stamp.copy_to_native(&mut msg.stamp);
}



        }


                          
                          impl Default for Response {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Response>::new();
                                  Response::from_native(&msg_native)
                              }
                          }
             


                        }
#[allow(non_snake_case)]
    pub mod GetResult {
    use super::super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__turtlesim__action__RotateAbsolute_GetResult()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              pub goal_id: unique_identifier_msgs::msg::UUID,

                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = turtlesim__action__RotateAbsolute_GetResult_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__turtlesim__action__RotateAbsolute_GetResult_Request() }
            }

            fn create_msg() -> *mut turtlesim__action__RotateAbsolute_GetResult_Request {

                unsafe { turtlesim__action__RotateAbsolute_GetResult_Request__create() }

            }

            fn destroy_msg(msg: *mut turtlesim__action__RotateAbsolute_GetResult_Request) -> () {

                unsafe { turtlesim__action__RotateAbsolute_GetResult_Request__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Request {
  Request {
goal_id: unique_identifier_msgs::msg::UUID::from_native(&msg.goal_id),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.goal_id.copy_to_native(&mut msg.goal_id);
}



        }


                          
                          impl Default for Request {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Request>::new();
                                  Request::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Response {

                              pub status: i8,
pub result: turtlesim::action::RotateAbsolute::Result,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = turtlesim__action__RotateAbsolute_GetResult_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__turtlesim__action__RotateAbsolute_GetResult_Response() }
            }

            fn create_msg() -> *mut turtlesim__action__RotateAbsolute_GetResult_Response {

                unsafe { turtlesim__action__RotateAbsolute_GetResult_Response__create() }

            }

            fn destroy_msg(msg: *mut turtlesim__action__RotateAbsolute_GetResult_Response) -> () {

                unsafe { turtlesim__action__RotateAbsolute_GetResult_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
status: msg.status,
result: turtlesim::action::RotateAbsolute::Result::from_native(&msg.result),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.status = self.status;
self.result.copy_to_native(&mut msg.result);
}



        }


                          
                          impl Default for Response {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Response>::new();
                                  Response::from_native(&msg_native)
                              }
                          }
             


                        }

                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct FeedbackMessage {

                              pub goal_id: unique_identifier_msgs::msg::UUID,
pub feedback: turtlesim::action::RotateAbsolute::Feedback,

                          }

                          impl WrappedTypesupport for FeedbackMessage { 

            type CStruct = turtlesim__action__RotateAbsolute_FeedbackMessage; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__turtlesim__action__RotateAbsolute_FeedbackMessage() }
            }

            fn create_msg() -> *mut turtlesim__action__RotateAbsolute_FeedbackMessage {

                unsafe { turtlesim__action__RotateAbsolute_FeedbackMessage__create() }

            }

            fn destroy_msg(msg: *mut turtlesim__action__RotateAbsolute_FeedbackMessage) -> () {

                unsafe { turtlesim__action__RotateAbsolute_FeedbackMessage__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> FeedbackMessage {
  FeedbackMessage {
goal_id: unique_identifier_msgs::msg::UUID::from_native(&msg.goal_id),
feedback: turtlesim::action::RotateAbsolute::Feedback::from_native(&msg.feedback),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.goal_id.copy_to_native(&mut msg.goal_id);
self.feedback.copy_to_native(&mut msg.feedback);
}



        }


                          
                          impl Default for FeedbackMessage {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<FeedbackMessage>::new();
                                  FeedbackMessage::from_native(&msg_native)
                              }
                          }
             


                        }
  }
