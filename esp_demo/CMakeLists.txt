cmake_minimum_required(VERSION 3.8)

project(EspDemo C CXX)

include(./cmake/grpc-common.cmake)

# Proto file path
get_filename_component(proto_include_path "../apexsky/apexsky_proto/proto/" ABSOLUTE)

# Generated sources
set(esp_proto_srcs 
  "${CMAKE_CURRENT_BINARY_DIR}/com/chettoy/apexsky/apexlegends/aimbot.pb.cc"
  "${CMAKE_CURRENT_BINARY_DIR}/com/chettoy/apexsky/apexlegends/esp_data.pb.cc"
  "${CMAKE_CURRENT_BINARY_DIR}/com/chettoy/apexsky/apexlegends/player.pb.cc"
  "${CMAKE_CURRENT_BINARY_DIR}/com/chettoy/apexsky/apexlegends/spectator.pb.cc"
  "${CMAKE_CURRENT_BINARY_DIR}/com/chettoy/apexsky/esp/esp_service.pb.cc")
set(esp_proto_hdrs 
  "${CMAKE_CURRENT_BINARY_DIR}/com/chettoy/apexsky/apexlegends/aimbot.pb.h"
  "${CMAKE_CURRENT_BINARY_DIR}/com/chettoy/apexsky/apexlegends/esp_data.pb.h"
  "${CMAKE_CURRENT_BINARY_DIR}/com/chettoy/apexsky/apexlegends/player.pb.h"
  "${CMAKE_CURRENT_BINARY_DIR}/com/chettoy/apexsky/apexlegends/spectator.pb.h"
  "${CMAKE_CURRENT_BINARY_DIR}/com/chettoy/apexsky/esp/esp_service.pb.h")
set(esp_grpc_srcs 
  "${CMAKE_CURRENT_BINARY_DIR}/com/chettoy/apexsky/apexlegends/aimbot.grpc.pb.cc"
  "${CMAKE_CURRENT_BINARY_DIR}/com/chettoy/apexsky/apexlegends/esp_data.grpc.pb.cc"
  "${CMAKE_CURRENT_BINARY_DIR}/com/chettoy/apexsky/apexlegends/player.grpc.pb.cc"
  "${CMAKE_CURRENT_BINARY_DIR}/com/chettoy/apexsky/apexlegends/spectator.grpc.pb.cc"
  "${CMAKE_CURRENT_BINARY_DIR}/com/chettoy/apexsky/esp/esp_service.grpc.pb.cc")
set(esp_grpc_hdrs 
  "${CMAKE_CURRENT_BINARY_DIR}/com/chettoy/apexsky/apexlegends/aimbot.grpc.pb.h"
  "${CMAKE_CURRENT_BINARY_DIR}/com/chettoy/apexsky/apexlegends/esp_data.grpc.pb.h"
  "${CMAKE_CURRENT_BINARY_DIR}/com/chettoy/apexsky/apexlegends/player.grpc.pb.h"
  "${CMAKE_CURRENT_BINARY_DIR}/com/chettoy/apexsky/apexlegends/spectator.grpc.pb.h"
  "${CMAKE_CURRENT_BINARY_DIR}/com/chettoy/apexsky/esp/esp_service.grpc.pb.h")

add_custom_command(
  OUTPUT ${esp_proto_srcs} ${esp_proto_hdrs} ${esp_grpc_srcs} ${esp_grpc_hdrs}
  COMMAND ${_PROTOBUF_PROTOC}
  ARGS --grpc_out "${CMAKE_CURRENT_BINARY_DIR}"
    --cpp_out "${CMAKE_CURRENT_BINARY_DIR}"
    -I "${proto_include_path}"
    --plugin=protoc-gen-grpc="${_GRPC_CPP_PLUGIN_EXECUTABLE}"
    "${proto_include_path}/com/chettoy/apexsky/apexlegends/aimbot.proto"
    "${proto_include_path}/com/chettoy/apexsky/apexlegends/esp_data.proto"
    "${proto_include_path}/com/chettoy/apexsky/apexlegends/player.proto"
    "${proto_include_path}/com/chettoy/apexsky/apexlegends/spectator.proto"
    "${proto_include_path}/com/chettoy/apexsky/esp/esp_service.proto"
  DEPENDS "${proto_include_path}/com/chettoy/apexsky/apexlegends/aimbot.proto"
    "${proto_include_path}/com/chettoy/apexsky/apexlegends/esp_data.proto"
    "${proto_include_path}/com/chettoy/apexsky/apexlegends/player.proto"
    "${proto_include_path}/com/chettoy/apexsky/apexlegends/spectator.proto"
    "${proto_include_path}/com/chettoy/apexsky/esp/esp_service.proto"
)

# Include generated *.pb.h files
include_directories("${CMAKE_CURRENT_BINARY_DIR}")

# esp_grpc_proto
add_library(esp_grpc_proto
  ${esp_grpc_srcs}
  ${esp_grpc_hdrs}
  ${esp_proto_srcs}
  ${esp_proto_hdrs})

target_link_libraries(esp_grpc_proto
  ${_REFLECTION}
  ${_GRPC_GRPCPP}
  ${_PROTOBUF_LIBPROTOBUF})

# Targets esp_client
add_executable(esp_demo "esp_client.cpp")
target_link_libraries(esp_demo
  esp_grpc_proto
  ${_REFLECTION}
  ${_GRPC_GRPCPP}
  ${_PROTOBUF_LIBPROTOBUF})
