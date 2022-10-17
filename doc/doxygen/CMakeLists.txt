cmake_minimum_required(VERSION 3.21)

set(doxyfile_in ${CMAKE_CURRENT_SOURCE_DIR}/Doxyfile.in)
set(doxyfile ${CMAKE_CURRENT_BINARY_DIR}/Doxyfile)

configure_file(${doxyfile_in} ${doxyfile} @ONLY)

find_program(DOT_PATH dot PATH_SUFFIXES graphviz2.38/bin graphviz/bin)

add_custom_target(doc
  COMMAND ${DOXYGEN_EXECUTABLE} ${doxyfile}
  WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}
  COMMENT "Generating API documentation with Doxygen"
  VERBATIM
)
