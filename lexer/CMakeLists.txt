cmake_minimum_required(VERSION 3.25)

project(lexer LANGUAGES CXX)
set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

if(NOT TARGET ast)
  add_subdirectory(${CMAKE_CURRENT_SOURCE_DIR}/../ast
                   ${CMAKE_CURRENT_BINARY_DIR}/ast)
endif()

add_library(lexer src/Lexer.cpp)
target_include_directories(lexer PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/include)
target_link_libraries(lexer PUBLIC ast)

add_executable(lexer_exe src/main.cpp)
target_link_libraries(lexer_exe PRIVATE lexer)
