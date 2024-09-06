cmake_minimum_required(VERSION 3.26)
project({{ name }} LANGUAGES C CXX ASM)

add_compile_options("-Wall -g")
add_compile_options("-fsized-deallocation")
add_compile_options("-Werror=delete-non-virtual-dtor")

set(CMAKE_C_STANDARD {{ cstd }})
set(CMAKE_CXX_STANDARD {{ cppstd }})
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_VERBOSE_MAKEFILE on)
set(CMAKE_BUILD_TYPE RelWithDebInfo)

{% if asan %}
option(ENABLE_SANITIZE "enable address sanitize" OFF)
if(ENABLE_SANITIZE)
  add_compile_options("-fno-omit-frame-pointer")
  add_compile_options("-fno-optimize-sibling-calls")
  add_compile_options("-fsanitize=address")
  add_compile_options("-fsanitize-address-use-after-scope")

  add_link_options("-fno-omit-frame-pointer")
  add_link_options("-fno-optimize-sibling-calls")
  add_link_options("-fsanitize=address")
endif()
{% endif %}

{% if tsan %}
option(ENABLE_SANITIZE_THREAD "enable thread sanitize" OFF)
if(ENABLE_SANITIZE_THREAD)
  add_compile_options("-fno-omit-frame-pointer")
  add_compile_options("-fno-optimize-sibling-calls")
  add_compile_options("-fsanitize=thread")
  add_compile_options("-fsanitize-address-use-after-scope")

  add_link_options("-fno-omit-frame-pointer")
  add_link_options("-fno-optimize-sibling-calls")
  add_link_options("-fsanitize=thread")
endif()
{% endif %}

{% if memory %}
option(ENABLE_SANITIZE_THREAD "enable memory sanitize" OFF)
if(ENABLE_SANITIZE_THREAD)
  add_compile_options("-fno-omit-frame-pointer")
  add_compile_options("-fno-optimize-sibling-calls")
  add_compile_options("-fsanitize=memory")
  add_compile_options("-fsanitize-memory-track-origins")

  add_link_options("-fno-omit-frame-pointer")
  add_link_options("-fno-optimize-sibling-calls")
  add_link_options("-fsanitize=memory")
  add_link_options("-fsanitize-memory-track-origins")
endif()
{% endif %}

{% if undefined %}
option(ENABLE_SANITIZE_THREAD "enable thread sanitize" OFF)
if(ENABLE_SANITIZE_THREAD)
  add_compile_options("-fno-omit-frame-pointer")
  add_compile_options("-fsanitize=undefined")

  add_link_options("-fno-omit-frame-pointer")
  add_link_options("-fsanitize=undefined")
endif()
{% endif %}

{% if tidy %}
option(ENABLE_TIDY "enable clang-tidy" OFF)
if(ENABLE_TIDY)
  set(CMAKE_CXX_CLANG_TIDY clang-tidy; -format-style='file';
                           -header-filter=${CMAKE_CURRENT_SOURCE_DIR};)
endif()
{% endif %}

{% if dwarf %}
message(STATUS "Enabling split-dwarf build" OFF)
add_compile_options("-gsplit-dwarf=single")
add_link_options("-gsplit-dwarf")
{% endif %}

set(CMAKE_POSITION_INDEPENDENT_CODE ON)
set(CMAKE_INTERPROCEDURAL_OPTIMIZATION TRUE)

find_package(Threads REQUIRED)

add_executable({{ name }} main.cpp)

target_link_libraries(
  ${PROJECT_NAME}
  PRIVATE Threads::Threads
)