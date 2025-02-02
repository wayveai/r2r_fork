  pub mod msg {
    use super::super::*;

                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct GoalID {

                              pub stamp: builtin_interfaces::msg::Time,
pub id: std::string::String,

                          }

                          impl WrappedTypesupport for GoalID { 

            type CStruct = actionlib_msgs__msg__GoalID; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__actionlib_msgs__msg__GoalID() }
            }

            fn create_msg() -> *mut actionlib_msgs__msg__GoalID {

                unsafe { actionlib_msgs__msg__GoalID__create() }

            }

            fn destroy_msg(msg: *mut actionlib_msgs__msg__GoalID) -> () {

                unsafe { actionlib_msgs__msg__GoalID__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> GoalID {
  GoalID {
stamp: builtin_interfaces::msg::Time::from_native(&msg.stamp),
id: msg.id.to_str().to_owned(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.stamp.copy_to_native(&mut msg.stamp);
msg.id.assign(&self.id);
}



        }


                          
                          impl Default for GoalID {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<GoalID>::new();
                                  GoalID::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct GoalStatus {

                              pub goal_id: actionlib_msgs::msg::GoalID,
pub status: u8,
pub text: std::string::String,

                          }

                          impl WrappedTypesupport for GoalStatus { 

            type CStruct = actionlib_msgs__msg__GoalStatus; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__actionlib_msgs__msg__GoalStatus() }
            }

            fn create_msg() -> *mut actionlib_msgs__msg__GoalStatus {

                unsafe { actionlib_msgs__msg__GoalStatus__create() }

            }

            fn destroy_msg(msg: *mut actionlib_msgs__msg__GoalStatus) -> () {

                unsafe { actionlib_msgs__msg__GoalStatus__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> GoalStatus {
  GoalStatus {
goal_id: actionlib_msgs::msg::GoalID::from_native(&msg.goal_id),
status: msg.status,
text: msg.text.to_str().to_owned(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.goal_id.copy_to_native(&mut msg.goal_id);
msg.status = self.status;
msg.text.assign(&self.text);
}



        }


                          
                          impl Default for GoalStatus {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<GoalStatus>::new();
                                  GoalStatus::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct GoalStatusArray {

                              pub header: std_msgs::msg::Header,
pub status_list: Vec<actionlib_msgs::msg::GoalStatus>,

                          }

                          impl WrappedTypesupport for GoalStatusArray { 

            type CStruct = actionlib_msgs__msg__GoalStatusArray; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__actionlib_msgs__msg__GoalStatusArray() }
            }

            fn create_msg() -> *mut actionlib_msgs__msg__GoalStatusArray {

                unsafe { actionlib_msgs__msg__GoalStatusArray__create() }

            }

            fn destroy_msg(msg: *mut actionlib_msgs__msg__GoalStatusArray) -> () {

                unsafe { actionlib_msgs__msg__GoalStatusArray__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> GoalStatusArray {
  GoalStatusArray {
header: std_msgs::msg::Header::from_native(&msg.header),
// is_upper_bound_: false
// member.array_size_ : 0
status_list : {
let mut temp = Vec::with_capacity(msg.status_list.size);
let slice = unsafe { std::slice::from_raw_parts(msg.status_list.data, msg.status_list.size)};
for s in slice { temp.push(actionlib_msgs::msg::GoalStatus::from_native(s)); }
temp },
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
unsafe { actionlib_msgs__msg__GoalStatus__Sequence__fini(&mut msg.status_list) };
unsafe { actionlib_msgs__msg__GoalStatus__Sequence__init(&mut msg.status_list, self.status_list.len()) };
let slice = unsafe { std::slice::from_raw_parts_mut(msg.status_list.data, msg.status_list.size)};
for (t,s) in slice.iter_mut().zip(&self.status_list) { s.copy_to_native(t);}
}



        }


                          
                          impl Default for GoalStatusArray {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<GoalStatusArray>::new();
                                  GoalStatusArray::from_native(&msg_native)
                              }
                          }
             


                      }
