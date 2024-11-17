Generates=Ninja
VCPKG_PATH=${VCPKG_ROOT}/scripts/buildsystems/vcpkg.cmake
OUTPUT=target
TARGET={{ name }}

config:
	cmake -B ${OUTPUT} -G Ninja -S . -DCMAKE_TOOLCHAIN_FILE=${VCPKG_PATH}

build: config
	cmake --build ${OUTPUT}

clean:
	rm -fr ${OUTPUT}

run: build
	./${OUTPUT}/${TARGET}
