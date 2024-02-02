cmake_minimum_required(VERSION 3.5)

project(apex_dma VERSION 0.1.0 LANGUAGES C CXX)

include(CTest)
enable_testing()

set(CPACK_PROJECT_NAME ${PROJECT_NAME})
set(CPACK_PROJECT_VERSION ${PROJECT_VERSION})
include(CPack)

set(CMAKE_CXX_STANDARD 20)
set(CXX_STANDARD_REQUIRED ON)

set(CMAKE_CXX_FLAGS "-g -Wall -Wformat -march=native")
set(CMAKE_BUILD_TYPE "Release")

# libs
set(DEP_LIBS_DIR ${PROJECT_SOURCE_DIR}/lib)
include(ExternalProject)
ExternalProject_Add(
    memflow_ffi
    DOWNLOAD_COMMAND ""
    CONFIGURE_COMMAND ""
    BINARY_DIR ${DEP_LIBS_DIR}/memflow/
    BUILD_COMMAND cargo build --release
    INSTALL_COMMAND ""
    BUILD_BYPRODUCTS ${DEP_LIBS_DIR}/memflow/target/release/libmemflow_ffi.a
    LOG_BUILD ON)
ExternalProject_Add(
    memflow_kvm
    DOWNLOAD_COMMAND ""
    CONFIGURE_COMMAND ""
    BINARY_DIR ${DEP_LIBS_DIR}/memflow-kvm/
    BUILD_COMMAND cargo build --release --all-features
    INSTALL_COMMAND cp ${DEP_LIBS_DIR}/memflow-kvm/target/release/libmemflow_kvm.so ${CMAKE_BINARY_DIR}/
    LOG_BUILD ON)
ExternalProject_Add(
    memflow_qemu
    DOWNLOAD_COMMAND ""
    CONFIGURE_COMMAND ""
    BINARY_DIR ${DEP_LIBS_DIR}/memflow-qemu/
    BUILD_COMMAND cargo build --release --all-features
    INSTALL_COMMAND cp ${DEP_LIBS_DIR}/memflow-qemu/target/release/libmemflow_qemu.so ${CMAKE_BINARY_DIR}/
    LOG_BUILD ON)
ExternalProject_Add(
    memflow_win32
    DOWNLOAD_COMMAND ""
    CONFIGURE_COMMAND ""
    BINARY_DIR ${DEP_LIBS_DIR}/memflow-win32/
    BUILD_COMMAND cargo build --release --all-features
    INSTALL_COMMAND cp ${DEP_LIBS_DIR}/memflow-win32/target/release/libmemflow_win32.so ${CMAKE_BINARY_DIR}/
    LOG_BUILD ON)
find_package(Threads REQUIRED)

# GLFW
set(GLFW_BUILD_WAYLAND ON)
set(GLFW_BUILD_X11 OFF)
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


# apex_dma
add_executable(apex_dma
    apex_dma.cpp apex_sky.cpp Game.cpp Math.cpp memory.cpp
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

ExternalProject_Add(
    apexsky
    DOWNLOAD_COMMAND ""
    CONFIGURE_COMMAND ""
    BINARY_DIR ${PROJECT_SOURCE_DIR}/apexsky/
    BUILD_COMMAND cargo build --release
    INSTALL_COMMAND ""
    BUILD_BYPRODUCTS ${PROJECT_SOURCE_DIR}/apexsky/target/release/libapexsky.a
    LOG_BUILD ON)

add_dependencies(apex_dma 
    apexsky
    memflow_ffi
    memflow_kvm
    memflow_qemu
    memflow_win32)

target_link_libraries(
    apex_dma
    ${PROJECT_SOURCE_DIR}/apexsky/target/release/libapexsky.a
    ${DEP_LIBS_DIR}/memflow/target/release/libmemflow_ffi.a
    glfw
    GL
    zstd
    ${CMAKE_THREAD_LIBS_INIT}
    ${CMAKE_DL_LIBS}
    m)
target_include_directories(
    apex_dma PUBLIC
    ${DEP_LIBS_DIR}/memflow/memflow-ffi)
