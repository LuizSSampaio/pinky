cmake_minimum_required(VERSION 3.25)

project(ast LANGUAGES CXX)
set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

add_library(ast)
target_include_directories(ast PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/include)
