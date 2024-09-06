CC=/usr/bin/clang
CXX=/usr/bin/clang++
Generates=Ninja
VCPKG_PATH=${VCPKG_ROOT}/scripts/buildsystems/vcpkg.cmake
OUTPUT=target
TARGET={{ name }}

PROFILING=--profiling-output ${OUTPUT}/profiling.json --profiling-format=google-trace

config:
	cmake -B ${OUTPUT} -G Ninja -S . -DCMAKE_TOOLCHAIN_FILE=${VCPKG_PATH} ${PROFILING}

build: config
	cmake --build ${OUTPUT}

clean:
	rm -fr ${OUTPUT}

run: build
	./${OUTPUT}/${TARGET}
