cmake_minimum_required(VERSION 3.5...4.0)
project(Harbour VERSION 1.0)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED True)

file(GLOB_RECURSE HEADERS "include/*.hpp")
file(GLOB_RECURSE SOURCES "src/*.cpp")

add_executable(Harbour ${SOURCES} ${HEADERS})
target_include_directories(Harbour PRIVATE include)

include(FetchContent)

FetchContent_Declare(
  indicators
  GIT_REPOSITORY https://github.com/p-ranav/indicators.git
  GIT_TAG v2.3
)
FetchContent_Declare(
  argparse
  GIT_REPOSITORY https://github.com/p-ranav/argparse.git
)
FetchContent_Declare(
    tomlplusplus
    GIT_REPOSITORY https://github.com/marzer/tomlplusplus.git
    GIT_TAG v3.4.0
)
FetchContent_Declare(
    spdlog
    GIT_REPOSITORY https://github.com/gabime/spdlog.git
    GIT_TAG v1.15.1
)

FetchContent_MakeAvailable(spdlog)
FetchContent_MakeAvailable(tomlplusplus)
FetchContent_MakeAvailable(indicators)
FetchContent_MakeAvailable(argparse)

target_link_libraries(Harbour PRIVATE indicators::indicators)
target_link_libraries(Harbour PRIVATE argparse)
target_link_libraries(Harbour PRIVATE tomlplusplus::tomlplusplus)
target_link_libraries(Harbour PRIVATE spdlog::spdlog)
