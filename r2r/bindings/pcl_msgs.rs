  pub mod srv {
#[allow(non_snake_case)]
    pub mod UpdateFilename {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__pcl_msgs__srv__UpdateFilename()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              pub filename: std::string::String,

                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = pcl_msgs__srv__UpdateFilename_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pcl_msgs__srv__UpdateFilename_Request() }
            }

            fn create_msg() -> *mut pcl_msgs__srv__UpdateFilename_Request {

                unsafe { pcl_msgs__srv__UpdateFilename_Request__create() }

            }

            fn destroy_msg(msg: *mut pcl_msgs__srv__UpdateFilename_Request) -> () {

                unsafe { pcl_msgs__srv__UpdateFilename_Request__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Request {
  Request {
filename: msg.filename.to_str().to_owned(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.filename.assign(&self.filename);
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

                              pub success: bool,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = pcl_msgs__srv__UpdateFilename_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pcl_msgs__srv__UpdateFilename_Response() }
            }

            fn create_msg() -> *mut pcl_msgs__srv__UpdateFilename_Response {

                unsafe { pcl_msgs__srv__UpdateFilename_Response__create() }

            }

            fn destroy_msg(msg: *mut pcl_msgs__srv__UpdateFilename_Response) -> () {

                unsafe { pcl_msgs__srv__UpdateFilename_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
success: msg.success,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.success = self.success;
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
                          pub struct ModelCoefficients {

                              pub header: std_msgs::msg::Header,
pub values: Vec<f32>,

                          }

                          impl WrappedTypesupport for ModelCoefficients { 

            type CStruct = pcl_msgs__msg__ModelCoefficients; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pcl_msgs__msg__ModelCoefficients() }
            }

            fn create_msg() -> *mut pcl_msgs__msg__ModelCoefficients {

                unsafe { pcl_msgs__msg__ModelCoefficients__create() }

            }

            fn destroy_msg(msg: *mut pcl_msgs__msg__ModelCoefficients) -> () {

                unsafe { pcl_msgs__msg__ModelCoefficients__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> ModelCoefficients {
  ModelCoefficients {
header: std_msgs::msg::Header::from_native(&msg.header),
// is_upper_bound_: false
// member.array_size_ : 0
values: msg.values.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.values.update(&self.values);
}



        }


                          
                          impl Default for ModelCoefficients {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<ModelCoefficients>::new();
                                  ModelCoefficients::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct PointIndices {

                              pub header: std_msgs::msg::Header,
pub indices: Vec<i32>,

                          }

                          impl WrappedTypesupport for PointIndices { 

            type CStruct = pcl_msgs__msg__PointIndices; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pcl_msgs__msg__PointIndices() }
            }

            fn create_msg() -> *mut pcl_msgs__msg__PointIndices {

                unsafe { pcl_msgs__msg__PointIndices__create() }

            }

            fn destroy_msg(msg: *mut pcl_msgs__msg__PointIndices) -> () {

                unsafe { pcl_msgs__msg__PointIndices__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> PointIndices {
  PointIndices {
header: std_msgs::msg::Header::from_native(&msg.header),
// is_upper_bound_: false
// member.array_size_ : 0
indices: msg.indices.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.indices.update(&self.indices);
}



        }


                          
                          impl Default for PointIndices {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<PointIndices>::new();
                                  PointIndices::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct PolygonMesh {

                              pub header: std_msgs::msg::Header,
pub cloud: sensor_msgs::msg::PointCloud2,
pub polygons: Vec<pcl_msgs::msg::Vertices>,

                          }

                          impl WrappedTypesupport for PolygonMesh { 

            type CStruct = pcl_msgs__msg__PolygonMesh; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pcl_msgs__msg__PolygonMesh() }
            }

            fn create_msg() -> *mut pcl_msgs__msg__PolygonMesh {

                unsafe { pcl_msgs__msg__PolygonMesh__create() }

            }

            fn destroy_msg(msg: *mut pcl_msgs__msg__PolygonMesh) -> () {

                unsafe { pcl_msgs__msg__PolygonMesh__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> PolygonMesh {
  PolygonMesh {
header: std_msgs::msg::Header::from_native(&msg.header),
cloud: sensor_msgs::msg::PointCloud2::from_native(&msg.cloud),
// is_upper_bound_: false
// member.array_size_ : 0
polygons : {
let mut temp = Vec::with_capacity(msg.polygons.size);
let slice = unsafe { std::slice::from_raw_parts(msg.polygons.data, msg.polygons.size)};
for s in slice { temp.push(pcl_msgs::msg::Vertices::from_native(s)); }
temp },
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
self.cloud.copy_to_native(&mut msg.cloud);
unsafe { pcl_msgs__msg__Vertices__Sequence__fini(&mut msg.polygons) };
unsafe { pcl_msgs__msg__Vertices__Sequence__init(&mut msg.polygons, self.polygons.len()) };
let slice = unsafe { std::slice::from_raw_parts_mut(msg.polygons.data, msg.polygons.size)};
for (t,s) in slice.iter_mut().zip(&self.polygons) { s.copy_to_native(t);}
}



        }


                          
                          impl Default for PolygonMesh {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<PolygonMesh>::new();
                                  PolygonMesh::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Vertices {

                              pub vertices: Vec<u32>,

                          }

                          impl WrappedTypesupport for Vertices { 

            type CStruct = pcl_msgs__msg__Vertices; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pcl_msgs__msg__Vertices() }
            }

            fn create_msg() -> *mut pcl_msgs__msg__Vertices {

                unsafe { pcl_msgs__msg__Vertices__create() }

            }

            fn destroy_msg(msg: *mut pcl_msgs__msg__Vertices) -> () {

                unsafe { pcl_msgs__msg__Vertices__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Vertices {
  Vertices {
// is_upper_bound_: false
// member.array_size_ : 0
vertices: msg.vertices.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.vertices.update(&self.vertices);
}



        }


                          
                          impl Default for Vertices {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Vertices>::new();
                                  Vertices::from_native(&msg_native)
                              }
                          }
             


                      }
