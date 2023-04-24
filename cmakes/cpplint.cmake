file(GLOB_RECURSE lint_files
  src/*.cpp
  src/*.hpp
  include/*.hpp
  capi/*.cpp
  capi/*.hpp
  examples/*.cpp
  examples/*.hpp
  tests/*.hpp
  tests/*.cpp
  bench/*.hpp
  bench/*.cpp
  dist/simulator/*.cpp
  dist/simulator/*.hpp
  dist/SOEMAUTDServer/*.cpp
  dist/SOEMAUTDServer/*.hpp
)
get_filename_component(argparse_path ${CMAKE_CURRENT_SOURCE_DIR}/dist/SOEMAUTDServer/argparse/argparse.hpp ABSOLUTE)
get_filename_component(tinycolormap_path ${CMAKE_CURRENT_SOURCE_DIR}/include/autd3/extra/tinycolormap/tinycolormap.hpp ABSOLUTE)
list(REMOVE_ITEM lint_files ${argparse_path} ${tinycolormap_path})
ADD_CUSTOM_TARGET(cpplint
  python
  ${PROJECT_SOURCE_DIR}/tools/cpplint/cpplint.py
  --filter=-runtime/references,-build/c++11,-whitespace/comments,-readability/braces,-build/include_subdir,-build/include_order,-build/include,-whitespace/parens,-readability/fn_size
  --linelength=1000
  ${lint_files}
)
