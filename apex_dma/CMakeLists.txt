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
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++20 -stdlib=libc++")
endif()

if(NOT WIN32)
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -g -Wall -Wformat -march=native")
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
    BUILD_COMMAND cargo build --release
    INSTALL_COMMAND ""
    BUILD_BYPRODUCTS ${DEP_LIBS_DIR}/memflow/target/release/${LIB_PREFIX}memflow_ffi${STATIC_LIB_EXT}
    LOG_BUILD ON)
include_directories(${DEP_LIBS_DIR}/memflow/memflow-ffi)

find_package(Threads REQUIRED)

# GLFW
set(GLFW_BUILD_WAYLAND ON)
set(GLFW_BUILD_X11 ON)
set(GLFW_DIR ${PROJECT_SOURCE_DIR}/../esp_legacy/Client/glfw)
option(GLFW_BUILD_EXAMPLES "Build the GLFW example programs" OFF)
option(GLFW_BUILD_TESTS "Build the GLFW test programs" OFF)
option(GLFW_BUILD_DOCS "Build the GLFW documentation" OFF)
option(GLFW_INSTALL "Generate installation target" OFF)
option(GLFW_DOCUMENT_INTERNALS "Include internals in documentation" OFF)
add_subdirectory(${GLFW_DIR} glfw-prefix EXCLUDE_FROM_ALL)
include_directories(${GLFW_DIR}/include)

# Dear ImGui
set(IMGUI_DIR ${PROJECT_SOURCE_DIR}/../esp_legacy/Client/imgui)
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
    BINARY_DIR ${PROJECT_SOURCE_DIR}/../apexsky/apexsky/
    BUILD_COMMAND cargo build --release
    INSTALL_COMMAND ""
    BUILD_BYPRODUCTS ${PROJECT_SOURCE_DIR}/../apexsky/target/release/${LIB_PREFIX}apexsky${STATIC_LIB_EXT}
    LOG_BUILD ON)

add_dependencies(apex_dma 
    apexsky
    memflow_ffi)


if (WIN32)
    target_link_libraries(
        apex_dma
        ${PROJECT_SOURCE_DIR}/../apexsky/target/release/${LIB_PREFIX}apexsky${STATIC_LIB_EXT}
        ${DEP_LIBS_DIR}/memflow/target/release/${LIB_PREFIX}memflow_ffi${STATIC_LIB_EXT}
        glfw
        opengl32
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
        ${PROJECT_SOURCE_DIR}/../apexsky/target/release/${LIB_PREFIX}apexsky${STATIC_LIB_EXT}
        ${DEP_LIBS_DIR}/memflow/target/release/${LIB_PREFIX}memflow_ffi${STATIC_LIB_EXT}
        glfw
        GL
        zstd
        ${CMAKE_THREAD_LIBS_INIT}
        ${CMAKE_DL_LIBS}
        m)
endif()

target_include_directories(
    apex_dma PUBLIC
    ${DEP_LIBS_DIR}/memflow/memflow-ffi)
