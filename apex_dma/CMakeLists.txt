cmake_minimum_required(VERSION 3.5)

project(apex_dma VERSION 0.1.0 LANGUAGES C CXX)

include(CTest)
enable_testing()

set(CPACK_PROJECT_NAME ${PROJECT_NAME})
set(CPACK_PROJECT_VERSION ${PROJECT_VERSION})
include(CPack)

set(CMAKE_CXX_STANDARD 17)
set(CXX_STANDARD_REQUIRED ON)

set(CMAKE_CXX_FLAGS "-Wno-multichar")
set(CMAKE_BUILD_TYPE "Debug")

include(ExternalProject)
ExternalProject_Add(
    memflow_win32_ffi
    DOWNLOAD_COMMAND ""
    CONFIGURE_COMMAND ""
    BINARY_DIR ${PROJECT_SOURCE_DIR}/memflow_lib/memflow-win32-ffi/
    BUILD_COMMAND cargo build --release
    INSTALL_COMMAND ""
    LOG_BUILD ON)
ExternalProject_Add(
    memflow_qemu_procfs
    DOWNLOAD_COMMAND ""
    CONFIGURE_COMMAND ""
    BINARY_DIR ${PROJECT_SOURCE_DIR}/memflow_lib/memflow-qemu-procfs/
    BUILD_COMMAND cargo build --release --all-features
    INSTALL_COMMAND cp ${PROJECT_SOURCE_DIR}/memflow_lib/memflow-qemu-procfs/target/release/libmemflow_qemu_procfs.so ${PROJECT_SOURCE_DIR}/build/
    LOG_BUILD ON)

find_package(Threads REQUIRED)
find_package(SDL2 REQUIRED)

add_executable(apex_dma apex_dma.cpp Game.cpp Math.cpp memory.cpp)

add_dependencies(apex_dma memflow_win32_ffi memflow_qemu_procfs)

target_link_libraries(
    apex_dma
    "${PROJECT_SOURCE_DIR}/memflow_lib/target/release/libmemflow_win32_ffi.a"
    ${SDL2_LIBRARIES}
    ${CMAKE_THREAD_LIBS_INIT}
    ${CMAKE_DL_LIBS}
    m)

target_include_directories(
    apex_dma PUBLIC
    "${PROJECT_SOURCE_DIR}/memflow_lib/memflow-win32-ffi"
    "${PROJECT_SOURCE_DIR}/memflow_lib/memflow-ffi"
    ${SDL2_INCLUDE_DIRS})