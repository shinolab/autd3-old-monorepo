include(FetchContent)
set(EIGEN_BUILD_DOC OFF)
set(BUILD_TESTING OFF)
set(EIGEN_BUILD_PKGCONFIG OFF)
FetchContent_Declare(
  soem
  GIT_REPOSITORY https://github.com/OpenEtherCATsociety/SOEM.git
  GIT_TAG master)
FetchContent_MakeAvailable(soem)

if(WIN32 AND AUTD3_BUILD_ARM)
  FetchContent_GetProperties(soem)
  execute_process(COMMAND git -C ${PROJECT_SOURCE_DIR} apply --directory=${soem_SOURCE_DIR} --unsafe-paths ${PROJECT_SOURCE_DIR}/src/link/soem/win-arm.patch)
endif()

set_target_properties(soem PROPERTIES FOLDER "3rdparty")
set_target_properties(soem PROPERTIES POSITION_INDEPENDENT_CODE ON)
