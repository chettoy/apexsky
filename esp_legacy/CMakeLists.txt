cmake_minimum_required(VERSION 3.8)

project(EspLegacyClient C CXX)

include(./cmake/grpc-common.cmake)

set(CMAKE_CXX_STANDARD 20)
set(CXX_STANDARD_REQUIRED ON)

if(CMAKE_CXX_COMPILER_ID STREQUAL "Clang")
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++20 -stdlib=libc++")
endif()

if(NOT WIN32)
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -g -Wall -Wformat -march=native")
endif()

set(CMAKE_BUILD_TYPE "Release")

find_package(Threads REQUIRED)

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

# GLFW
set(GLFW_BUILD_WAYLAND ON)
set(GLFW_BUILD_X11 ON)
set(GLFW_DIR ${PROJECT_SOURCE_DIR}/Client/glfw)
option(GLFW_BUILD_EXAMPLES "Build the GLFW example programs" OFF)
option(GLFW_BUILD_TESTS "Build the GLFW test programs" OFF)
option(GLFW_BUILD_DOCS "Build the GLFW documentation" OFF)
option(GLFW_INSTALL "Generate installation target" OFF)
option(GLFW_DOCUMENT_INTERNALS "Include internals in documentation" OFF)
add_subdirectory(${GLFW_DIR} glfw-prefix EXCLUDE_FROM_ALL)
include_directories(${GLFW_DIR}/include)

# Dear ImGui
set(IMGUI_DIR ${PROJECT_SOURCE_DIR}/Client/imgui)
include_directories(${IMGUI_DIR} ${IMGUI_DIR}/backends)

# Targets esp_client
add_executable(esp_client
  esp_client.cpp
  ${PROJECT_SOURCE_DIR}/Client/main.cpp
  ${PROJECT_SOURCE_DIR}/Client/overlay.cpp
  ${PROJECT_SOURCE_DIR}/Client/font.cpp
  ${IMGUI_DIR}/imgui.cpp
  ${IMGUI_DIR}/imgui_demo.cpp
  ${IMGUI_DIR}/imgui_draw.cpp
  ${IMGUI_DIR}/imgui_tables.cpp
  ${IMGUI_DIR}/imgui_widgets.cpp
  ${IMGUI_DIR}/backends/imgui_impl_glfw.cpp
  ${IMGUI_DIR}/backends/imgui_impl_opengl3.cpp)

if (WIN32)
  target_link_libraries(esp_client
    esp_grpc_proto
    ${_REFLECTION}
    ${_GRPC_GRPCPP}
    ${_PROTOBUF_LIBPROTOBUF}
    glfw
    opengl32
    ${CMAKE_THREAD_LIBS_INIT}
    ${CMAKE_DL_LIBS}
    bcrypt
    gdi32
    imm32
    ntdll
    Crypt32
    Iphlpapi
    UserEnv
    Ws2_32)
else()
  target_link_libraries(esp_client
    esp_grpc_proto
    ${_REFLECTION}
    ${_GRPC_GRPCPP}
    ${_PROTOBUF_LIBPROTOBUF}
    glfw
    GL
    ${CMAKE_THREAD_LIBS_INIT}
    ${CMAKE_DL_LIBS}
    m)
endif()
