cmake_minimum_required(VERSION 3.20)
project(qi)

set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Makes Qi available.
# Also makes OpenSSL and Boost available.
include(FetchContent)
FetchContent_Declare(
  qi
  GIT_REPOSITORY https://github.com/victorpaleologue/libqi.git
  GIT_TAG f91f686a215a8106b8767fc691ee4aafc73d1ec7
)
FetchContent_MakeAvailable(qi)
