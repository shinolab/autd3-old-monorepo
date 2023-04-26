include(FetchContent)
FetchContent_Declare(
  ads
  GIT_REPOSITORY https://github.com/Beckhoff/ADS.git
  GIT_TAG v20)
FetchContent_MakeAvailable(ads)

if(WIN32)
  target_compile_definitions(ads PRIVATE NOMINMAX)
  target_compile_definitions(AdsLibTest.bin PRIVATE NOMINMAX)
endif()

set_target_properties(ads PROPERTIES FOLDER "3rdparty")
set_target_properties(example.bin PROPERTIES FOLDER "3rdparty")
set_target_properties(AdsLibTest.bin PROPERTIES FOLDER "3rdparty")
