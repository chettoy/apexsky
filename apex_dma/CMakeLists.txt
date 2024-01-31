cmake_minimum_required(VERSION 3.5)

project(apex_dma VERSION 0.1.0 LANGUAGES C CXX)

include(CTest)
enable_testing()

set(CPACK_PROJECT_NAME ${PROJECT_NAME})
set(CPACK_PROJECT_VERSION ${PROJECT_VERSION})
include(CPack)

set(CMAKE_CXX_STANDARD 20)
set(CXX_STANDARD_REQUIRED ON)

if(CMAKE_CXX_COMPILER_ID STREQUAL "Clang")
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++20")
    set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -static-libstdc++")
endif()

if(NOT WIN32)
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -g -Wall -Wformat")
endif()

set(CMAKE_BUILD_TYPE "Release")

# libs
set(DEP_LIBS_DIR ${PROJECT_SOURCE_DIR}/lib)
include(ExternalProject)

# Define library extensions based on platform
if (WIN32)
    set(LIB_PREFIX "")
    set(SHARED_LIB_EXT ".dll")
    set(STATIC_LIB_EXT ".lib")
else()
    set(LIB_PREFIX "lib")
    set(SHARED_LIB_EXT ".so")
    set(STATIC_LIB_EXT ".a")
endif()

ExternalProject_Add(
    memflow_ffi
    DOWNLOAD_COMMAND ""
    CONFIGURE_COMMAND ""
    BINARY_DIR ${DEP_LIBS_DIR}/memflow/
    BUILD_COMMAND cargo build --target aarch64-linux-android --release
    INSTALL_COMMAND ""
    BUILD_BYPRODUCTS ${DEP_LIBS_DIR}/memflow/target/aarch64-linux-android/release/${LIB_PREFIX}memflow_ffi${STATIC_LIB_EXT}
    LOG_BUILD ON)

# ExternalProject_Add(
#     memflow_pcileech
#     DOWNLOAD_COMMAND ""
#     CONFIGURE_COMMAND ""
#     BINARY_DIR ${DEP_LIBS_DIR}/memflow-pcileech/
#     BUILD_COMMAND cargo build --target aarch64-linux-android --release --all-features
#     INSTALL_COMMAND cp ${DEP_LIBS_DIR}/memflow-pcileech/target/release/${LIB_PREFIX}memflow_pcileech${SHARED_LIB_EXT} ${CMAKE_BINARY_DIR}/
#     LOG_BUILD ON)

# ExternalProject_Add(
#     memflow_win32
#     DOWNLOAD_COMMAND ""
#     CONFIGURE_COMMAND ""
#     BINARY_DIR ${DEP_LIBS_DIR}/memflow-win32/
#     BUILD_COMMAND cargo build --target aarch64-linux-android --release --all-features
#     INSTALL_COMMAND cp ${DEP_LIBS_DIR}/memflow-win32/target/release/${LIB_PREFIX}memflow_win32${SHARED_LIB_EXT} ${CMAKE_BINARY_DIR}/
#     LOG_BUILD ON)

find_package(Threads REQUIRED)

# apex_dma
add_executable(apex_dma
    apex_dma.cpp apex_sky.cpp Game.cpp Math.cpp memory.cpp)

ExternalProject_Add(
    apexsky
    DOWNLOAD_COMMAND ""
    CONFIGURE_COMMAND ""
    BINARY_DIR ${PROJECT_SOURCE_DIR}/apexsky/
    BUILD_COMMAND cargo build --target aarch64-linux-android --release
    INSTALL_COMMAND ""
    BUILD_BYPRODUCTS ${PROJECT_SOURCE_DIR}/apexsky/target/aarch64-linux-android/release/${LIB_PREFIX}apexsky${STATIC_LIB_EXT}
    LOG_BUILD ON)

add_dependencies(apex_dma 
    apexsky
    memflow_ffi
    #memflow_pcileech
    #memflow_win32
    )

    
if (WIN32)
    target_link_libraries(
        apex_dma
        ${PROJECT_SOURCE_DIR}/apexsky/target/release/${LIB_PREFIX}apexsky${STATIC_LIB_EXT}
        ${DEP_LIBS_DIR}/memflow/target/release/${LIB_PREFIX}memflow_ffi${STATIC_LIB_EXT}
        zstd
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
    target_link_libraries(
        apex_dma
        ${PROJECT_SOURCE_DIR}/apexsky/target/aarch64-linux-android/release/${LIB_PREFIX}apexsky${STATIC_LIB_EXT}
        ${DEP_LIBS_DIR}/memflow/target/aarch64-linux-android/release/${LIB_PREFIX}memflow_ffi${STATIC_LIB_EXT}
        ${CMAKE_THREAD_LIBS_INIT}
        ${CMAKE_DL_LIBS}
        m)
endif()

target_include_directories(
    apex_dma PUBLIC
    ${DEP_LIBS_DIR}/memflow/memflow-ffi)
