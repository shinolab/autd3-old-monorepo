set(BOOST_INCLUDE_LIBRARIES algorithm align array asio assert bind concept_check config container core date_time detail exception format function interprocess intrusive iterator move mp11 mpl numeric/conversion optional parameter predef preprocessor range regex smart_ptr static_assert system throw_exception type_traits utility)
set(BOOST_REQUIRED_SUBMODULES libs/algorithm libs/align libs/array libs/asio libs/assert libs/bind libs/concept_check libs/config libs/container libs/core libs/date_time libs/detail libs/exception libs/format libs/function libs/interprocess libs/intrusive libs/iterator libs/move libs/mp11 libs/mpl libs/numeric/conversion libs/optional libs/parameter libs/predef libs/preprocessor libs/range libs/regex libs/smart_ptr libs/static_assert libs/system libs/throw_exception libs/type_traits libs/utility)
if(WIN32)
  list(APPEND BOOST_INCLUDE_LIBRARIES winapi)
  list(APPEND BOOST_REQUIRED_SUBMODULES libs/winapi)
endif()
set(BOOST_ALL_NO_LIB ON)
include(FetchContent)
message(STATUS "Downloading Boost Library Sources. This will take some minutes...")
set(FETCHCONTENT_QUIET FALSE)
FetchContent_Declare(
  Boost
  GIT_REPOSITORY https://github.com/boostorg/boost.git
  GIT_SHALLOW ON
  GIT_TAG boost-1.82.0
  GIT_SUBMODULES ${BOOST_REQUIRED_SUBMODULES}
  GIT_PROGRESS TRUE
  CONFIGURE_COMMAND ""
)
FetchContent_Populate(Boost)
set(Boost_INCLUDE_DIRS "")
foreach(boost_depends_lib ${BOOST_INCLUDE_LIBRARIES})
  list(APPEND Boost_INCLUDE_DIRS ${boost_SOURCE_DIR}/libs/${boost_depends_lib}/include)
endforeach()
if(WIN32)
  list(APPEND Boost_INCLUDE_DIRS ${boost_SOURCE_DIR}/libs/winapi/include)
endif()
set(FETCHCONTENT_QUIET TRUE)
