cmake_minimum_required(VERSION 3.22)
project(example_cpp_mod)

set(CMAKE_CXX_STANDARD 20)
set(CXX_STANDARD_REQUIRED ON)

if(CMAKE_CXX_COMPILER_ID STREQUAL "Clang")
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++20 -stdlib=libc++")
endif()

set(CMAKE_BUILD_TYPE "Release")

set(CARGO_MANIFEST ${CMAKE_SOURCE_DIR}/Cargo.toml)
set(CARGO_TARGET_DIR ${CMAKE_SOURCE_DIR}/target)

set(SKYAPI_SOURCE_FILES
        ${CMAKE_SOURCE_DIR}/src/cxx/dmalib.rs
        ${CMAKE_SOURCE_DIR}/src/cxx/host.rs)
set(SKYAPI_BRIDGE_CPP 
        ${CARGO_TARGET_DIR}/cxxbridge/ohosky_api/src/cxx/dmalib.rs.cc
        ${CARGO_TARGET_DIR}/cxxbridge/ohosky_api/src/cxx/host.rs.cc)
set(SKYAPI_LIB ${CARGO_TARGET_DIR}/release/${CMAKE_STATIC_LIBRARY_PREFIX}ohosky_api${CMAKE_STATIC_LIBRARY_SUFFIX})

# Add a custom command that builds the rust crate and generates C++ bridge code
add_custom_command(
	OUTPUT ${SKYAPI_BRIDGE_CPP} ${SKYAPI_LIB}
        COMMAND CARGO_TARGET_DIR=${CARGO_TARGET_DIR} cargo build --manifest-path ${CARGO_MANIFEST} --features=cxx,skydream --release
	DEPENDS ${SKYAPI_SOURCE_FILE}
        USES_TERMINAL
        COMMENT "Running cargo..."
)

add_library(example_cpp_mod SHARED src-cpp/main.cpp ${SKYAPI_BRIDGE_CPP})

target_include_directories(
        example_cpp_mod
        PRIVATE
        include/
        ${CARGO_TARGET_DIR}/cxxbridge/
)

target_link_libraries(example_cpp_mod ${SKYAPI_LIB})

set_target_properties(
        example_cpp_mod
        PROPERTIES ADDITIONAL_CLEAN_FILES ${CARGO_TARGET_DIR}
)

# Windows-only configuration
if(WIN32)
    target_link_libraries(example_cpp_mod userenv ws2_32 bcrypt)
    set_target_properties(
            example_cpp_mod
            PROPERTIES
            MSVC_RUNTIME_LIBRARY "MultiThreadedDLL"
            RUNTIME_OUTPUT_DIRECTORY_DEBUG ${CMAKE_BINARY_DIR}
            RUNTIME_OUTPUT_DIRECTORY_RELEASE ${CMAKE_BINARY_DIR}
    )
endif()
