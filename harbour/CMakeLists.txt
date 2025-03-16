cmake_minimum_required(VERSION 4.0)
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
  GIT_TAG        v2.3
)
FetchContent_MakeAvailable(indicators)

target_link_libraries(Harbour PRIVATE indicators::indicators)
