cmake_minimum_required(VERSION 3.12)
project(varunastra)

set(CMAKE_CXX_STANDARD 17)
set(static_runtime ON)
set(encryption OFF)
set(exceptions ON)
set(deprecated-functions OFF)
set(i2p OFF)
set(logging OFF)
set(streaming OFF)

include(FetchContent)
FetchContent_Declare(
    hello_imgui
    GIT_REPOSITORY https://github.com/pthom/hello_imgui.git
)

FetchContent_Declare(
    libtorrent
    GIT_REPOSITORY https://github.com/arvidn/libtorrent.git
    GIT_TAG 64817e0e8793d0875fc10245de52ffb2540a223d # 2.0.8
)

FetchContent_MakeAvailable(hello_imgui)
FetchContent_MakeAvailable(libtorrent)

list(APPEND CMAKE_MODULE_PATH ${HELLOIMGUI_CMAKE_PATH})
include(hello_imgui_add_app)

include_directories(varunastra PUBLIC ${Boost_INCLUDE_DIR})
include_directories(varunastra PUBLIC ${libtorrent_SOURCE_DIR}/include)
link_libraries(torrent-rasterbar)

hello_imgui_add_app(varunastra src/varunastra.cpp src/state.cpp src/imgui_stdlib.cpp)
