  pub mod srv {
#[allow(non_snake_case)]
    pub mod DeleteParam {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__rosapi_msgs__srv__DeleteParam()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              pub name: std::string::String,

                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = rosapi_msgs__srv__DeleteParam_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosapi_msgs__srv__DeleteParam_Request() }
            }

            fn create_msg() -> *mut rosapi_msgs__srv__DeleteParam_Request {

                unsafe { rosapi_msgs__srv__DeleteParam_Request__create() }

            }

            fn destroy_msg(msg: *mut rosapi_msgs__srv__DeleteParam_Request) -> () {

                unsafe { rosapi_msgs__srv__DeleteParam_Request__destroy(msg) };

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

            type CStruct = rosapi_msgs__srv__DeleteParam_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosapi_msgs__srv__DeleteParam_Response() }
            }

            fn create_msg() -> *mut rosapi_msgs__srv__DeleteParam_Response {

                unsafe { rosapi_msgs__srv__DeleteParam_Response__create() }

            }

            fn destroy_msg(msg: *mut rosapi_msgs__srv__DeleteParam_Response) -> () {

                unsafe { rosapi_msgs__srv__DeleteParam_Response__destroy(msg) };

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
    pub mod GetActionServers {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__rosapi_msgs__srv__GetActionServers()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              
                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = rosapi_msgs__srv__GetActionServers_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosapi_msgs__srv__GetActionServers_Request() }
            }

            fn create_msg() -> *mut rosapi_msgs__srv__GetActionServers_Request {

                unsafe { rosapi_msgs__srv__GetActionServers_Request__create() }

            }

            fn destroy_msg(msg: *mut rosapi_msgs__srv__GetActionServers_Request) -> () {

                unsafe { rosapi_msgs__srv__GetActionServers_Request__destroy(msg) };

            }

            fn from_native(_msg: &Self::CStruct) -> Request {
  Request {
      }
    }



            fn copy_to_native(&self, _msg: &mut Self::CStruct) {}



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

                              pub action_servers: Vec<std::string::String>,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = rosapi_msgs__srv__GetActionServers_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosapi_msgs__srv__GetActionServers_Response() }
            }

            fn create_msg() -> *mut rosapi_msgs__srv__GetActionServers_Response {

                unsafe { rosapi_msgs__srv__GetActionServers_Response__create() }

            }

            fn destroy_msg(msg: *mut rosapi_msgs__srv__GetActionServers_Response) -> () {

                unsafe { rosapi_msgs__srv__GetActionServers_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
// is_upper_bound_: false
// member.array_size_ : 0
action_servers: msg.action_servers.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.action_servers.update(&self.action_servers);
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
    pub mod GetParam {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__rosapi_msgs__srv__GetParam()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              pub name: std::string::String,
pub default_value: std::string::String,

                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = rosapi_msgs__srv__GetParam_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosapi_msgs__srv__GetParam_Request() }
            }

            fn create_msg() -> *mut rosapi_msgs__srv__GetParam_Request {

                unsafe { rosapi_msgs__srv__GetParam_Request__create() }

            }

            fn destroy_msg(msg: *mut rosapi_msgs__srv__GetParam_Request) -> () {

                unsafe { rosapi_msgs__srv__GetParam_Request__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Request {
  Request {
name: msg.name.to_str().to_owned(),
default_value: msg.default_value.to_str().to_owned(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.name.assign(&self.name);
msg.default_value.assign(&self.default_value);
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

                              pub value: std::string::String,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = rosapi_msgs__srv__GetParam_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosapi_msgs__srv__GetParam_Response() }
            }

            fn create_msg() -> *mut rosapi_msgs__srv__GetParam_Response {

                unsafe { rosapi_msgs__srv__GetParam_Response__create() }

            }

            fn destroy_msg(msg: *mut rosapi_msgs__srv__GetParam_Response) -> () {

                unsafe { rosapi_msgs__srv__GetParam_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
value: msg.value.to_str().to_owned(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.value.assign(&self.value);
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
    pub mod GetParamNames {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__rosapi_msgs__srv__GetParamNames()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              
                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = rosapi_msgs__srv__GetParamNames_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosapi_msgs__srv__GetParamNames_Request() }
            }

            fn create_msg() -> *mut rosapi_msgs__srv__GetParamNames_Request {

                unsafe { rosapi_msgs__srv__GetParamNames_Request__create() }

            }

            fn destroy_msg(msg: *mut rosapi_msgs__srv__GetParamNames_Request) -> () {

                unsafe { rosapi_msgs__srv__GetParamNames_Request__destroy(msg) };

            }

            fn from_native(_msg: &Self::CStruct) -> Request {
  Request {
      }
    }



            fn copy_to_native(&self, _msg: &mut Self::CStruct) {}



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

                              pub names: Vec<std::string::String>,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = rosapi_msgs__srv__GetParamNames_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosapi_msgs__srv__GetParamNames_Response() }
            }

            fn create_msg() -> *mut rosapi_msgs__srv__GetParamNames_Response {

                unsafe { rosapi_msgs__srv__GetParamNames_Response__create() }

            }

            fn destroy_msg(msg: *mut rosapi_msgs__srv__GetParamNames_Response) -> () {

                unsafe { rosapi_msgs__srv__GetParamNames_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
// is_upper_bound_: false
// member.array_size_ : 0
names: msg.names.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.names.update(&self.names);
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
    pub mod GetROSVersion {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__rosapi_msgs__srv__GetROSVersion()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              
                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = rosapi_msgs__srv__GetROSVersion_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosapi_msgs__srv__GetROSVersion_Request() }
            }

            fn create_msg() -> *mut rosapi_msgs__srv__GetROSVersion_Request {

                unsafe { rosapi_msgs__srv__GetROSVersion_Request__create() }

            }

            fn destroy_msg(msg: *mut rosapi_msgs__srv__GetROSVersion_Request) -> () {

                unsafe { rosapi_msgs__srv__GetROSVersion_Request__destroy(msg) };

            }

            fn from_native(_msg: &Self::CStruct) -> Request {
  Request {
      }
    }



            fn copy_to_native(&self, _msg: &mut Self::CStruct) {}



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

                              pub version: u8,
pub distro: std::string::String,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = rosapi_msgs__srv__GetROSVersion_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosapi_msgs__srv__GetROSVersion_Response() }
            }

            fn create_msg() -> *mut rosapi_msgs__srv__GetROSVersion_Response {

                unsafe { rosapi_msgs__srv__GetROSVersion_Response__create() }

            }

            fn destroy_msg(msg: *mut rosapi_msgs__srv__GetROSVersion_Response) -> () {

                unsafe { rosapi_msgs__srv__GetROSVersion_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
version: msg.version,
distro: msg.distro.to_str().to_owned(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.version = self.version;
msg.distro.assign(&self.distro);
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
    pub mod GetTime {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__rosapi_msgs__srv__GetTime()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              
                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = rosapi_msgs__srv__GetTime_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosapi_msgs__srv__GetTime_Request() }
            }

            fn create_msg() -> *mut rosapi_msgs__srv__GetTime_Request {

                unsafe { rosapi_msgs__srv__GetTime_Request__create() }

            }

            fn destroy_msg(msg: *mut rosapi_msgs__srv__GetTime_Request) -> () {

                unsafe { rosapi_msgs__srv__GetTime_Request__destroy(msg) };

            }

            fn from_native(_msg: &Self::CStruct) -> Request {
  Request {
      }
    }



            fn copy_to_native(&self, _msg: &mut Self::CStruct) {}



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

                              pub time: builtin_interfaces::msg::Time,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = rosapi_msgs__srv__GetTime_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosapi_msgs__srv__GetTime_Response() }
            }

            fn create_msg() -> *mut rosapi_msgs__srv__GetTime_Response {

                unsafe { rosapi_msgs__srv__GetTime_Response__create() }

            }

            fn destroy_msg(msg: *mut rosapi_msgs__srv__GetTime_Response) -> () {

                unsafe { rosapi_msgs__srv__GetTime_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
time: builtin_interfaces::msg::Time::from_native(&msg.time),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.time.copy_to_native(&mut msg.time);
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
    pub mod HasParam {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__rosapi_msgs__srv__HasParam()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              pub name: std::string::String,

                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = rosapi_msgs__srv__HasParam_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosapi_msgs__srv__HasParam_Request() }
            }

            fn create_msg() -> *mut rosapi_msgs__srv__HasParam_Request {

                unsafe { rosapi_msgs__srv__HasParam_Request__create() }

            }

            fn destroy_msg(msg: *mut rosapi_msgs__srv__HasParam_Request) -> () {

                unsafe { rosapi_msgs__srv__HasParam_Request__destroy(msg) };

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

                              pub exists: bool,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = rosapi_msgs__srv__HasParam_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosapi_msgs__srv__HasParam_Response() }
            }

            fn create_msg() -> *mut rosapi_msgs__srv__HasParam_Response {

                unsafe { rosapi_msgs__srv__HasParam_Response__create() }

            }

            fn destroy_msg(msg: *mut rosapi_msgs__srv__HasParam_Response) -> () {

                unsafe { rosapi_msgs__srv__HasParam_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
exists: msg.exists,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.exists = self.exists;
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
    pub mod MessageDetails {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__rosapi_msgs__srv__MessageDetails()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              pub type_: std::string::String,

                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = rosapi_msgs__srv__MessageDetails_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosapi_msgs__srv__MessageDetails_Request() }
            }

            fn create_msg() -> *mut rosapi_msgs__srv__MessageDetails_Request {

                unsafe { rosapi_msgs__srv__MessageDetails_Request__create() }

            }

            fn destroy_msg(msg: *mut rosapi_msgs__srv__MessageDetails_Request) -> () {

                unsafe { rosapi_msgs__srv__MessageDetails_Request__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Request {
  Request {
type_: msg.type_.to_str().to_owned(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.type_.assign(&self.type_);
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

                              pub typedefs: Vec<rosapi_msgs::msg::TypeDef>,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = rosapi_msgs__srv__MessageDetails_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosapi_msgs__srv__MessageDetails_Response() }
            }

            fn create_msg() -> *mut rosapi_msgs__srv__MessageDetails_Response {

                unsafe { rosapi_msgs__srv__MessageDetails_Response__create() }

            }

            fn destroy_msg(msg: *mut rosapi_msgs__srv__MessageDetails_Response) -> () {

                unsafe { rosapi_msgs__srv__MessageDetails_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
// is_upper_bound_: false
// member.array_size_ : 0
typedefs : {
let mut temp = Vec::with_capacity(msg.typedefs.size);
let slice = unsafe { std::slice::from_raw_parts(msg.typedefs.data, msg.typedefs.size)};
for s in slice { temp.push(rosapi_msgs::msg::TypeDef::from_native(s)); }
temp },
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {unsafe { rosapi_msgs__msg__TypeDef__Sequence__fini(&mut msg.typedefs) };
unsafe { rosapi_msgs__msg__TypeDef__Sequence__init(&mut msg.typedefs, self.typedefs.len()) };
let slice = unsafe { std::slice::from_raw_parts_mut(msg.typedefs.data, msg.typedefs.size)};
for (t,s) in slice.iter_mut().zip(&self.typedefs) { s.copy_to_native(t);}
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
    pub mod NodeDetails {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__rosapi_msgs__srv__NodeDetails()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              pub node: std::string::String,

                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = rosapi_msgs__srv__NodeDetails_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosapi_msgs__srv__NodeDetails_Request() }
            }

            fn create_msg() -> *mut rosapi_msgs__srv__NodeDetails_Request {

                unsafe { rosapi_msgs__srv__NodeDetails_Request__create() }

            }

            fn destroy_msg(msg: *mut rosapi_msgs__srv__NodeDetails_Request) -> () {

                unsafe { rosapi_msgs__srv__NodeDetails_Request__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Request {
  Request {
node: msg.node.to_str().to_owned(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.node.assign(&self.node);
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

                              pub subscribing: Vec<std::string::String>,
pub publishing: Vec<std::string::String>,
pub services: Vec<std::string::String>,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = rosapi_msgs__srv__NodeDetails_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosapi_msgs__srv__NodeDetails_Response() }
            }

            fn create_msg() -> *mut rosapi_msgs__srv__NodeDetails_Response {

                unsafe { rosapi_msgs__srv__NodeDetails_Response__create() }

            }

            fn destroy_msg(msg: *mut rosapi_msgs__srv__NodeDetails_Response) -> () {

                unsafe { rosapi_msgs__srv__NodeDetails_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
// is_upper_bound_: false
// member.array_size_ : 0
subscribing: msg.subscribing.to_vec(),
// is_upper_bound_: false
// member.array_size_ : 0
publishing: msg.publishing.to_vec(),
// is_upper_bound_: false
// member.array_size_ : 0
services: msg.services.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.subscribing.update(&self.subscribing);
msg.publishing.update(&self.publishing);
msg.services.update(&self.services);
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
    pub mod Nodes {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__rosapi_msgs__srv__Nodes()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              
                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = rosapi_msgs__srv__Nodes_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosapi_msgs__srv__Nodes_Request() }
            }

            fn create_msg() -> *mut rosapi_msgs__srv__Nodes_Request {

                unsafe { rosapi_msgs__srv__Nodes_Request__create() }

            }

            fn destroy_msg(msg: *mut rosapi_msgs__srv__Nodes_Request) -> () {

                unsafe { rosapi_msgs__srv__Nodes_Request__destroy(msg) };

            }

            fn from_native(_msg: &Self::CStruct) -> Request {
  Request {
      }
    }



            fn copy_to_native(&self, _msg: &mut Self::CStruct) {}



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

                              pub nodes: Vec<std::string::String>,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = rosapi_msgs__srv__Nodes_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosapi_msgs__srv__Nodes_Response() }
            }

            fn create_msg() -> *mut rosapi_msgs__srv__Nodes_Response {

                unsafe { rosapi_msgs__srv__Nodes_Response__create() }

            }

            fn destroy_msg(msg: *mut rosapi_msgs__srv__Nodes_Response) -> () {

                unsafe { rosapi_msgs__srv__Nodes_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
// is_upper_bound_: false
// member.array_size_ : 0
nodes: msg.nodes.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.nodes.update(&self.nodes);
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
    pub mod Publishers {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__rosapi_msgs__srv__Publishers()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              pub topic: std::string::String,

                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = rosapi_msgs__srv__Publishers_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosapi_msgs__srv__Publishers_Request() }
            }

            fn create_msg() -> *mut rosapi_msgs__srv__Publishers_Request {

                unsafe { rosapi_msgs__srv__Publishers_Request__create() }

            }

            fn destroy_msg(msg: *mut rosapi_msgs__srv__Publishers_Request) -> () {

                unsafe { rosapi_msgs__srv__Publishers_Request__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Request {
  Request {
topic: msg.topic.to_str().to_owned(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.topic.assign(&self.topic);
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

                              pub publishers: Vec<std::string::String>,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = rosapi_msgs__srv__Publishers_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosapi_msgs__srv__Publishers_Response() }
            }

            fn create_msg() -> *mut rosapi_msgs__srv__Publishers_Response {

                unsafe { rosapi_msgs__srv__Publishers_Response__create() }

            }

            fn destroy_msg(msg: *mut rosapi_msgs__srv__Publishers_Response) -> () {

                unsafe { rosapi_msgs__srv__Publishers_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
// is_upper_bound_: false
// member.array_size_ : 0
publishers: msg.publishers.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.publishers.update(&self.publishers);
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
    pub mod ServiceNode {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__rosapi_msgs__srv__ServiceNode()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              pub service: std::string::String,

                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = rosapi_msgs__srv__ServiceNode_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosapi_msgs__srv__ServiceNode_Request() }
            }

            fn create_msg() -> *mut rosapi_msgs__srv__ServiceNode_Request {

                unsafe { rosapi_msgs__srv__ServiceNode_Request__create() }

            }

            fn destroy_msg(msg: *mut rosapi_msgs__srv__ServiceNode_Request) -> () {

                unsafe { rosapi_msgs__srv__ServiceNode_Request__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Request {
  Request {
service: msg.service.to_str().to_owned(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.service.assign(&self.service);
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

                              pub node: std::string::String,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = rosapi_msgs__srv__ServiceNode_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosapi_msgs__srv__ServiceNode_Response() }
            }

            fn create_msg() -> *mut rosapi_msgs__srv__ServiceNode_Response {

                unsafe { rosapi_msgs__srv__ServiceNode_Response__create() }

            }

            fn destroy_msg(msg: *mut rosapi_msgs__srv__ServiceNode_Response) -> () {

                unsafe { rosapi_msgs__srv__ServiceNode_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
node: msg.node.to_str().to_owned(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.node.assign(&self.node);
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
    pub mod ServiceProviders {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__rosapi_msgs__srv__ServiceProviders()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              pub service: std::string::String,

                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = rosapi_msgs__srv__ServiceProviders_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosapi_msgs__srv__ServiceProviders_Request() }
            }

            fn create_msg() -> *mut rosapi_msgs__srv__ServiceProviders_Request {

                unsafe { rosapi_msgs__srv__ServiceProviders_Request__create() }

            }

            fn destroy_msg(msg: *mut rosapi_msgs__srv__ServiceProviders_Request) -> () {

                unsafe { rosapi_msgs__srv__ServiceProviders_Request__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Request {
  Request {
service: msg.service.to_str().to_owned(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.service.assign(&self.service);
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

                              pub providers: Vec<std::string::String>,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = rosapi_msgs__srv__ServiceProviders_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosapi_msgs__srv__ServiceProviders_Response() }
            }

            fn create_msg() -> *mut rosapi_msgs__srv__ServiceProviders_Response {

                unsafe { rosapi_msgs__srv__ServiceProviders_Response__create() }

            }

            fn destroy_msg(msg: *mut rosapi_msgs__srv__ServiceProviders_Response) -> () {

                unsafe { rosapi_msgs__srv__ServiceProviders_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
// is_upper_bound_: false
// member.array_size_ : 0
providers: msg.providers.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.providers.update(&self.providers);
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
    pub mod ServiceRequestDetails {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__rosapi_msgs__srv__ServiceRequestDetails()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              pub type_: std::string::String,

                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = rosapi_msgs__srv__ServiceRequestDetails_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosapi_msgs__srv__ServiceRequestDetails_Request() }
            }

            fn create_msg() -> *mut rosapi_msgs__srv__ServiceRequestDetails_Request {

                unsafe { rosapi_msgs__srv__ServiceRequestDetails_Request__create() }

            }

            fn destroy_msg(msg: *mut rosapi_msgs__srv__ServiceRequestDetails_Request) -> () {

                unsafe { rosapi_msgs__srv__ServiceRequestDetails_Request__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Request {
  Request {
type_: msg.type_.to_str().to_owned(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.type_.assign(&self.type_);
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

                              pub typedefs: Vec<rosapi_msgs::msg::TypeDef>,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = rosapi_msgs__srv__ServiceRequestDetails_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosapi_msgs__srv__ServiceRequestDetails_Response() }
            }

            fn create_msg() -> *mut rosapi_msgs__srv__ServiceRequestDetails_Response {

                unsafe { rosapi_msgs__srv__ServiceRequestDetails_Response__create() }

            }

            fn destroy_msg(msg: *mut rosapi_msgs__srv__ServiceRequestDetails_Response) -> () {

                unsafe { rosapi_msgs__srv__ServiceRequestDetails_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
// is_upper_bound_: false
// member.array_size_ : 0
typedefs : {
let mut temp = Vec::with_capacity(msg.typedefs.size);
let slice = unsafe { std::slice::from_raw_parts(msg.typedefs.data, msg.typedefs.size)};
for s in slice { temp.push(rosapi_msgs::msg::TypeDef::from_native(s)); }
temp },
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {unsafe { rosapi_msgs__msg__TypeDef__Sequence__fini(&mut msg.typedefs) };
unsafe { rosapi_msgs__msg__TypeDef__Sequence__init(&mut msg.typedefs, self.typedefs.len()) };
let slice = unsafe { std::slice::from_raw_parts_mut(msg.typedefs.data, msg.typedefs.size)};
for (t,s) in slice.iter_mut().zip(&self.typedefs) { s.copy_to_native(t);}
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
    pub mod ServiceResponseDetails {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__rosapi_msgs__srv__ServiceResponseDetails()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              pub type_: std::string::String,

                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = rosapi_msgs__srv__ServiceResponseDetails_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosapi_msgs__srv__ServiceResponseDetails_Request() }
            }

            fn create_msg() -> *mut rosapi_msgs__srv__ServiceResponseDetails_Request {

                unsafe { rosapi_msgs__srv__ServiceResponseDetails_Request__create() }

            }

            fn destroy_msg(msg: *mut rosapi_msgs__srv__ServiceResponseDetails_Request) -> () {

                unsafe { rosapi_msgs__srv__ServiceResponseDetails_Request__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Request {
  Request {
type_: msg.type_.to_str().to_owned(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.type_.assign(&self.type_);
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

                              pub typedefs: Vec<rosapi_msgs::msg::TypeDef>,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = rosapi_msgs__srv__ServiceResponseDetails_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosapi_msgs__srv__ServiceResponseDetails_Response() }
            }

            fn create_msg() -> *mut rosapi_msgs__srv__ServiceResponseDetails_Response {

                unsafe { rosapi_msgs__srv__ServiceResponseDetails_Response__create() }

            }

            fn destroy_msg(msg: *mut rosapi_msgs__srv__ServiceResponseDetails_Response) -> () {

                unsafe { rosapi_msgs__srv__ServiceResponseDetails_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
// is_upper_bound_: false
// member.array_size_ : 0
typedefs : {
let mut temp = Vec::with_capacity(msg.typedefs.size);
let slice = unsafe { std::slice::from_raw_parts(msg.typedefs.data, msg.typedefs.size)};
for s in slice { temp.push(rosapi_msgs::msg::TypeDef::from_native(s)); }
temp },
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {unsafe { rosapi_msgs__msg__TypeDef__Sequence__fini(&mut msg.typedefs) };
unsafe { rosapi_msgs__msg__TypeDef__Sequence__init(&mut msg.typedefs, self.typedefs.len()) };
let slice = unsafe { std::slice::from_raw_parts_mut(msg.typedefs.data, msg.typedefs.size)};
for (t,s) in slice.iter_mut().zip(&self.typedefs) { s.copy_to_native(t);}
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
    pub mod ServiceType {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__rosapi_msgs__srv__ServiceType()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              pub service: std::string::String,

                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = rosapi_msgs__srv__ServiceType_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosapi_msgs__srv__ServiceType_Request() }
            }

            fn create_msg() -> *mut rosapi_msgs__srv__ServiceType_Request {

                unsafe { rosapi_msgs__srv__ServiceType_Request__create() }

            }

            fn destroy_msg(msg: *mut rosapi_msgs__srv__ServiceType_Request) -> () {

                unsafe { rosapi_msgs__srv__ServiceType_Request__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Request {
  Request {
service: msg.service.to_str().to_owned(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.service.assign(&self.service);
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

                              pub type_: std::string::String,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = rosapi_msgs__srv__ServiceType_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosapi_msgs__srv__ServiceType_Response() }
            }

            fn create_msg() -> *mut rosapi_msgs__srv__ServiceType_Response {

                unsafe { rosapi_msgs__srv__ServiceType_Response__create() }

            }

            fn destroy_msg(msg: *mut rosapi_msgs__srv__ServiceType_Response) -> () {

                unsafe { rosapi_msgs__srv__ServiceType_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
type_: msg.type_.to_str().to_owned(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.type_.assign(&self.type_);
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
    pub mod Services {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__rosapi_msgs__srv__Services()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              
                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = rosapi_msgs__srv__Services_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosapi_msgs__srv__Services_Request() }
            }

            fn create_msg() -> *mut rosapi_msgs__srv__Services_Request {

                unsafe { rosapi_msgs__srv__Services_Request__create() }

            }

            fn destroy_msg(msg: *mut rosapi_msgs__srv__Services_Request) -> () {

                unsafe { rosapi_msgs__srv__Services_Request__destroy(msg) };

            }

            fn from_native(_msg: &Self::CStruct) -> Request {
  Request {
      }
    }



            fn copy_to_native(&self, _msg: &mut Self::CStruct) {}



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

                              pub services: Vec<std::string::String>,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = rosapi_msgs__srv__Services_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosapi_msgs__srv__Services_Response() }
            }

            fn create_msg() -> *mut rosapi_msgs__srv__Services_Response {

                unsafe { rosapi_msgs__srv__Services_Response__create() }

            }

            fn destroy_msg(msg: *mut rosapi_msgs__srv__Services_Response) -> () {

                unsafe { rosapi_msgs__srv__Services_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
// is_upper_bound_: false
// member.array_size_ : 0
services: msg.services.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.services.update(&self.services);
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
    pub mod ServicesForType {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__rosapi_msgs__srv__ServicesForType()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              pub type_: std::string::String,

                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = rosapi_msgs__srv__ServicesForType_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosapi_msgs__srv__ServicesForType_Request() }
            }

            fn create_msg() -> *mut rosapi_msgs__srv__ServicesForType_Request {

                unsafe { rosapi_msgs__srv__ServicesForType_Request__create() }

            }

            fn destroy_msg(msg: *mut rosapi_msgs__srv__ServicesForType_Request) -> () {

                unsafe { rosapi_msgs__srv__ServicesForType_Request__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Request {
  Request {
type_: msg.type_.to_str().to_owned(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.type_.assign(&self.type_);
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

                              pub services: Vec<std::string::String>,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = rosapi_msgs__srv__ServicesForType_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosapi_msgs__srv__ServicesForType_Response() }
            }

            fn create_msg() -> *mut rosapi_msgs__srv__ServicesForType_Response {

                unsafe { rosapi_msgs__srv__ServicesForType_Response__create() }

            }

            fn destroy_msg(msg: *mut rosapi_msgs__srv__ServicesForType_Response) -> () {

                unsafe { rosapi_msgs__srv__ServicesForType_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
// is_upper_bound_: false
// member.array_size_ : 0
services: msg.services.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.services.update(&self.services);
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
    pub mod SetParam {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__rosapi_msgs__srv__SetParam()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              pub name: std::string::String,
pub value: std::string::String,

                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = rosapi_msgs__srv__SetParam_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosapi_msgs__srv__SetParam_Request() }
            }

            fn create_msg() -> *mut rosapi_msgs__srv__SetParam_Request {

                unsafe { rosapi_msgs__srv__SetParam_Request__create() }

            }

            fn destroy_msg(msg: *mut rosapi_msgs__srv__SetParam_Request) -> () {

                unsafe { rosapi_msgs__srv__SetParam_Request__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Request {
  Request {
name: msg.name.to_str().to_owned(),
value: msg.value.to_str().to_owned(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.name.assign(&self.name);
msg.value.assign(&self.value);
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

            type CStruct = rosapi_msgs__srv__SetParam_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosapi_msgs__srv__SetParam_Response() }
            }

            fn create_msg() -> *mut rosapi_msgs__srv__SetParam_Response {

                unsafe { rosapi_msgs__srv__SetParam_Response__create() }

            }

            fn destroy_msg(msg: *mut rosapi_msgs__srv__SetParam_Response) -> () {

                unsafe { rosapi_msgs__srv__SetParam_Response__destroy(msg) };

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
    pub mod Subscribers {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__rosapi_msgs__srv__Subscribers()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              pub topic: std::string::String,

                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = rosapi_msgs__srv__Subscribers_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosapi_msgs__srv__Subscribers_Request() }
            }

            fn create_msg() -> *mut rosapi_msgs__srv__Subscribers_Request {

                unsafe { rosapi_msgs__srv__Subscribers_Request__create() }

            }

            fn destroy_msg(msg: *mut rosapi_msgs__srv__Subscribers_Request) -> () {

                unsafe { rosapi_msgs__srv__Subscribers_Request__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Request {
  Request {
topic: msg.topic.to_str().to_owned(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.topic.assign(&self.topic);
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

                              pub subscribers: Vec<std::string::String>,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = rosapi_msgs__srv__Subscribers_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosapi_msgs__srv__Subscribers_Response() }
            }

            fn create_msg() -> *mut rosapi_msgs__srv__Subscribers_Response {

                unsafe { rosapi_msgs__srv__Subscribers_Response__create() }

            }

            fn destroy_msg(msg: *mut rosapi_msgs__srv__Subscribers_Response) -> () {

                unsafe { rosapi_msgs__srv__Subscribers_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
// is_upper_bound_: false
// member.array_size_ : 0
subscribers: msg.subscribers.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.subscribers.update(&self.subscribers);
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
    pub mod TopicType {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__rosapi_msgs__srv__TopicType()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              pub topic: std::string::String,

                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = rosapi_msgs__srv__TopicType_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosapi_msgs__srv__TopicType_Request() }
            }

            fn create_msg() -> *mut rosapi_msgs__srv__TopicType_Request {

                unsafe { rosapi_msgs__srv__TopicType_Request__create() }

            }

            fn destroy_msg(msg: *mut rosapi_msgs__srv__TopicType_Request) -> () {

                unsafe { rosapi_msgs__srv__TopicType_Request__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Request {
  Request {
topic: msg.topic.to_str().to_owned(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.topic.assign(&self.topic);
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

                              pub type_: std::string::String,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = rosapi_msgs__srv__TopicType_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosapi_msgs__srv__TopicType_Response() }
            }

            fn create_msg() -> *mut rosapi_msgs__srv__TopicType_Response {

                unsafe { rosapi_msgs__srv__TopicType_Response__create() }

            }

            fn destroy_msg(msg: *mut rosapi_msgs__srv__TopicType_Response) -> () {

                unsafe { rosapi_msgs__srv__TopicType_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
type_: msg.type_.to_str().to_owned(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.type_.assign(&self.type_);
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
    pub mod Topics {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__rosapi_msgs__srv__Topics()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              
                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = rosapi_msgs__srv__Topics_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosapi_msgs__srv__Topics_Request() }
            }

            fn create_msg() -> *mut rosapi_msgs__srv__Topics_Request {

                unsafe { rosapi_msgs__srv__Topics_Request__create() }

            }

            fn destroy_msg(msg: *mut rosapi_msgs__srv__Topics_Request) -> () {

                unsafe { rosapi_msgs__srv__Topics_Request__destroy(msg) };

            }

            fn from_native(_msg: &Self::CStruct) -> Request {
  Request {
      }
    }



            fn copy_to_native(&self, _msg: &mut Self::CStruct) {}



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

                              pub topics: Vec<std::string::String>,
pub types: Vec<std::string::String>,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = rosapi_msgs__srv__Topics_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosapi_msgs__srv__Topics_Response() }
            }

            fn create_msg() -> *mut rosapi_msgs__srv__Topics_Response {

                unsafe { rosapi_msgs__srv__Topics_Response__create() }

            }

            fn destroy_msg(msg: *mut rosapi_msgs__srv__Topics_Response) -> () {

                unsafe { rosapi_msgs__srv__Topics_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
// is_upper_bound_: false
// member.array_size_ : 0
topics: msg.topics.to_vec(),
// is_upper_bound_: false
// member.array_size_ : 0
types: msg.types.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.topics.update(&self.topics);
msg.types.update(&self.types);
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
    pub mod TopicsAndRawTypes {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__rosapi_msgs__srv__TopicsAndRawTypes()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              
                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = rosapi_msgs__srv__TopicsAndRawTypes_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosapi_msgs__srv__TopicsAndRawTypes_Request() }
            }

            fn create_msg() -> *mut rosapi_msgs__srv__TopicsAndRawTypes_Request {

                unsafe { rosapi_msgs__srv__TopicsAndRawTypes_Request__create() }

            }

            fn destroy_msg(msg: *mut rosapi_msgs__srv__TopicsAndRawTypes_Request) -> () {

                unsafe { rosapi_msgs__srv__TopicsAndRawTypes_Request__destroy(msg) };

            }

            fn from_native(_msg: &Self::CStruct) -> Request {
  Request {
      }
    }



            fn copy_to_native(&self, _msg: &mut Self::CStruct) {}



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

                              pub topics: Vec<std::string::String>,
pub types: Vec<std::string::String>,
pub typedefs_full_text: Vec<std::string::String>,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = rosapi_msgs__srv__TopicsAndRawTypes_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosapi_msgs__srv__TopicsAndRawTypes_Response() }
            }

            fn create_msg() -> *mut rosapi_msgs__srv__TopicsAndRawTypes_Response {

                unsafe { rosapi_msgs__srv__TopicsAndRawTypes_Response__create() }

            }

            fn destroy_msg(msg: *mut rosapi_msgs__srv__TopicsAndRawTypes_Response) -> () {

                unsafe { rosapi_msgs__srv__TopicsAndRawTypes_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
// is_upper_bound_: false
// member.array_size_ : 0
topics: msg.topics.to_vec(),
// is_upper_bound_: false
// member.array_size_ : 0
types: msg.types.to_vec(),
// is_upper_bound_: false
// member.array_size_ : 0
typedefs_full_text: msg.typedefs_full_text.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.topics.update(&self.topics);
msg.types.update(&self.types);
msg.typedefs_full_text.update(&self.typedefs_full_text);
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
    pub mod TopicsForType {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__rosapi_msgs__srv__TopicsForType()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              pub type_: std::string::String,

                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = rosapi_msgs__srv__TopicsForType_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosapi_msgs__srv__TopicsForType_Request() }
            }

            fn create_msg() -> *mut rosapi_msgs__srv__TopicsForType_Request {

                unsafe { rosapi_msgs__srv__TopicsForType_Request__create() }

            }

            fn destroy_msg(msg: *mut rosapi_msgs__srv__TopicsForType_Request) -> () {

                unsafe { rosapi_msgs__srv__TopicsForType_Request__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Request {
  Request {
type_: msg.type_.to_str().to_owned(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.type_.assign(&self.type_);
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

                              pub topics: Vec<std::string::String>,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = rosapi_msgs__srv__TopicsForType_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosapi_msgs__srv__TopicsForType_Response() }
            }

            fn create_msg() -> *mut rosapi_msgs__srv__TopicsForType_Response {

                unsafe { rosapi_msgs__srv__TopicsForType_Response__create() }

            }

            fn destroy_msg(msg: *mut rosapi_msgs__srv__TopicsForType_Response) -> () {

                unsafe { rosapi_msgs__srv__TopicsForType_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
// is_upper_bound_: false
// member.array_size_ : 0
topics: msg.topics.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.topics.update(&self.topics);
}



        }


                          
                          impl Default for Response {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Response>::new();
                                  Response::from_native(&msg_native)
                              }
                          }
             


                        }
  }
  pub mod msg {
    use super::super::*;

                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct TypeDef {

                              pub type_: std::string::String,
pub fieldnames: Vec<std::string::String>,
pub fieldtypes: Vec<std::string::String>,
pub fieldarraylen: Vec<i32>,
pub examples: Vec<std::string::String>,
pub constnames: Vec<std::string::String>,
pub constvalues: Vec<std::string::String>,

                          }

                          impl WrappedTypesupport for TypeDef { 

            type CStruct = rosapi_msgs__msg__TypeDef; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosapi_msgs__msg__TypeDef() }
            }

            fn create_msg() -> *mut rosapi_msgs__msg__TypeDef {

                unsafe { rosapi_msgs__msg__TypeDef__create() }

            }

            fn destroy_msg(msg: *mut rosapi_msgs__msg__TypeDef) -> () {

                unsafe { rosapi_msgs__msg__TypeDef__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> TypeDef {
  TypeDef {
type_: msg.type_.to_str().to_owned(),
// is_upper_bound_: false
// member.array_size_ : 0
fieldnames: msg.fieldnames.to_vec(),
// is_upper_bound_: false
// member.array_size_ : 0
fieldtypes: msg.fieldtypes.to_vec(),
// is_upper_bound_: false
// member.array_size_ : 0
fieldarraylen: msg.fieldarraylen.to_vec(),
// is_upper_bound_: false
// member.array_size_ : 0
examples: msg.examples.to_vec(),
// is_upper_bound_: false
// member.array_size_ : 0
constnames: msg.constnames.to_vec(),
// is_upper_bound_: false
// member.array_size_ : 0
constvalues: msg.constvalues.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.type_.assign(&self.type_);
msg.fieldnames.update(&self.fieldnames);
msg.fieldtypes.update(&self.fieldtypes);
msg.fieldarraylen.update(&self.fieldarraylen);
msg.examples.update(&self.examples);
msg.constnames.update(&self.constnames);
msg.constvalues.update(&self.constvalues);
}



        }


                          
                          impl Default for TypeDef {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<TypeDef>::new();
                                  TypeDef::from_native(&msg_native)
                              }
                          }
             


                      }
