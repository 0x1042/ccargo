message(STATUS "Enabling split-dwarf build:" OFF)

add_compile_options("-gsplit-dwarf=single")
add_link_options("-gsplit-dwarf")
