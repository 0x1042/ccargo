option(ENABLE_TIDY "enable clang-tidy" OFF)

if(ENABLE_TIDY)
  find_program(CLANG_TIDY_EXECUTABLE NAMES clang-tidy)
  mark_as_advanced(CLANG_TIDY_EXECUTABLE)
  if(${CLANG_TIDY_EXECUTABLE})
    message(FATAL_ERROR "Clang-tidy not found")
  else()
    message(STATUS "Enabling clang-tidy")
    set(CMAKE_CXX_CLANG_TIDY
        "${CLANG_TIDY_EXECUTABLE};-format-style='file';-header-filter=${CMAKE_CURRENT_SOURCE_DIR}"
    )
  endif()
endif()
