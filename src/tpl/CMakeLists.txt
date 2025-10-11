cmake_minimum_required(VERSION 4.0)
project({{ name }} LANGUAGES C CXX ASM)

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

{% if fast %}
set(CMAKE_C_FLAGS_RELWITHDEBINFO "-O3")
set(CMAKE_CXX_FLAGS_RELWITHDEBINFO "-O3")
{% endif %}
set(CMAKE_C_STANDARD {{ cstd }})
set(CMAKE_CXX_STANDARD {{ cppstd }})
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_VERBOSE_MAKEFILE on)
set(CMAKE_BUILD_TYPE RelWithDebInfo)
set(CMAKE_LINKER_TYPE {{ linker }})

set(CMAKE_POSITION_INDEPENDENT_CODE ON)
set(CMAKE_INTERPROCEDURAL_OPTIMIZATION TRUE)

add_compile_options(-march=native)
add_compile_options(-stdlib=libc++)

add_link_options(-lc++abi)

find_package(Threads REQUIRED)
add_executable({{ name }} main.cc)

target_link_libraries(
  ${PROJECT_NAME}
  PUBLIC Threads::Threads
)
