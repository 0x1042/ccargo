cmake_minimum_required(VERSION 4.0)
project({{ name }} LANGUAGES C CXX ASM)

message(STATUS "--------------------------------------------------------")
message(STATUS "Configuration Summary")
message(STATUS "  CMAKE_HOST_SYSTEM_NAME          ${CMAKE_HOST_SYSTEM_NAME}")
message(STATUS "  CMAKE_HOST_SYSTEM_PROCESSOR     ${CMAKE_HOST_SYSTEM_PROCESSOR}")
message(STATUS "  CMAKE_BUILD_TYPE                ${CMAKE_BUILD_TYPE}")
message(STATUS "  CMAKE_C_FLAGS                   ${CMAKE_C_FLAGS}")
message(STATUS "  CMAKE_CXX_FLAGS                 ${CMAKE_CXX_FLAGS}")
message(STATUS "  CMAKE_EXE_LINKER_FLAGS          ${CMAKE_EXE_LINKER_FLAGS}")
message(STATUS "--------------------------------------------------------")

include(CheckCXXCompilerFlag)
include(CMakeDependentOption)
include(CheckLanguage)
include(GNUInstallDirs)

include(cmake/clang-tidy.cmake)
include(cmake/sanitizer.cmake)
include(cmake/dwarf.cmake)

add_compile_options("-Wall")
add_compile_options("-g")
add_compile_options("-fsized-deallocation")
add_compile_options("-Werror=delete-non-virtual-dtor")

add_compile_options(-march=native)
add_compile_options(-stdlib=libc++)

add_link_options(-lc++abi)

find_package(Threads REQUIRED)
add_executable({{ name }} main.cc)

target_link_libraries(
  ${PROJECT_NAME}
  PUBLIC Threads::Threads
)
