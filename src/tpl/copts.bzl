DEFAULT_COPTS = [
    "-std=c++20",
    "-Wall",
    "-Wextra",
    "-Werror",
    "-Werror=return-type",
    "-Wno-unused-parameter",
]

DEFAULT_LINKOPTS = select({
    "@platforms//os:osx": ["-dead_strip"],
    "@platforms//os:linux": [
        "-Wl,--gc-sections",
        "-Wl,--strip-all",
    ],
})
