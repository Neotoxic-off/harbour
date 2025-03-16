cmake_minimum_required(VERSION 3.14)
project(Harbour VERSION 1.0)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED True)

# Collect source files
file(GLOB_RECURSE HEADERS "include/*.hpp")
file(GLOB_RECURSE SOURCES "src/*.cpp")

# Define the executable
add_executable(Harbour ${SOURCES} ${HEADERS})

# Include headers
target_include_directories(Harbour PRIVATE include)

# Fetch indicators library
include(FetchContent)
FetchContent_Declare(
  indicators
  GIT_REPOSITORY https://github.com/p-ranav/indicators.git
  GIT_TAG        v2.3
)
FetchContent_MakeAvailable(indicators)

# Link the indicators library to your project
target_link_libraries(Harbour PRIVATE indicators::indicators)
